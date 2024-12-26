#![allow(dead_code)]
mod handler;
mod idempotency_key_cleaner;
mod idempotency_key_storage;
mod kv_storage;
mod server;

use kv_server::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().filter_or("RUST_LOG", "info"));
    run().await
}
