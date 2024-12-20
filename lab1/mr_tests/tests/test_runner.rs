use std::time::Duration;
use tokio::task;
use tokio::time::interval;

fn get_ext() -> &'static str {
    if cfg!(windows) {
        return "dll";
    }
    "so"
}
async fn run_worker() {
    let args = mr_worker::args::Args {
        plugin: format!("./mr_wc.{}", get_ext()).into(),
    };
    mr_worker::run_with_args(args).await.unwrap();
}

#[tokio::test]
async fn wc_test() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().filter_or("RUST_LOG", "info"));

    let mut interval = interval(Duration::from_secs(1));
    interval.tick().await;

    //start master
    let mut tasks = vec![];
    tasks.push(task::spawn(async {
        let args = mr_master::args::Args {
            path_to_files: "./../data".into(),
            reduce_task_num: 10,
        };
        mr_master::run_with_args(args).await.unwrap();
    }));

    interval.tick().await;

    //start multiple workers
    tasks.push(task::spawn(run_worker()));
    tasks.push(task::spawn(run_worker()));
    tasks.push(task::spawn(run_worker()));

    futures::future::join_all(tasks).await;

    Ok(())
}
