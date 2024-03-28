use chrono::{DateTime, Utc};
use sqlx::{Execute, MySqlPool, PgPool};

use crate::model::{dto::TaskType, entity::AggregatedTask};

pub struct AggregationRepo {
    pub mysql_pool: MySqlPool,
    pub pg_pool: PgPool,
}


impl AggregationRepo {
    pub async fn aggregate_tasks(
        &self,
        page: usize,
        page_size: usize,
        task_types: Option<String>,
        region_id: Option<u32>,
        account_id: Option<u32>,
        division_id: Option<u32>,
        date_from: Option<DateTime<Utc>>,
        date_to: Option<DateTime<Utc>>,
        object_ids: Option<String>,
    ) -> sqlx::Result<(usize, Vec<AggregatedTask>)> {
        let mut builder =
            sqlx::QueryBuilder::new("SELECT * FROM aggregated_tasks");
        let mut count_builder =
            sqlx::QueryBuilder::new("SELECT count(*) FROM aggregated_tasks");
        let mut curr_delim = " WHERE ";
        if let Some(types) = task_types {
            let types: Vec<TaskType> = types
                .split(",")
                .map(|e| e.try_into())
                .filter(Result::is_ok)
                .map(|e| e.unwrap())
                .collect();
            builder.push(curr_delim);
            builder.push("task_type IN (");
            count_builder.push(curr_delim);
            count_builder.push("task_type IN (");
            let mut types = types.into_iter();
            if let Some(t) = types.next() {
                builder.push_bind(t.to_string());
                count_builder.push_bind(t.to_string());
            }
            for t in types {
                builder.push(", ");
                builder.push_bind(t.to_string());
                count_builder.push(", ");
                count_builder.push_bind(t.to_string());
            }
            builder.push(")");
            count_builder.push(")");
            curr_delim = " AND ";
        }
        if let Some(region_id) = region_id {
            builder.push(curr_delim);
            builder.push("region_id = ");
            builder.push_bind(region_id);
            count_builder.push(curr_delim);
            count_builder.push("region_id = ");
            count_builder.push_bind(region_id);
            curr_delim = " AND ";
        }
        if let Some(account_id) = account_id {
            builder.push(curr_delim);
            builder.push("account_id = ");
            builder.push_bind(account_id);
            count_builder.push(curr_delim);
            count_builder.push("account_id = ");
            count_builder.push_bind(account_id);
            curr_delim = " AND ";
        }
        if let Some(division_id) = division_id {
            todo!()
        }
        if let Some(date_from) = date_from {
            builder.push(curr_delim);
            builder.push("created >= ");
            builder.push_bind(date_from);
            count_builder.push(curr_delim);
            count_builder.push("created >= ");
            count_builder.push_bind(date_from);
            curr_delim = " AND ";
        }
        if let Some(date_to) = date_to {
            builder.push(curr_delim);
            builder.push("created <= ");
            builder.push_bind(date_to);
            count_builder.push(curr_delim);
            count_builder.push("created <= ");
            count_builder.push_bind(date_to);
            curr_delim = " AND ";
        }
        if let Some(ids) = object_ids {
            let ids: Vec<u32> =
                ids.split(',').map(|s| s.parse::<u32>().unwrap()).collect();
            builder.push(curr_delim);
            builder.push("object_id IN (");
            count_builder.push(curr_delim);
            count_builder.push("object_id IN (");
            let mut ids = ids.into_iter();
            if let Some(id) = ids.next() {
                builder.push_bind(id);
                count_builder.push_bind(id);
            }
            for id in ids {
                builder.push(", ");
                builder.push_bind(id);
                count_builder.push(", ");
                count_builder.push_bind(id);
            }
            builder.push(")");
            count_builder.push(")");
        }

        builder.push(" LIMIT ");
        builder.push_bind(page_size as u64);
        builder.push(" OFFSET ");
        builder.push_bind((page * page_size) as u64);

        let query = builder.build_query_as::<AggregatedTask>();
        let count_query = count_builder.build_query_scalar();

        log::debug!("Executing query:\n{}", count_query.sql());
        let count: i64 = count_query.fetch_one(&self.mysql_pool).await?;

        log::debug!("Executing query:\n{}", query.sql());
        let rows = query.fetch_all(&self.mysql_pool).await?;

        let total_pages = count / page_size as i64;
        Ok((total_pages as usize, rows))
    }
}
