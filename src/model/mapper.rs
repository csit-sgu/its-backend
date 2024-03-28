use std::collections::HashMap;

use super::{dto::{Location, ServiceObject, Task, Transition}, entity::FlatTask};

pub trait MapperLike {
    type FromType;
    type ToType;

    fn convert(value: impl IntoIterator<Item = Self::FromType>) -> impl IntoIterator<Item = Self::ToType>;
}

pub struct TasksMapper;

impl MapperLike for TasksMapper {
    type FromType = FlatTask;
    type ToType = Task;

    fn convert(value: impl IntoIterator<Item = Self::FromType>) -> impl Iterator<Item = Self::ToType> {
        let mut m: HashMap<u32, Task> = HashMap::new();
        for t in value.into_iter() {
            if !m.contains_key(&t.task_id) {
                m.insert(t.task_id, Task {
                    task_id: t.task_id,
                    task_type: t.task_type.as_str().try_into().unwrap(),
                    object: ServiceObject {
                        object_id: t.object_id,
                        object_place_id: t.object_place_id,
                        location: Location {
                            lat: t.place_lat,
                            lon: t.place_lon,
                        },
                        region_id: t.region_id,
                        region_title: t.region_title,
                    },
                    deadline: t.task_deadline,
                    transitions: vec![],
                });
            }

            if let Some(task) = m.get_mut(&t.task_id) {
                task.transitions.push(Transition {
                    status: t.task_transition_title,
                    timestamp: t.task_transitioned_at,
                });
            }
        }
        m.into_iter().map(|(_, v)| v)
    }
} 

