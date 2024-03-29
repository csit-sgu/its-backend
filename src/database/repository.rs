use chrono::{DateTime, Utc};
use sqlx::{Execute, MySqlPool};

use crate::database::core::{MySqlRepository, Repository};
use crate::model::entity::TaskStageTransition;
use crate::model::{
    entity::FlatTask,
    entity::FlatDetailedTask,
    entity::TransitionView,
    dto::TaskType,
};

pub struct AggregationRepo {
    pub mysql_pool: MySqlPool,
}

pub struct TransitionRepo {
    pub mysql_pool: MySqlPool,
}

impl Repository for TransitionRepo {
    fn get_conn_pool(&self) -> MySqlPool {
        self.mysql_pool.clone()
    }
}

impl MySqlRepository<TransitionView> for TransitionRepo {}

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
        task_id: Option<u32>,
    ) -> sqlx::Result<(usize, Vec<FlatTask>)> {
        let mut id_builder = sqlx::QueryBuilder::new(
            "SELECT DISTINCT task_id FROM aggregated_tasks",
        );
        let mut count_builder = sqlx::QueryBuilder::new(
            "SELECT count(DISTINCT task_id) FROM aggregated_tasks",
        );

        let mut curr_delim = " WHERE ";
        if let Some(types) = task_types {
            let types: Vec<TaskType> = types
                .split(",")
                .map(|e| e.try_into())
                .filter(Result::is_ok)
                .map(|e| e.unwrap())
                .collect();
            id_builder.push(curr_delim);
            id_builder.push("task_type IN (");
            count_builder.push(curr_delim);
            count_builder.push("task_type IN (");
            let mut types = types.into_iter();
            if let Some(t) = types.next() {
                id_builder.push_bind(t.to_string());
                count_builder.push_bind(t.to_string());
            }
            for t in types {
                id_builder.push(", ");
                id_builder.push_bind(t.to_string());
                count_builder.push(", ");
                count_builder.push_bind(t.to_string());
            }
            id_builder.push(")");
            count_builder.push(")");
            curr_delim = " AND ";
        }
        if let Some(region_id) = region_id {
            id_builder.push(curr_delim);
            id_builder.push("region_id = ");
            id_builder.push_bind(region_id);
            count_builder.push(curr_delim);
            count_builder.push("region_id = ");
            count_builder.push_bind(region_id);
            curr_delim = " AND ";
        }
        if let Some(account_id) = account_id {
            id_builder.push(curr_delim);
            id_builder.push("account_id = ");
            id_builder.push_bind(account_id);
            count_builder.push(curr_delim);
            count_builder.push("account_id = ");
            count_builder.push_bind(account_id);
            curr_delim = " AND ";
        }
        if let Some(_division_id) = division_id {
            todo!()
        }
        if let Some(date_from) = date_from {
            id_builder.push(curr_delim);
            id_builder.push("created >= ");
            id_builder.push_bind(date_from);
            count_builder.push(curr_delim);
            count_builder.push("created >= ");
            count_builder.push_bind(date_from);
            curr_delim = " AND ";
        }
        if let Some(date_to) = date_to {
            id_builder.push(curr_delim);
            id_builder.push("created <= ");
            id_builder.push_bind(date_to);
            count_builder.push(curr_delim);
            count_builder.push("created <= ");
            count_builder.push_bind(date_to);
            curr_delim = " AND ";
        }
        if let Some(task_id) = task_id {
            id_builder.push(curr_delim);
            id_builder.push("task_id = ");
            id_builder.push_bind(task_id);
            count_builder.push(curr_delim);
            count_builder.push("task_id = ");
            count_builder.push_bind(task_id);
            curr_delim = " AND ";
        }
        if let Some(ids) = object_ids {
            let ids: Vec<u32> =
                ids.split(',').map(|s| s.parse::<u32>().unwrap()).collect();
            id_builder.push(curr_delim);
            id_builder.push("object_id in (");
            count_builder.push(curr_delim);
            count_builder.push("object_id in (");
            let mut ids = ids.into_iter();
            if let Some(id) = ids.next() {
                id_builder.push_bind(id);
                count_builder.push_bind(id);
            }
            for id in ids {
                id_builder.push(", ");
                id_builder.push_bind(id);
                count_builder.push(", ");
                count_builder.push_bind(id);
            }
            id_builder.push(")");
            count_builder.push(")");
        }

        id_builder.push(" LIMIT ");
        id_builder.push_bind(page_size as u64);
        id_builder.push(" OFFSET ");
        id_builder.push_bind((page * page_size) as u64);

        let id_query = id_builder.build_query_scalar();
        log::debug!("Executing query:\n{}", id_query.sql());
        let ids: Vec<u32> = id_query.fetch_all(&self.mysql_pool).await?;

        let mut builder = sqlx::QueryBuilder::new(
            "SELECT * FROM aggregated_tasks WHERE task_id IN (",
        );
        let mut ids = ids.into_iter();
        if let Some(id) = ids.next() {
            builder.push_bind(id);
        }
        for id in ids {
            builder.push(", ");
            builder.push_bind(id);
        }
        builder.push(")");

        let query = builder.build_query_as::<FlatTask>();
        log::debug!("Executing query:\n{}", query.sql());
        let rows = query.fetch_all(&self.mysql_pool).await?;

        let count_query = count_builder.build_query_scalar();
        log::debug!("Executing query:\n{}", count_query.sql());
        let count: i64 = count_query.fetch_one(&self.mysql_pool).await?;
        let total_pages = count / page_size as i64;
        Ok((total_pages as usize, rows))
    }

    pub async fn detailed_task(
        &self,
        id: u32,
    ) -> sqlx::Result<Option<FlatDetailedTask>> {
        sqlx::query_as::<_, FlatDetailedTask>(
            "SELECT * FROM detailed_tasks WHERE task_id = ?"
        )
            .bind(id)
            .fetch_optional(&self.mysql_pool)
            .await
    }

    pub async fn traverse(
        &self,
        start_id: u32,
    ) -> sqlx::Result<Vec<u32>> {
        let query_text = r#"
            SELECT *
            FROM task_stage_transitions AS tst
            WHERE tst.current_stage_id = ? 
        "#;

        let mut res = vec![start_id];
        let mut curr_id = start_id;
        loop {
            let record = sqlx::query_as::<_, TaskStageTransition>(query_text)
                .bind(curr_id)
                .fetch_optional(&self.mysql_pool)
                .await?;
            match record {
                Some(tr) => {
                    log::debug!("{:?}", tr);
                    curr_id = tr.next_stage_id;
                    res.push(tr.next_stage_id);
                },
                None => break,
            }
        }
        Ok(res)
    }

    pub async fn get_stage_names(
        &self,
        stage_ids: Vec<u32>
    ) -> sqlx::Result<Vec<String>> {
        let mut titles = vec![];
        for stage_id in stage_ids.into_iter() {
            let query = sqlx::query_scalar("SELECT title FROM task_stages WHERE id = ?")
                .bind(stage_id);
            let stage_title: String = query.fetch_one(&self.mysql_pool).await?;
            titles.push(stage_title);
        }
        Ok(titles)
    }
}
