use std::sync::Arc;

use chrono::{DateTime, Utc};
use poem::error::NotFound;
use poem::{error::InternalServerError, Result};
use poem_openapi::param::Path;
use poem_openapi::{param::Query, payload::Json, OpenApi};

use crate::model::dto::{DetailedTask, Task, Transition};
use crate::model::mapper::{BatchMapperLike, DetailedTaskMapper, MapperLike, TasksMapper};
use crate::util::EmptyError;
use crate::{api::ApiTag, model::dto::AggregatedTasksResp, util::Context};

pub struct TasksRoute {
    pub ctx: Arc<Context>,
}

#[OpenApi]
impl TasksRoute {
    #[oai(path = "/tasks", method = "get", tag = ApiTag::Tasks)]
    pub async fn get_many(
        &self,
        page: Query<usize>,
        size: Query<usize>,
        task_types: Query<Option<String>>,
        region_id: Query<Option<u32>>,
        account_id: Query<Option<u32>>,
        division_id: Query<Option<u32>>,
        date_from: Query<Option<DateTime<Utc>>>,
        date_to: Query<Option<DateTime<Utc>>>,
        object_ids: Query<Option<String>>,
    ) -> Result<Json<AggregatedTasksResp>> {
        let res = self
            .ctx
            .aggregation_repo
            .aggregate_tasks(
                page.0,
                size.0,
                task_types.0,
                region_id.0,
                account_id.0,
                division_id.0,
                date_from.0,
                date_to.0,
                object_ids.0,
                None,
            )
            .await
            .map_err(|e| {
                log::error!("{}", &e);
                InternalServerError(e)
            })?;

        Ok(Json(AggregatedTasksResp {
            total_pages: res.0,
            data: TasksMapper::convert_many(res.1).collect(),
        }))
    }

    #[oai(path = "/tasks/:id", method = "get", tag = ApiTag::Tasks)]
    pub async fn get_one(
        &self,
        id: Path<u32>,
    ) -> Result<Json<DetailedTask>> {
        let res = self
            .ctx
            .aggregation_repo
            .detailed_task(id.0)
            .await
            .map_err(|e| {
                log::error!("{}", &e);
                InternalServerError(e)
            })?;

        match DetailedTaskMapper::convert(res) {
            Some(data) => Ok(Json(data)),
            None => Err(NotFound(EmptyError))
        }
    }

    #[oai(path = "/tasks/:id/stages/traverse", method = "get", tag = ApiTag::Stages)]
    pub async fn traverse(
        &self,
        id: Path<u32>,
    ) -> Result<Json<Vec<String>>> {
        let (_, task) = self
            .ctx
            .aggregation_repo
            .aggregate_tasks(
                0,
                100,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(id.0),
            )
            .await
            .map_err(|e| {
                log::error!("{}", &e);
                InternalServerError(e)
            })?;

        if task.len() < 1 {
            return Err(NotFound(EmptyError))
        }
        
        let task: Task = TasksMapper::convert_many(task).next().unwrap();
        let st_transition = task.transitions
            .iter()
            .find(|tr| tr.stage_info.is_start)
            .ok_or(InternalServerError(EmptyError))?;
        let start_id = st_transition.stage_info.id;
        let stage_ids = self.ctx
            .aggregation_repo
            .traverse(start_id)
            .await
            .map_err(InternalServerError)?;
        log::info!("Got here #1");
        let stage_titles = self.ctx
            .aggregation_repo
            .get_stage_names(stage_ids)
            .await
            .map_err(InternalServerError)?;
        log::info!("Got here #2");
        Ok(Json(stage_titles))
    }
}
