use crate::server::Server;
use kv_common::KvServer;
use tarpc::context::Context;

impl KvServer for Server {
    async fn put(self, _: Context, key: String, value: String) -> String {
        self.storage.put(&key, &value)
    }

    async fn get(self, _: Context, key: String) -> Option<String> {
        self.storage.get(&key)
    }

    async fn append(self, _: Context, key: String, value: String) -> String {
        self.storage.append(&key, &value)
    }
}
