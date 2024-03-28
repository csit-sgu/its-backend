use sqlx::{MySqlPool, PgPool};

use crate::model::entity::AggregatedTask;

pub struct AggregationRepo {
    pub mysql_pool: MySqlPool,
    pub pg_pool: PgPool,
}

impl AggregationRepo {
    pub async fn aggregate_tasks(&self, page: usize, page_size: usize) -> sqlx::Result<(usize, Vec<AggregatedTask>)> {
        let query = sqlx::query_as::<_, AggregatedTask>(
            "SELECT * FROM aggregated_tasks LIMIT ? OFFSET ?"
        )
            .bind(page_size as u64)
            .bind((page * page_size) as u64);

        let count_query: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM aggregated_tasks")
            .fetch_one(&self.mysql_pool)
            .await?;

        let rows = query.fetch_all(&self.mysql_pool).await?;
        let total_pages = count_query / page_size as i64;
        Ok((total_pages as usize, rows))
    }
}
