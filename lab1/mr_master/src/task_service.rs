use mr_common::{Task, TaskState};
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct TaskService {
    pub(crate) tasks: Mutex<Vec<Task>>,
}

impl TaskService {
    pub fn new() -> Self {
        TaskService {
            tasks: Mutex::new(Vec::new()),
        }
    }

    pub async fn add_task(&self, task: Task) {
        let mut tasks = self.tasks.lock().await;
        tasks.push(task);
    }

    pub async fn update_task_state(&self, id: Uuid, state: TaskState) {
        let mut tasks = self.tasks.lock().await;
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.state = state;
        }
    }

    pub async fn get_tasks_by_filter<P>(&self,  mut predicate: P) -> Vec<Task>
    where
        P: FnMut(&Task) -> bool,
    {
        let tasks = self.tasks.lock().await;
        tasks
            .iter()
            .filter(|task| predicate(task))
            .cloned()
            .collect()
    }

    pub async fn get_first_task_by_state_and_exchange_state(
        &self,
        state: TaskState,
        new_state: TaskState,
    ) -> Option<Task> {
        let mut tasks = self.tasks.lock().await;
        tasks
            .iter_mut()
            .find(|task| task.state == state)
            .map(|task| {
                task.state = new_state;
                task.clone()
            })
    }

    pub async fn all_tasks<P>(&self, mut predicate: P) -> bool
    where
        P: FnMut(&Task) -> bool,
    {
        let tasks = self.tasks.lock().await;
        tasks.iter().all(|task| predicate(task))
    }
}
