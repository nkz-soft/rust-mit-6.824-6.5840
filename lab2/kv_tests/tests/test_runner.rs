
#[cfg(test)]
mod tests {
    use serial_test::serial;
    use futures::future;
    use kv_clerk::Clerk;
    use std::time::Duration;
    use tokio::sync::OnceCell;
    use tokio::task;
    use tokio::task::JoinHandle;
    use tokio::time::interval;
    use uuid::Uuid;
    use log::info;

    static ONCE: OnceCell<JoinHandle<()>> = OnceCell::const_new();

    async fn start_kv_server() -> JoinHandle<()> {
        env_logger::init_from_env(env_logger::Env::default().filter_or("RUST_LOG", "info"));
        task::spawn(async {
            kv_server::run().await.unwrap();
        })
    }

    #[macro_export]
    macro_rules! run_kv_server {
        () => {{
            let mut interval = interval(Duration::from_secs(1));
            interval.tick().await;
            //start kv_server
            let _ = ONCE.get_or_init(start_kv_server).await;

            interval.tick().await;
        }};
    }

    #[tokio::test]
    #[ignore]
    async fn one_client_test() -> anyhow::Result<()> {

        run_kv_server!();

        let clerk = Clerk::new().await?;
        let value = clerk.put("one_client_key1", "value1").await?;
        assert_eq!(value, "value1");
        let value = clerk.put("one_client_key2", "value2").await?;
        assert_eq!(value, "value2");
        let value = clerk.put("one_client_key3", "value3").await?;
        assert_eq!(value, "value3");

        let value = clerk.get("one_client_key1").await?;
        assert_eq!(value.unwrap(), "value1");
        let value = clerk.get("one_client_key2").await?;
        assert_eq!(value.unwrap(), "value2");
        let value = clerk.get("one_client_key3").await?;
        assert_eq!(value.unwrap(), "value3");

        let value = clerk.append("one_client_key1", "value4").await?;
        assert_eq!(value, "value1value4");
        let value = clerk.append("one_client_key2", "value5").await?;
        assert_eq!(value, "value2value5");
        let value = clerk.append("one_client_key3", "value6").await?;
        assert_eq!(value, "value3value6");

        Ok(())
    }

    #[tokio::test]
    async fn many_clients_test() {

        run_kv_server!();

        // I need to run this test 5 times asynchronously but the assert_eq! does not work
        // https://stackoverflow.com/questions/53068563/how-do-you-write-test-assertions-inside-of-tokiorun-futures
        let mut tasks = vec![];
        for i in 0..15 {
            tasks.push(task::spawn(async move {
                let clerk = Clerk::new().await.unwrap();
                let value = clerk
                    .put(format!("many_clients_key{}", i).as_str(), format!("value{}", i).as_str())
                    .await
                    .unwrap();
                assert_eq!(value, format!("value{}", i));

                let value = clerk
                    .get(format!("many_clients_key{}", i).as_str())
                    .await
                    .unwrap()
                    .unwrap();
                assert_eq!(value, format!("value{}", i));

                let value = clerk
                    .append(format!("many_clients_key{}", i).as_str(), format!("value{}", i).as_str())
                    .await
                    .unwrap();
                assert_eq!(value, format!("value{}value{}", i, i));

                let value = clerk
                    .get(format!("many_clients_key{}", i).as_str())
                    .await
                    .unwrap()
                    .unwrap();
                assert_eq!(value, format!("value{}value{}", i, i));
            }));
        }

        future::join_all(tasks).await;
    }

    #[tokio::test]
    async fn idempotency_key_one_client_test() -> anyhow::Result<()> {

        run_kv_server!();

        let first_idempotency_key = Some(Uuid::new_v4());

        let clerk = Clerk::new().await?;
        let value = clerk
            .put_with_idempotency("idempotency_key_one_client_key1", "value1", first_idempotency_key)
            .await?;
        assert_eq!(value, "value1");

        let value = clerk.get("idempotency_key_one_client_key1").await?;
        assert_eq!(value.unwrap(), "value1");

        let value = clerk
            .put_with_idempotency("idempotency_key_one_client_key1", "value1_2", first_idempotency_key)
            .await?;
        assert_eq!(value, "value1");

        let value = clerk
            .append_with_idempotency("idempotency_key_one_client_key1", "value1_3", first_idempotency_key)
            .await?;
        assert_eq!(value, "value1");

        Ok(())
    }
}