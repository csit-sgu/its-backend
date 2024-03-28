use std::sync::Arc;

use poem::{error::InternalServerError, Result};
use poem_openapi::{param::Query, payload::Json, OpenApi};

use crate::{api::ApiTag, model::dto::AggregatedTasksResp, util::Context};

pub struct TasksRoute {
    pub ctx: Arc<Context>,
}

#[OpenApi]
impl TasksRoute {
    #[oai(path = "/tasks", method = "get", tag = ApiTag::Tasks)]
    pub async fn get_one(&self, page: Query<usize>, size: Query<usize>) -> Result<Json<AggregatedTasksResp>> {
        let res =
            self.ctx
                .aggregation_repo
                .aggregate_tasks(page.0, size.0)
                .await
                .map_err(|e| {
                    log::error!("{}", &e);
                    InternalServerError(e)
                })?;
        Ok(Json(AggregatedTasksResp { total_pages: res.0, data: res.1 }))
    }
}
