use std::sync::Arc;

use poem::{error::InternalServerError, error::NotFound, Result};
use poem_openapi::{param::Path, param::Query, payload::Json, OpenApi};
use uuid::Uuid;

use crate::{
    api::ApiTag,
    database::BasicRepositoryExt,
    model::{dto::BookUpdate, entity::Book},
    util::{Context, EmptyError},
};

pub struct BookRoute {
    pub ctx: Arc<Context>,
}

#[OpenApi]
impl BookRoute {
    #[oai(path = "/book/:id", method = "get", tag = ApiTag::Book)]
    pub async fn get_one(&self, id: Path<Uuid>) -> Result<Json<Book>> {
        let obj = self
            .ctx
            .book_repo
            .fetch_one("id", id.0)
            .await
            .map_err(InternalServerError)?;

        match obj {
            Some(obj) => Ok(Json(obj)),
            None => Err(NotFound(EmptyError)),
        }
    }

    #[oai(path = "/book", method = "get", tag = ApiTag::Book)]
    pub async fn get_many(
        &self,
        page: Query<i64>,
        size: Query<i64>,
    ) -> Result<Json<Vec<Book>>> {
        let objs = self
            .ctx
            .book_repo
            .fetch_many_paginated(page.0, size.0)
            .await
            .map_err(InternalServerError)?;

        Ok(Json(objs))
    }

    #[oai(path = "/book/:id", method = "put", tag = ApiTag::Book)]
    pub async fn update(
        &self,
        id: Path<Uuid>,
        upd: Json<BookUpdate>,
    ) -> Result<()> {
        self.ctx
            .book_repo
            .update_with_cond(serde_json::to_value(upd.0).unwrap(), "id", id.0)
            .await
            .map_err(InternalServerError)?;

        Ok(())
    }
}
