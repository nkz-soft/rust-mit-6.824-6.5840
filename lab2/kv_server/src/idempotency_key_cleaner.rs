use crate::server::Services;
use log::info;
use std::sync::Arc;
use std::time::Duration;
use tokio::task;
use tokio::task::JoinHandle;
use tokio::time::interval;

pub struct IdempotencyKeyCleaner {}

impl IdempotencyKeyCleaner {
    pub async fn start(services: Arc<Services>) -> JoinHandle<()> {
        info!("Idempotency key cleaner started");
        task::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                info!("Cleaning idempotency keys");
                services.idempotency_key_storage().clear();
            }
        })
    }
}
