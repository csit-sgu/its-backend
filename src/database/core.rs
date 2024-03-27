use futures::TryStreamExt;
use serde::Serialize;
use sqlx::{
    postgres::{PgPool, PgRow},
    Execute, Postgres,
};
use std::fmt::Display;

use crate::model::entity::Entity;

pub trait Repository {
    fn get_conn_pool(&self) -> PgPool;
}

pub trait BasicRepositoryExt<E>: Repository
where
    E: for<'r> sqlx::FromRow<'r, PgRow> + Entity + Serialize + Send + Unpin,
{
    /// Fetch one row from the database using condition: key-value pair.
    async fn fetch_one<'q, K, V>(
        &self,
        key: K,
        value: V,
    ) -> sqlx::Result<Option<E>>
    where
        K: Display,
        V: 'q + Send + sqlx::Encode<'q, Postgres> + sqlx::Type<Postgres>,
    {
        log::debug!("Using `BasicRepostioryExt::fetch_one` implementation");
        let pool = self.get_conn_pool();

        let mut builder = sqlx::QueryBuilder::new("SELECT * FROM ");
        builder.push(E::table_name());

        builder.push(" WHERE ");
        builder.push(key);
        builder.push(" = ");
        builder.push_bind(value);
        builder.push(" LIMIT 1");

        let query = builder.build_query_as::<E>();
        log::debug!("Query built: {}. Executing", query.sql());

        let mut rows = query.fetch(&pool);
        rows.try_next().await
    }

    /// Fetch all rows from the table unconditionally.
    /// If you would like to add some sort of condition,
    /// use `fetch_many_with_cond()`.
    async fn fetch_many<'q>(&self) -> sqlx::Result<Vec<E>> {
        log::debug!("Using `BasicRepostioryExt::fetch_many` implementation");
        let pool = self.get_conn_pool();

        let mut builder = sqlx::QueryBuilder::new("SELECT * FROM ");
        builder.push(E::table_name());

        let query = builder.build_query_as::<E>();
        log::debug!("Query built: {}. Executing", query.sql());

        let rows = query.fetch(&pool);
        rows.try_collect().await
    }

    /// Fetch all rows from the database using condition: key-value pair.
    async fn fetch_many_with_cond<'q, K, V>(
        &self,
        key: K,
        value: V,
    ) -> sqlx::Result<Vec<E>>
    where
        K: Display,
        V: 'q + Send + sqlx::Encode<'q, Postgres> + sqlx::Type<Postgres>,
    {
        log::debug!(
            "Using `BasicRepostioryExt::fetch_many_with_cond` implementation"
        );
        let pool = self.get_conn_pool();

        let mut builder = sqlx::QueryBuilder::new("SELECT * FROM ");
        builder.push(E::table_name());

        builder.push(" WHERE ");
        builder.push(key);
        builder.push(" = ");
        builder.push_bind(value);

        let query = builder.build_query_as::<E>();
        log::debug!("Query built: {}. Executing", query.sql());

        let rows = query.fetch(&pool);
        rows.try_collect().await
    }

    async fn fetch_many_paginated<'q>(
        &self,
        page: i64,
        size: i64,
    ) -> sqlx::Result<Vec<E>> {
        log::debug!(
            "Using `BasicRepostioryExt::fetch_many_paginated` implementation"
        );
        let pool = self.get_conn_pool();

        let mut builder = sqlx::QueryBuilder::new("SELECT * FROM ");
        builder.push(E::table_name());

        builder.push(" OFFSET ");
        builder.push_bind(page * size);
        builder.push(" LIMIT ");
        builder.push_bind(size);

        let query = builder.build_query_as::<E>();
        log::debug!("Query built: {}. Executing", query.sql());

        let rows = query.fetch(&pool);
        rows.try_collect().await
    }

    /// Fetch rows from the database, using appended condition `cond`
    /// as a `WHERE` clause. For obvious reasons, this operation is unsafe
    /// and can introduce vulnerabilities. It is implemented here, though,
    /// to allow precise data fetching, when `fetch_one()` and `fetch_many()`
    /// cannot satisfy your needs.
    async fn fetch_unsafe<'a>(&self, cond: &'a str) -> sqlx::Result<Vec<E>> {
        log::debug!("Using `BasicRepostioryExt::fetch_unsafe` implementation");
        let pool = self.get_conn_pool();

        let mut builder = sqlx::QueryBuilder::new("SELECT * FROM ");
        builder.push(E::table_name());
        builder.push(" WHERE ");
        builder.push(cond);

        let query = builder.build_query_as::<E>();
        log::debug!("Query built: {}. Executing", query.sql());

        let rows = query.fetch(&pool);
        rows.try_collect().await
    }

    /// Insert rows to the database. In an ORM-like manner, takes an
    /// iterable `entities` each representing a new potential row.
    async fn add(
        &self,
        entities: impl IntoIterator<Item = E>,
    ) -> sqlx::Result<()> {
        log::debug!("Using `BasicRepostioryExt::add` implementation");
        for e in entities {
            if let serde_json::Value::Object(obj) =
                serde_json::to_value(e).unwrap()
            {
                let mut builder =
                    sqlx::QueryBuilder::<Postgres>::new("INSERT INTO ");

                builder.push(E::table_name());
                builder.push(" (");

                let mut keys = obj.iter().map(|(k, _)| k);
                if let Some(key) = keys.next() {
                    builder.push(key);
                }
                for key in keys {
                    builder.push(", ");
                    builder.push(key);
                }
                builder.push(") ");

                let values = obj.into_iter().map(|(_, v)| v);
                builder.push_values(
                    vec![values].into_iter(),
                    |mut b, values| {
                        for value in values {
                            b.push_bind(value);
                        }
                    },
                );

                let query = builder.build();
                log::debug!("Query built: {}. Executing", query.sql());
                query.execute(&self.get_conn_pool()).await?;
            } else {
                log::warn!(
                    "An attempt to save something other than object to the database. Ignoring"
                );
            }
        }
        Ok(())
    }

    /// Update rows, which match condition specified in `key` and `value`, with
    /// data `update`.
    async fn update_with_cond<'q, K, V>(
        &self,
        update: serde_json::Value,
        key: K,
        value: V,
    ) -> sqlx::Result<()>
    where
        K: Display,
        V: 'q + Send + sqlx::Encode<'q, Postgres> + sqlx::Type<Postgres>,
    {
        log::debug!("Using `BasicRepostioryExt::update` implementation");
        let mut builder = sqlx::QueryBuilder::<Postgres>::new("UPDATE ");
        let pool = self.get_conn_pool();
        builder.push(E::table_name());

        if let serde_json::Value::Object(obj) = update {
            let mut update_it = obj.into_iter();
            if let Some((k, v)) = update_it.next() {
                builder.push(" SET ");
                builder.push(k);
                builder.push(" = ");
                builder.push_bind(v);
            }
            for (k, v) in update_it {
                builder.push(", ");
                builder.push(k);
                builder.push(" = ");
                builder.push_bind(v);
            }

            builder.push(" WHERE ");
            builder.push(key);
            builder.push(" = ");
            builder.push_bind(value);

            let query = builder.build();
            log::debug!("Query built: {}. Executing", query.sql());
            query.execute(&pool).await?;
        } else {
            log::warn!(
                "An attempt to update table rows with something other than object. Ignoring"
            );
        }
        Ok(())
    }

    /// Update rows, which match condition specified in `key` and `value`,
    /// in an unsafe manner.
    /// Condition `cond` is basically appended as `WHERE` clause to the query.
    /// For obvious reasons, this operation is unsafe
    /// and can introduce vulnerabilities. It is implemented here, though,
    /// to allow precise data updating, when `update()` cannot satisfy your needs.
    async fn update_unsafe<'a, 'q, K, V>(
        &self,
        update: serde_json::Value,
        cond: &'a str,
    ) -> sqlx::Result<()>
    where
        K: Display,
        V: 'q + Send + sqlx::Encode<'q, Postgres> + sqlx::Type<Postgres>,
    {
        log::debug!("Using `BasicRepostioryExt::update_unsafe` implementation");
        let mut builder = sqlx::QueryBuilder::<Postgres>::new("UPDATE ");
        let pool = self.get_conn_pool();
        builder.push(E::table_name());

        if let serde_json::Value::Object(obj) = update {
            let mut update_it = obj.into_iter();
            if let Some((k, v)) = update_it.next() {
                builder.push(" SET ");
                builder.push(k);
                builder.push(" = ");
                builder.push_bind(v);
            }
            for (k, v) in update_it {
                builder.push(", ");
                builder.push(k);
                builder.push(" = ");
                builder.push_bind(v);
            }

            builder.push(" WHERE ");
            builder.push(cond);

            let query = builder.build();
            log::debug!("Query built: {}. Executing", query.sql());
            query.execute(&pool).await?;
        } else {
            log::warn!(
                "An attempt to update table rows with something other than object. Ignoring"
            );
        }

        Ok(())
    }
}
