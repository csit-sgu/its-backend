use crate::model::dto::{ExtractedFeatures, Task, TaskType};

use chrono::{DateTime, TimeDelta, Utc};
use std::collections::{BTreeMap, HashMap, HashSet};

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
            .get_mut(&task.object.object_id)
            .unwrap()
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
                (val, task_group.iter().collect::<Vec<_>>())
            })
            .map(|(_, group)| {
                log::debug!("{}", group.len());
                let summary = group
                    .iter()
                    .zip(group.iter().skip(1))
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

                        log::debug!("{:?}", last_fulfilled);
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
        let groups = group_tasks_by_object(tasks);
        let mut sum = 0i64;
        for (_, group) in groups {
            sum += group.iter().fold(0i64, |mut sum, task| {
                let end_transition = task
                    .transitions
                    .iter()
                    .find(|x| x.stage_info.is_closed == true);

                sum += match end_transition {
                    Some(transition) => get_time_penalty(
                        transition.transitioned_at,
                        task.deadline_at,
                    ),
                    None => 0i64,
                };
                sum
            });
        }
        sum
    }

    fn extract_remission_rate(tasks: &Vec<Task>) -> i64 {
        0
    }

    fn extract_fallback_rate(tasks: &Vec<Task>) -> i64 {
        log::debug!("Extracting fallback rate");
        let mut metric = 0;
        for task in tasks {
            let stage_ids = task.transitions
                .clone()
                .into_iter()
                .map(|tr| tr.stage_info.id);
            let mut m: HashMap<i64, i64> = HashMap::new();
            for id in stage_ids {
                if !m.contains_key(&(id as i64)) {
                    m.insert(id as i64, 0);
                }
                *m.get_mut(&(id as i64)).unwrap() += 1;
            }
            let mut acc = 0;
            for (_, cnt) in m.iter() {
                acc += cnt;
            }
            acc -= m.len() as i64;
            acc *= acc;
            metric += acc;
        }
        metric / tasks.len() as i64
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
