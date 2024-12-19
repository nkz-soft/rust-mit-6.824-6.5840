use tokio::time::Instant;
use uuid::Uuid;

#[derive(Clone)]
pub struct Worker {
    pub id: Uuid,
    pub last_heartbeat: Instant,
    pub running_task_id: Option<Uuid>,
}

impl Worker {
    pub fn new(id: Uuid) -> Self {
        Worker {
            id,
            last_heartbeat: Instant::now(),
            running_task_id: None,
        }
    }
}
