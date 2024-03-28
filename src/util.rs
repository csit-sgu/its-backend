use derive_more::{Display, Error};

use crate::database::AggregationRepo;

pub struct Context {
    pub aggregation_repo: AggregationRepo,
}

#[derive(Debug, Error, Display)]
pub struct EmptyError;
