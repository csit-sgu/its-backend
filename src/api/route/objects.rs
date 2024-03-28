use std::sync::Arc;
use poem_openapi::{OpenApi};

use crate::{util::Context};

pub struct ObjectsRoute {
    pub ctx: Arc<Context>,
}

#[OpenApi]
impl ObjectsRoute {
}
