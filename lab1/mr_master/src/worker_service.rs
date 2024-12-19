use crate::server::Services;
use crate::worker::Worker;
use log::info;
use mr_common::TaskState;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Instant;
use uuid::Uuid;

pub struct WorkerService {
    pub(crate) workers: Mutex<Vec<Worker>>,
    services: Arc<Services>,
}

impl WorkerService {
    pub fn new(services: Arc<Services>) -> Self {
        WorkerService {
            workers: Mutex::new(Vec::new()),
            services,
        }
    }

    pub async fn add_worker(&self, worker: Worker) {
        let mut workers = self.workers.lock().await;
        workers.push(worker);
    }

    pub async fn add_running_task_to_worker(&self, id: Uuid, task_id: Uuid) {
        let mut workers = self.workers.lock().await;
        if let Some(worker) = workers.iter_mut().find(|worker| worker.id == id) {
            worker.running_task_id = Option::from(task_id);
        }
    }

    pub async fn set_worker_heartbeat(&self, id: Uuid) {
        let mut workers = self.workers.lock().await;
        if let Some(worker) = workers.iter_mut().find(|worker| worker.id == id) {
            worker.last_heartbeat = Instant::now();
        }
    }

    pub async fn check_workers_lifetime(&self) {
        let mut workers = self.workers.lock().await;
        info!("Checking lifetime for {} workers", workers.len());

        let expired_workers = workers
            .iter()
            .filter(|worker| worker.last_heartbeat.elapsed() > std::time::Duration::from_secs(5))
            .cloned()
            .collect::<Vec<Worker>>();

        for worker in &expired_workers {
            info!("Worker {} expired", worker.id);
            let running_task_id = worker.running_task_id;
            if let Some(running_task_id) = running_task_id {
                info!("..its running task {} set to idle ", running_task_id);
                self.services
                    .task_service()
                    .update_task_state(running_task_id, TaskState::Idle)
                    .await;
            }
        }
        workers.retain(|w| !expired_workers.iter().any(|x| x.id == w.id));
    }
}
