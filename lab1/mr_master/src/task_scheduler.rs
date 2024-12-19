use crate::server::Services;
use log::info;
use mr_common::{Task, TaskKind, TaskState};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;
use tokio::time::interval;
use tokio::{fs, task};

pub struct TaskScheduler {}

impl TaskScheduler {
    pub fn start(services: Arc<Services>, sender: Sender<u8>) -> JoinHandle<()> {
        info!("Task scheduler started");
        task::spawn(async move {
            let path_to_files = services.configuration_service().path_to_files();
            let reduce_task_num = services.configuration_service().reduce_task_num();

            let mut paths = fs::read_dir(path_to_files).await.unwrap();
            while let Ok(Some(entry)) = paths.next_entry().await {
                info!("Loading task {}", entry.file_name().to_str().unwrap());
                services
                    .task_service()
                    .add_task(Task::new(
                        TaskKind::Map,
                        entry.file_name().to_str().unwrap().to_string(),
                    ))
                    .await;
            }
            info!("Map phase started");
            let mut interval = interval(Duration::from_secs(5));
            // Wait for all map tasks to be completed
            //TODO Rewrite to use channel or condvar to avoid blocking
            while !services
                .task_service()
                .all_tasks(|task| task.state == TaskState::Completed)
                .await
            {
                interval.tick().await;
            }
            info!("Map phase finished");

            let tasks = services
                .task_service()
                .get_tasks_by_filter(|task| task.state == TaskState::Completed)
                .await;

            for task in tasks {
                services
                    .task_service()
                    .add_task(Task::with_parent(TaskKind::Reduce, task.id))
                    .await;
            }
            info!("Reduce phase started");
            // Wait for all reduce tasks to be completed
            while !services
                .task_service()
                .all_tasks(|task| task.state == TaskState::Completed)
                .await
            {
                interval.tick().await;
            }

            info!("Delete intermediate files");
            let tasks = services
                .task_service()
                .get_tasks_by_filter(|task| {
                    task.state == TaskState::Completed && task.kind == TaskKind::Reduce
                })
                .await;

            for task in tasks {
                let task_id = task.parent.unwrap();
                for reduce_task in 0..reduce_task_num {
                    fs::remove_file(format!("mr-{task_id}-{reduce_task}"))
                        .await
                        .unwrap();
                }
            }
            info!("MapReduce phase finished");
            sender.send(0).await.unwrap();
        })
    }
}
