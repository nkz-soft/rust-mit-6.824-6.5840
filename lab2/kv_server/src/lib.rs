mod handler;
mod server;
mod storage;

use crate::server::Server;
use futures::{future, StreamExt};
use kv_common::KvServer;
use log::info;
use std::future::Future;
use std::net::{IpAddr, Ipv4Addr};
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

    tokio::spawn(async move {
        listener
            // Ignore accept errors.
            .filter_map(|r| future::ready(r.ok()))
            .map(tarpc_server::BaseChannel::with_defaults)
            // Limit channels to 5 per IP.
            .max_channels_per_key(5, |t| t.transport().peer_addr().unwrap().ip())
            // serve is generated by the service attribute. It takes as input any type implementing
            // the generated Master trait.
            .map(move |channel| {
                let server = Server::default();
                channel.execute(server.serve()).for_each(spawn)
            })
            // Max 10 channels.
            .buffer_unordered(10)
            .for_each(|_| async {})
            .await;
    })
    .await?;
    Ok(())
}