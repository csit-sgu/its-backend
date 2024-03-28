use std::sync::Arc;

use chrono::{DateTime, Utc};
use poem::{error::InternalServerError, Result};
use poem_openapi::param::Query;
use poem_openapi::{payload::Json, OpenApi};

use crate::model::dto::TaskType;
use crate::{api::ApiTag, model::entity::AggregatedTask, util::Context};

pub struct TasksRoute {
    pub ctx: Arc<Context>,
}

#[OpenApi]
impl TasksRoute {
    #[oai(path = "/tasks", method = "get", tag = ApiTag::Tasks)]
    pub async fn get_one(
        &self,
        task_types: Query<Option<String>>,
        region: Query<Option<String>>,
        account_id: Query<Option<u32>>,
        department_id: Query<Option<u32>>,
        date_from: Query<Option<DateTime<Utc>>>,
        date_to: Query<Option<DateTime<Utc>>>,
        object_ids: Query<Option<String>>,
    ) -> Result<Json<Vec<AggregatedTask>>> {
        if let Some(types) = task_types.0 {
            let types: Vec<TaskType> =
                types.split(',').map(|s| s.try_into().unwrap()).collect();
            todo!();
        }
        if let Some(region) = region.0 {
            todo!()
        }
        if let Some(account_id) = account_id.0 {
            todo!()
        }
        if let Some(department_id) = department_id.0 {
            todo!()
        }
        if let Some(date_from) = date_from.0 {
            todo!()
        }
        if let Some(date_to) = date_to.0 {
            todo!()
        }
        if let Some(ids) = object_ids.0 {
            let ids: Vec<u32> =
                ids.split(',').map(|s| s.parse::<u32>().unwrap()).collect();
            todo!()
        }
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
