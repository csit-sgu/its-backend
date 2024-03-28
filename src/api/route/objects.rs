use poem_openapi::OpenApi;
use std::sync::Arc;

use crate::util::Context;

pub struct ObjectsRoute {
    pub ctx: Arc<Context>,
}

#[OpenApi]
impl ObjectsRoute {}
