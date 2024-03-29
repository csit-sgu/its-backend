use crate::model::dto::{ExtractedFeatures, Task, TaskType};

use chrono::{DateTime, TimeDelta, Utc};
use std::collections::BTreeMap;

pub struct MetricExtractor;

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

fn get_time_penalty(current: DateTime<Utc>, deadline: DateTime<Utc>) -> i64 {
    let days = (current - deadline).num_days();
    if days < 0 {
        log::debug!("Days: {}", days);
        0
    } else {
        log::debug!("Days: {}", days);
        days * days
    }
}

fn group_tasks_by_object(tasks: &Vec<Task>) -> BTreeMap<u32, Vec<Task>> {
    let mut groups: BTreeMap<u32, Vec<Task>> = BTreeMap::new();
    for task in tasks {
        if !groups.contains_key(&task.object.object_id) {
            groups.insert(task.object.object_id, vec![]);
        }
        groups
            .get_mut(&task.object.object_id).unwrap()
            .push(task.clone());
    }
    log::debug!("{}", groups.len());
    groups
}

impl MetricExtractor {
    fn extract_regularity(tasks: &Vec<Task>) -> i64 {
        log::debug!("Extracting regularity");
        let groups = group_tasks_by_object(tasks);
        groups
            .iter()
            .map(|(val, task_group)| {
                (
                    val,
                    task_group
                        .iter()
                        .filter(|x| x.task_type == TaskType::Regular),
                )
            })
            .map(|(_, group)| {
                let summary = group.clone()
                    .zip(group.clone().skip(1))
                    .map(|(last, next)| {
                        let last_fulfilled = last
                            .transitions
                            .iter()
                            .find(|x| {
                                x.stage_info.is_fulfilled
                                    && !x.stage_info.is_closed
                            })
                            .unwrap();

                        let next_fulfilled = next
                            .transitions
                            .iter()
                            .find(|x| {
                                x.stage_info.is_fulfilled
                                    && !x.stage_info.is_closed
                            })
                            .unwrap();
                        let time_info = last.object.time_info.clone();
                        get_time_penalty(
                            next_fulfilled.transitioned_at,
                            last_fulfilled.transitioned_at
                                + TimeDelta::new(
                                    (time_info.period + time_info.delta) as i64,
                                    0u32,
                                )
                                .unwrap(),
                        )
                    })
                    .sum::<i64>();
                summary
            })
            .sum()
    }

    fn extract_speed(tasks: &Vec<Task>) -> i64 {
        // let groups = group_tasks_by_object(tasks);
        // groups.iter().fold(0i64, |mut sum, (_, task)| {
        //     let end_transition = task
        //         .transitions
        //         .iter()
        //         .find(|x| x.stage_info.is_closed == true);

        //     sum += match end_transition {
        //         Some(transition) => get_time_penalty(
        //             transition.transitioned_at,
        //             task.deadline_at,
        //         ),
        //         None => 0i64,
        //     };
        //     sum
        // });
        0
    }

    fn extract_remission_rate(tasks: &Vec<Task>) -> i64 {
        0
    }

    fn extract_fallback_rate(tasks: &Vec<Task>) -> i64 {
        0
    }

    pub fn extract(&self, tasks: &Vec<Task>) -> ExtractedFeatures {
        log::debug!("Got {} tasks in total", tasks.len());
        let (regularity, speed, remission_rate, fallback_rate) = (
            Self::extract_regularity(tasks),
            Self::extract_speed(tasks),
            Self::extract_remission_rate(tasks),
            Self::extract_fallback_rate(tasks),
        );

        ExtractedFeatures::new(regularity, speed, remission_rate, fallback_rate)
    }
}
