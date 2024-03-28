use futures::TryStreamExt;
use sqlx::{MySqlPool, PgPool};

use crate::model::entity::AggregatedTask;

pub struct AggregationRepo {
    pub mysql_pool: MySqlPool,
    pub pg_pool: PgPool,
}

impl AggregationRepo {
    pub async fn aggregate_tasks(&self) -> sqlx::Result<Vec<AggregatedTask>> {
        // wrong column names
        let query = sqlx::query_as::<_, AggregatedTask>(
            r#"
            SELECT
                t.task_id, t.account_id, t.assigner_id, t.taskable_type, t.deadline_at,
                so.object_id, so.place_id, so.account_id,
                p.place_id, p.region_id, p.district_id
            FROM tasks AS t
            JOIN service_object_task sot ON sot.task_id = t.task_id
            JOIN service_objects so ON so.object_id = sot.object_id
            JOIN places p ON p.region_id = so.object_id
            "#,
        );
        let rows = query.fetch(&self.mysql_pool);
        rows.try_collect().await
    }
}
