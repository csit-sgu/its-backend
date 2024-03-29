use derive_more::{Display, Error};

use crate::database::{AggregationRepo, TransitionRepo};
use crate::processing::extractor;

pub struct Context {
    pub aggregation_repo: AggregationRepo,
    pub transition_repo: TransitionRepo,
    pub metric_extractor: extractor::MetricExtractor 
}

#[derive(Debug, Error, Display)]
pub struct EmptyError;
