use poem_openapi::{payload::PlainText, OpenApi};

use crate::api::ApiTag;

pub struct IndexRoute;

#[OpenApi]
impl IndexRoute {
    #[oai(path = "/", method = "get", tag = ApiTag::Index)]
    pub async fn index(&self) -> PlainText<String> {
        PlainText("Service is running!".to_string())
    }
}
