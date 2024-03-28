use std::sync::Arc;

use crate::database::core::MySqlRepository;
use poem::{error::InternalServerError, Result};
use poem_openapi::{param::Path, payload::Json, OpenApi};

use crate::model::entity::TransitionView;
use crate::{api::ApiTag, util::Context};

pub struct TransitionRoute {
    pub ctx: Arc<Context>,
}

#[OpenApi]
impl TransitionRoute {
    #[oai(path = "/transitions/:task_id", method = "get", tag = ApiTag::Transitions)]
    pub async fn get_one(
        &self,
        task_id: Path<u32>,
    ) -> Result<Json<Vec<TransitionView>>> {
        let result = self
            .ctx
            .transition_repo
            .fetch_many_with_cond("task_id", task_id.0)
            .await
            .map_err(InternalServerError)?;

        Ok(Json(result))
    }
}
