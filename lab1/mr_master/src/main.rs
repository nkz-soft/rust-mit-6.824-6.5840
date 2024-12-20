#![allow(dead_code)]
mod args;
mod handler;
mod server;
mod task_scheduler;
mod task_service;
mod worker;
mod worker_health_check;
mod worker_service;

use mr_common::Master;
use mr_master::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().filter_or("RUST_LOG", "info"));
    run().await
}
