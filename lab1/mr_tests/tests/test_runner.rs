use std::time::Duration;
use tokio::task;
use tokio::time::interval;

#[tokio::test]
async fn wc_test() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().filter_or("RUST_LOG", "info"));

    let mut interval = interval(Duration::from_secs(1));
    interval.tick().await;

    //start master
    let mut tasks = vec![];
    tasks.push(task::spawn(async {
        mr_master::run().await.unwrap();
    }));

    interval.tick().await;

    //start multiple workers
    tasks.push(task::spawn(async {
        mr_worker::run().await.unwrap();
    }));

    tasks.push(task::spawn(async {
        mr_worker::run().await.unwrap();
    }));

    tasks.push(task::spawn(async {
        mr_worker::run().await.unwrap();
    }));

    futures::future::join_all(tasks).await;

    Ok(())
}
