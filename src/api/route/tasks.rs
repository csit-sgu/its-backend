use std::sync::Arc;

use poem::{error::InternalServerError, Result};
use poem_openapi::{payload::Json, OpenApi};

use crate::{api::ApiTag, model::entity::AggregatedTask, util::Context};

pub struct TasksRoute {
    pub ctx: Arc<Context>,
}

#[OpenApi]
impl TasksRoute {
    #[oai(path = "/tasks", method = "get", tag = ApiTag::Tasks)]
    pub async fn get_one(&self) -> Result<Json<Vec<AggregatedTask>>> {
        let tasks =
            self.ctx
                .aggregation_repo
                .aggregate_tasks()
                .await
                .map_err(|e| {
                    log::error!("{}", &e);
                    InternalServerError(e)
                })?;
        Ok(Json(tasks))
    }
}
