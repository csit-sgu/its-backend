use derive_more::{Display, Error};

use crate::database::{AggregationRepo, TransitionRepo};
use crate::model::entity::TransitionView;

pub struct Context {
    pub aggregation_repo: AggregationRepo,
    pub transition_repo: TransitionRepo,
}

#[derive(Debug, Error, Display)]
pub struct EmptyError;
