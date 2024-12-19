pub mod plugin;

use std::option::Option;
use std::path::PathBuf;
use tarpc::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum TaskKind {
    Map,
    Reduce,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum TaskState {
    Idle,
    InProgress,
    Completed,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: Uuid,
    pub kind: TaskKind,
    pub state: TaskState,
    pub file: Option<String>,
    pub parent: Option<Uuid>,
}

impl Task {
    pub fn new(kind: TaskKind, file: String) -> Task {
        Task {
            id: Uuid::new_v4(),
            kind,
            state: TaskState::Idle,
            file: Some(file),
            parent: None,
        }
    }

    pub fn with_parent(kind: TaskKind, parent: Uuid) -> Task {
        Task {
            id: Uuid::new_v4(),
            kind,
            state: TaskState::Idle,
            parent: Some(parent),
            file: None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Configuration {
    path_to_files: PathBuf,
    reduce_task_num: u32,
}

impl Configuration {
    pub fn new(path_to_files: PathBuf, reduce_task_num: u32) -> Configuration {
        Configuration {
            path_to_files,
            reduce_task_num,
        }
    }
    pub fn path_to_files(&self) -> &PathBuf {
        &self.path_to_files
    }

    pub fn reduce_task_num(&self) -> u32 {
        self.reduce_task_num
    }
}

#[tarpc::service]
pub trait Master {
    async fn register(worker_id: Uuid) -> Configuration;
    async fn heartbeat(worker_id: Uuid);
    async fn get_task(worker_id: Uuid) -> Option<Task>;
    async fn put_task(worker_id: Uuid, task: Task);
}
