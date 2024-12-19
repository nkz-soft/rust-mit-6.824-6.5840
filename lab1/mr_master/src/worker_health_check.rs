use crate::server::Services;
use log::info;
use std::sync::Arc;
use std::time::Duration;
use tokio::task;
use tokio::task::JoinHandle;
use tokio::time::interval;

pub struct WorkerHealthCheck {}

impl WorkerHealthCheck {
    pub fn start(services: Arc<Services>) -> JoinHandle<()> {
        info!("Health check started");
        task::spawn(async move {
            let mut interval = interval(Duration::from_secs(5));
            loop {
                interval.tick().await;
                info!("Checking workers lifetime");
                services.worker_service().check_workers_lifetime().await;
            }
        })
    }
}
