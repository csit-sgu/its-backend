use std::sync::Arc;

use chrono::{DateTime, Utc};
use poem::{error::InternalServerError, Result};
use poem_openapi::{param::Query, payload::Json, OpenApi};

use crate::model::mapper::{MapperLike, TasksMapper};
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
            )
            .await
            .map_err(|e| {
                log::error!("{}", &e);
                InternalServerError(e)
            })?;

        let mut aggregated_data = AggregatedTasksResp {
            total_pages: res.0,
            data: TasksMapper::convert(res.1).collect(),
            stats: None,
        };

        aggregated_data.stats =
            Some(self.ctx.metric_extractor.extract(&aggregated_data.data));
        Ok(Json(aggregated_data))
    }

    // #[oai(path = "/tasks/:id", method = "get", tag = ApiTag::Tasks)]
    // pub async fn get_one(
    //     &self,
    //     id: Path<u32>,
    // ) -> Result<Json<DetailedTask>> {
    //     let res = self
    //         .ctx
    //         .aggregation_repo
    //         .detailed_task(id)
    //         .await
    //         .map_err(|e| {
    //             log::error!("{}", &e);
    //             InternalServerError(e)
    //         })?;
    //     Ok(Json(AggregatedTasksResp {
    //         total_pages: res.0,
    //         data: res.1,
    //     }))
    // }
}
