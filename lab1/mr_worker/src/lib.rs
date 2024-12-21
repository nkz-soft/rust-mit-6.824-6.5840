pub mod args;
mod heartbeat;
mod plugin_holder;

use crate::heartbeat::Heartbeat;
use crate::plugin_holder::PluginHolder;
use clap::Parser;
use log::info;
use mr_common::{MasterClient, TaskKind, TaskState};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use std::time::Duration;
use tarpc::tokio_serde::formats::Json;
use tarpc::{client, context};
use tokio::time::sleep;
use uuid::Uuid;

pub async fn run() -> anyhow::Result<()> {
    let args = args::Args::parse();
    run_with_args(args).await
}

pub async fn run_with_args(args: args::Args) -> anyhow::Result<()> {
    let server_addr = (IpAddr::V4(Ipv4Addr::LOCALHOST), 5555);

    let mut transport = tarpc::serde_transport::tcp::connect(server_addr, Json::default);
    transport.config_mut().max_frame_length(usize::MAX);

    let client = MasterClient::new(client::Config::default(), transport.await?).spawn();

    let worker_id = Uuid::new_v4();

    info!("Starting worker with id {}", worker_id);
    let configuration = client.register(context::current(), worker_id).await?;

    info!("with configuration {:#?}", configuration);

    Heartbeat::start(Arc::from(client.clone()), worker_id).await;

    let mut plugin_holder = PluginHolder::new();
    unsafe {
        plugin_holder.load_lib(args.plugin)?;
    }
    unsafe {
        loop {
            let task = client.get_task(context::current(), worker_id).await?;

            if task.is_none() {
                info!("No tasks for worker {}", worker_id);
                sleep(Duration::from_secs(5)).await;
                continue;
            }

            let mut task = task.unwrap();
            match task.kind {
                TaskKind::Map => plugin_holder.map(&task, &configuration)?,
                TaskKind::Reduce => plugin_holder.reduce(&task, &configuration)?,
            };
            task.state = TaskState::Completed;
            client.put_task(context::current(), worker_id, task).await?;
        }
    }
}
