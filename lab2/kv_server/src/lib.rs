mod handler;
mod idempotency_key_cleaner;
mod idempotency_key_storage;
mod kv_storage;
mod server;

use crate::idempotency_key_cleaner::IdempotencyKeyCleaner;
use crate::server::{Server, Services};
use futures::{future, StreamExt};
use kv_common::KvServer;
use log::info;
use std::future::Future;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use tarpc::server as tarpc_server;
use tarpc::server::incoming::Incoming;
use tarpc::server::Channel;
use tarpc::tokio_serde::formats::Json;

async fn spawn(fut: impl Future<Output = ()> + Send + 'static) {
    tokio::spawn(fut);
}

pub async fn run() -> anyhow::Result<()> {
    let server_addr = (IpAddr::V4(Ipv4Addr::LOCALHOST), 5555);

    let mut listener = tarpc::serde_transport::tcp::listen(&server_addr, Json::default).await?;
    listener.config_mut().max_frame_length(usize::MAX);
    info!("Listening on port {}", listener.local_addr().port());

    let services = Arc::new(Services::new());
    IdempotencyKeyCleaner::start(services.clone()).await;

    tokio::spawn(async move {
        listener
            // Ignore accept errors.
            .filter_map(|r| future::ready(r.ok()))
            .map(tarpc_server::BaseChannel::with_defaults)
            .map(move |channel| {
                let server = Server::new(services.clone());
                channel.execute(server.serve()).for_each(spawn)
            })
            // Max 25 channels.
            .buffer_unordered(100)
            .for_each(|_| async {})
            .await;
    })
    .await?;
    Ok(())
}
