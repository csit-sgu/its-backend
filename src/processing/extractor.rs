use crate::model::dto::Task;

pub struct MetricExtractor;

struct ExtractedFeatures {
    regularity_score: i64,
    speed_score: i64,
    remission_rate: i64,
    fallback_rate: i64,
}

impl ExtractedFeatures {
    pub fn new(
        regularity_score: i64,
        speed_score: i64,
        remission_rate: i64,
        fallback_rate: i64,
    ) -> ExtractedFeatures {
        Self {
            regularity_score,
            speed_score,
            remission_rate,
            fallback_rate,
        }
    }
}

impl MetricExtractor {
    fn extract_regularity(tasks: &Vec<Task>) -> i64 {
        0
    }

    fn extract_speed(tasks: &Vec<Task>) -> i64 {
        0
    }

    fn extract_remission_rate(tasks: &Vec<Task>) -> i64 {
        0
    }

    fn extract_fallback_rate(tasks: &Vec<Task>) -> i64 {
        0
    }

    pub fn extract(&self, tasks: &Vec<Task>) -> ExtractedFeatures {
        let (regularity, speed, remission_rate, fallback_rate) = (
            Self::extract_regularity(tasks),
            Self::extract_speed(tasks),
            Self::extract_remission_rate(tasks),
            Self::extract_fallback_rate(tasks),
        );

        ExtractedFeatures::new(regularity, speed, remission_rate, fallback_rate)
    }
}
