use chrono::Utc;

pub mod extractor;
pub mod utils;

pub fn get_time_penalty(current: chrono::DateTime<Utc>, deadline: chrono::DateTime<Utc>) -> i64 {
    let diff = current - deadline;
    let minutes = diff.num_seconds() / 60;
    if minutes < 0 {
        return 0; 
    } else {
        return minutes * minutes;
    }
}
