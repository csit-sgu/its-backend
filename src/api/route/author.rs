use std::sync::Arc;

use poem::{error::InternalServerError, error::NotFound, Result};
use poem_openapi::{param::Path, param::Query, payload::Json, OpenApi};
use uuid::Uuid;

use crate::{
    api::ApiTag,
    database::BasicRepositoryExt,
    model::{dto::AuthorUpdate, entity::Author},
    util::{Context, EmptyError},
};

pub struct AuthorRoute {
    pub ctx: Arc<Context>,
}

#[OpenApi]
impl AuthorRoute {
    #[oai(path = "/author/:id", method = "get", tag = ApiTag::Author)]
    pub async fn get_one(&self, id: Path<Uuid>) -> Result<Json<Author>> {
        let obj = self
            .ctx
            .author_repo
            .fetch_one("id", id.0)
            .await
            .map_err(InternalServerError)?;

        match obj {
            Some(obj) => Ok(Json(obj)),
            None => Err(NotFound(EmptyError)),
        }
    }

    #[oai(path = "/author", method = "get", tag = ApiTag::Author)]
    pub async fn get_many(
        &self,
        page: Query<i64>,
        size: Query<i64>,
    ) -> Result<Json<Vec<Author>>> {
        let objs = self
            .ctx
            .author_repo
            .fetch_many_paginated(page.0, size.0)
            .await
            .map_err(InternalServerError)?;

        Ok(Json(objs))
    }

    #[oai(path = "/author/:id", method = "put", tag = ApiTag::Author)]
    pub async fn update(
        &self,
        id: Path<Uuid>,
        upd: Json<AuthorUpdate>,
    ) -> Result<()> {
        self.ctx
            .author_repo
            .update_with_cond(serde_json::to_value(upd.0).unwrap(), "id", id.0)
            .await
            .map_err(InternalServerError)?;

        Ok(())
    }
}
