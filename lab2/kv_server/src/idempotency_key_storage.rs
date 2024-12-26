use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::Instant;
use uuid::Uuid;

#[derive(Clone)]
struct IdempotencyKey {
    #[allow(dead_code)]
    id: Uuid,
    value: String,
    created: Instant,
}

impl IdempotencyKey {
    pub fn new(id: Uuid, value: String) -> Self {
        IdempotencyKey {
            id,
            value,
            created: Instant::now(),
        }
    }
}

#[derive(Clone)]
pub struct IdempotencyKeyStorage {
    store: Arc<Mutex<HashMap<Uuid, IdempotencyKey>>>,
    clean_period_sec: u64,
}

impl IdempotencyKeyStorage {
    #[allow(dead_code)]
    fn with_clean_period_sec(mut self, clean_period_sec: u64) -> Self {
        self.clean_period_sec = clean_period_sec;
        self
    }

    pub fn get_or_map<F>(&self, uuid: Uuid, f: F) -> String
    where
        F: FnOnce() -> String,
    {
        let mut store = self.store.lock().unwrap();
        if store.contains_key(&uuid) {
            let key = store.get(&uuid).unwrap();
            return key.value.clone();
        }

        let value = f();
        store.insert(uuid, IdempotencyKey::new(uuid, value.clone()));
        value
    }

    pub fn clear(&self) {
        let mut store = self.store.lock().unwrap();
        store.retain(|_, v| {
            v.created.elapsed() < std::time::Duration::from_secs(self.clean_period_sec)
        });
    }
}

impl Default for IdempotencyKeyStorage {
    fn default() -> Self {
        IdempotencyKeyStorage {
            store: Arc::new(Mutex::new(HashMap::new())),
            clean_period_sec: 60,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_or_map_test() {
        let storage = IdempotencyKeyStorage::default();
        let uuid = Uuid::new_v4();
        let value = storage.get_or_map(uuid, || "value".to_string());
        assert_eq!(value, "value");

        // try to get the value again with the same key
        let value = storage.get_or_map(uuid, || "value".to_string());
        assert_eq!(value, "value");
    }

    #[test]
    fn clear_test_empty() {
        let storage = IdempotencyKeyStorage::default().with_clean_period_sec(0);
        let uuid = Uuid::new_v4();
        let value = storage.get_or_map(uuid, || "value".to_string());
        assert_eq!(value, "value");

        storage.clear();

        assert_eq!(storage.store.lock().unwrap().iter().count(), 0);
    }

    #[test]
    fn clear_test_not_empty() {
        let storage = IdempotencyKeyStorage::default().with_clean_period_sec(10);
        let uuid = Uuid::new_v4();
        let value = storage.get_or_map(uuid, || "value".to_string());
        assert_eq!(value, "value");

        storage.clear();

        assert_eq!(storage.store.lock().unwrap().iter().count(), 1);
    }
}
