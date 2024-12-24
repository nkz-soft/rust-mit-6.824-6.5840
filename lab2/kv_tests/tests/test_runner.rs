use kv_clerk::Clerk;
use std::time::Duration;
use tokio::task;
use tokio::time::interval;

#[tokio::test]
async fn one_client_test() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().filter_or("RUST_LOG", "info"));

    let mut interval = interval(Duration::from_secs(1));
    interval.tick().await;

    //start kv_server
    let mut tasks = vec![];
    tasks.push(task::spawn(async {
        kv_server::run().await.unwrap();
    }));

    interval.tick().await;

    let clerk = Clerk::new().await?;
    let value = clerk.put("key1", "value1").await?;
    assert_eq!(value, "value1");
    let value = clerk.put("key2", "value2").await?;
    assert_eq!(value, "value2");
    let value = clerk.put("key3", "value3").await?;
    assert_eq!(value, "value3");

    let value = clerk.get("key1").await?;
    assert_eq!(value.unwrap(), "value1");
    let value = clerk.get("key2").await?;
    assert_eq!(value.unwrap(), "value2");
    let value = clerk.get("key3").await?;
    assert_eq!(value.unwrap(), "value3");

    let value = clerk.append("key1", "value4").await?;
    assert_eq!(value, "value1value4");
    let value = clerk.append("key2", "value5").await?;
    assert_eq!(value, "value2value5");
    let value = clerk.append("key3", "value6").await?;
    assert_eq!(value, "value3value6");

    Ok(())
}
