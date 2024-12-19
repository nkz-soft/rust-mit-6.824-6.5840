use crate::args::Args;
use crate::task_service::TaskService;
use crate::worker_service::WorkerService;
use mr_common::Configuration;
use std::sync::Arc;

#[derive(Clone)]
pub struct MasterServer {
    pub services: Arc<Services>,
}

impl MasterServer {
    pub fn new(services: Arc<Services>) -> Self {
        MasterServer { services }
    }
}

#[derive(Clone)]
pub struct Services {
    worker_service: Option<Arc<WorkerService>>,
    task_service: Option<Arc<TaskService>>,
    configuration_service: Arc<Configuration>,
}

impl Services {
    pub fn new(args: Args) -> Self {
        let mut service = Self {
            worker_service: None,
            task_service: None,
            configuration_service: Arc::new(args.into()),
        };

        service.task_service = Some(Arc::new(TaskService::new()));
        service.worker_service = Some(Arc::new(WorkerService::new(Arc::from(service.clone()))));
        service
    }

    pub const fn worker_service(&self) -> &Arc<WorkerService> {
        self.worker_service.as_ref().unwrap()
    }

    pub const fn task_service(&self) -> &Arc<TaskService> {
        self.task_service.as_ref().unwrap()
    }

    pub const fn configuration_service(&self) -> &Arc<Configuration> {
        &self.configuration_service
    }
}
