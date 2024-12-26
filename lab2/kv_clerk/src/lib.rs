use anyhow::Context;
use kv_common::KvServerClient;
use std::net::{IpAddr, Ipv4Addr};
use tarpc::tokio_serde::formats::Json;
use tarpc::{client, context};
use uuid::Uuid;

pub struct Clerk {
    client: KvServerClient,
}

impl Clerk {
    pub async fn new() -> anyhow::Result<Clerk> {
        let server_addr = (IpAddr::V4(Ipv4Addr::LOCALHOST), 5555);
        let transport = tarpc::serde_transport::tcp::connect(server_addr, Json::default);
        Ok(Self {
            client: KvServerClient::new(client::Config::default(), transport.await?).spawn(),
        })
    }

    pub async fn get(&self, key: &str) -> anyhow::Result<Option<String>> {
        self.client
            .get(context::current(), key.to_owned())
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))
            .context("Failed to get key")
    }

    pub async fn put(&self, key: &str, value: &str) -> anyhow::Result<String> {
        self.put_with_idempotency(key, value, None).await
    }

    pub async fn put_with_idempotency(
        &self,
        key: &str,
        value: &str,
        idempotency_key: Option<Uuid>,
    ) -> anyhow::Result<String> {
        self.client
            .put(
                context::current(),
                key.to_owned(),
                value.to_owned(),
                idempotency_key,
            )
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))
            .context("Failed to put key")
    }
    pub async fn append(&self, key: &str, value: &str) -> anyhow::Result<String> {
        self.append_with_idempotency(key, value, None).await
    }

    pub async fn append_with_idempotency(
        &self,
        key: &str,
        value: &str,
        idempotency_key: Option<Uuid>,
    ) -> anyhow::Result<String> {
        self.client
            .append(
                context::current(),
                key.to_owned(),
                value.to_owned(),
                idempotency_key,
            )
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))
            .context("Failed to append key")
    }
}
