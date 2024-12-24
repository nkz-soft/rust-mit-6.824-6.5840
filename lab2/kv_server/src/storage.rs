use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct Storage {
    store: Arc<Mutex<HashMap<String, String>>>,
}

impl Storage {
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
