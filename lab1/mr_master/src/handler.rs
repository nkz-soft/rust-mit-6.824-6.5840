use crate::server::MasterServer;
use crate::worker::Worker;
use crate::Master;
use log::info;
use mr_common::{Configuration, Task, TaskState};
use tarpc::context::Context;
use uuid::Uuid;

impl Master for MasterServer {
    async fn register(self, _: Context, worker_id: Uuid) -> Configuration {
        info!("Worker {} registered", worker_id);
        self.services
            .worker_service()
            .add_worker(Worker::new(worker_id))
            .await;
        self.services
            .configuration_service()
            .clone()
            .as_ref()
            .clone()
    }

    async fn heartbeat(self, _: Context, worker_id: Uuid) {
        info!("Worker {} heartbeat", worker_id);
        self.services
            .worker_service()
            .set_worker_heartbeat(worker_id)
            .await;
    }

    async fn get_task(self, _: Context, worker_id: Uuid) -> Option<Task> {
        info!("Worker {} get task", worker_id);
        let task = self
            .services
            .task_service()
            .get_first_task_by_state_and_exchange_state(TaskState::Idle, TaskState::InProgress)
            .await;

        if let Some(task) = task {
            self.services
                .worker_service()
                .add_running_task_to_worker(worker_id, task.id)
                .await;
            return Some(task);
        }
        None
    }

    async fn put_task(self, _: Context, worker_id: Uuid, task: Task) {
        info!("Worker {} put task {}", worker_id, task.id);
        self.services
            .task_service()
            .update_task_state(task.id, task.state)
            .await;
    }
}
