mod handler;
mod server;
mod storage;

use kv_server::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().filter_or("RUST_LOG", "info"));
    run().await
}
