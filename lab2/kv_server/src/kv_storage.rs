use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct KvStorage {
    store: Arc<Mutex<HashMap<String, String>>>,
}

impl KvStorage {
    pub fn get(&self, key: &str) -> Option<String> {
        self.store.lock().unwrap().get(key).cloned()
    }

    pub fn put(&self, key: &str, value: &str) -> String {
        self.store
            .lock()
            .unwrap()
            .insert(key.to_owned(), value.to_owned());
        value.to_owned()
    }

    pub fn append(&self, key: &str, value: &str) -> String {
        let mut guard = self.store.lock().unwrap();

        let entry = guard.entry(key.to_owned());

        match entry {
            Occupied(mut o) => {
                o.get_mut().push_str(value);
                o.get().clone()
            }
            Vacant(v) => {
                v.insert(value.to_owned());
                value.to_owned()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_test() {
        let storage = KvStorage::default();
        let value = storage.get("get_test_key");
        assert!(value.is_none());
    }

    #[test]
    fn put_test() {
        let storage = KvStorage::default();
        let value = storage.put("put_test_key", "test_value");
        assert_eq!(value, "test_value");

        let value = storage.get("put_test_key");
        assert_eq!(value.unwrap(), "test_value");
    }

    #[test]
    fn append_test() {
        let storage = KvStorage::default();
        let value = storage.append("append_test_key", "test_value");
        assert_eq!(value, "test_value");

        let value = storage.get("append_test_key");
        assert_eq!(value.unwrap(), "test_value");
    }
}
