use std::collections::HashMap;

use super::{
    dto::{
        Account, DetailedServiceObject, DetailedTask, DetailedTransition,
        Location, ServiceObject, StageInfo, Task, TimeInfo, Transition,
    },
    entity::{FlatDetailedTask, FlatTask},
};

pub trait BatchMapperLike {
    type FromType;
    type ToType;

    fn convert_many(
        value: impl IntoIterator<Item = Self::FromType>,
    ) -> impl IntoIterator<Item = Self::ToType>;
}

pub trait MapperLike {
    type FromType;
    type ToType;

    fn convert(
        value: impl IntoIterator<Item = Self::FromType>,
    ) -> Option<Self::ToType>;
}

pub struct TasksMapper;

impl BatchMapperLike for TasksMapper {
    type FromType = FlatTask;
    type ToType = Task;

    fn convert_many(
        value: impl IntoIterator<Item = Self::FromType>,
    ) -> impl Iterator<Item = Self::ToType> {
        let mut m: HashMap<u32, Task> = HashMap::new();
        for t in value.into_iter() {
            if !m.contains_key(&t.task_id) {
                m.insert(
                    t.task_id,
                    Task {
                        task_id: t.task_id,
                        task_type: t.task_type.as_str().try_into().unwrap(),
                        object: ServiceObject {
                            object_id: t.object_id,
                            object_place_id: t.object_place_id,
                            object_type_id: t.object_type_id,
                            location: Location {
                                lat: t.place_lat,
                                lon: t.place_lon,
                            },
                            time_info: TimeInfo {
                                period: t.period,
                                delta: t.delta,
                            },
                            region_id: t.region_id,
                            region_title: t.region_title,
                        },
                        deadline_at: t.task_deadline_at,
                        transitions: vec![],
                    },
                );
            }

            if let Some(task) = m.get_mut(&t.task_id) {
                task.transitions.push(Transition {
                    status: t.task_transition_title,
                    transitioned_at: t.task_transitioned_at,
                    stage_info: StageInfo {
                        is_start: t.task_stage_is_start,
                        is_fulfilled: t.task_stage_is_fulfilled,
                        is_closed: t.task_stage_is_closed,
                        is_cancelled: t.task_stage_is_cancelled,
                    },
                });
            }
        }
        m.into_iter().map(|(_, v)| v)
    }
}

pub struct DetailedTaskMapper;

impl MapperLike for DetailedTaskMapper {
    type FromType = FlatDetailedTask;
    type ToType = DetailedTask;

    fn convert(
        value: impl IntoIterator<Item = Self::FromType>,
    ) -> Option<Self::ToType> {
        let mut res: Option<_> = None;
        for t in value.into_iter() {
            if let None = res {
                res = Some(DetailedTask {
                    task_id: t.task_id,
                    task_type: t.task_type.as_str().try_into().unwrap(),
                    object: DetailedServiceObject {
                        object_id: t.object_id,
                        object_place_id: t.object_place_id,
                        object_title: t.object_title,
                        object_subtitle: t.object_subtitle,
                        location: Location {
                            lat: t.place_lat,
                            lon: t.place_lon,
                        },
                        region_id: t.region_id,
                        region_title: t.region_title,
                        region_capital: t.region_capital,
                    },
                    deadline_at: t.task_deadline_at,
                    description: t.task_description,
                    account: Account {
                        account_id: t.account_id,
                        title: t.account_title,
                        account_type_id: t.account_type_id,
                        account_type_title: t.account_type_title,
                    },
                    transitions: vec![],
                });
            }

            let mut new_res = res.clone().unwrap();
            new_res.transitions.push(DetailedTransition {
                status: t.task_transition_title,
                transitioned_at: t.task_transitioned_at,
                stage_info: StageInfo {
                    is_start: t.task_stage_is_start,
                    is_fulfilled: t.task_stage_is_fulfilled,
                    is_closed: t.task_stage_is_closed,
                    is_cancelled: t.task_stage_is_cancelled,
                },
                transitioned_by_id: t.task_transitioned_by_id,
            });
            res = Some(new_res);
        }
        res
    }
}
