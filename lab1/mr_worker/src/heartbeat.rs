use log::info;
use mr_common::MasterClient;
use std::sync::Arc;
use std::time::Duration;
use tarpc::context;
use tokio::task;
use tokio::time::interval;
use uuid::Uuid;

pub struct Heartbeat {}

impl Heartbeat {
    pub async fn start(client: Arc<MasterClient>, uuid: Uuid) {
        info!("Heartbeat started");

        task::spawn(async move {
            let mut interval = interval(Duration::from_secs(5));
            loop {
                interval.tick().await;
                client.heartbeat(context::current(), uuid).await.unwrap();
            }
        });
    }
}
