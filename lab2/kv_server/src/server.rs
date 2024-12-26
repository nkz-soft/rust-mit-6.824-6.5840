use crate::idempotency_key_storage::IdempotencyKeyStorage;
use crate::kv_storage::KvStorage;
use std::sync::Arc;

#[derive(Clone)]
pub struct Server {
    pub services: Arc<Services>,
}

impl Server {
    pub fn new(services: Arc<Services>) -> Self {
        Self { services }
    }
}

#[derive(Clone)]
pub struct Services {
    kv_storage: Arc<KvStorage>,
    idempotency_key_storage: Arc<IdempotencyKeyStorage>,
}

impl Services {
    pub fn new() -> Self {
        Self {
            kv_storage: Arc::new(KvStorage::default()),
            idempotency_key_storage: Arc::new(IdempotencyKeyStorage::default()),
        }
    }

    pub const fn kv_storage(&self) -> &Arc<KvStorage> {
        &self.kv_storage
    }

    pub const fn idempotency_key_storage(&self) -> &Arc<IdempotencyKeyStorage> {
        &self.idempotency_key_storage
    }
}
