use crate::server::Server;
use kv_common::KvServer;
use tarpc::context::Context;
use uuid::Uuid;

impl KvServer for Server {
    async fn get(self, _: Context, key: String) -> Option<String> {
        self.services.kv_storage().get(&key)
    }

    async fn put(
        self,
        _: Context,
        key: String,
        value: String,
        idempotency_key: Option<Uuid>,
    ) -> String {
        match idempotency_key {
            Some(idempotency_key) => self
                .services
                .idempotency_key_storage()
                .get_or_map(idempotency_key, || {
                    self.services.kv_storage().put(key.as_str(), value.as_str())
                }),
            None => self.services.kv_storage().put(key.as_str(), value.as_str()),
        }
    }

    async fn append(
        self,
        _: Context,
        key: String,
        value: String,
        idempotency_key: Option<Uuid>,
    ) -> String {
        match idempotency_key {
            Some(idempotency_key) => self
                .services
                .idempotency_key_storage()
                .get_or_map(idempotency_key, || {
                    self.services.kv_storage().append(&key, &value)
                }),
            None => self.services.kv_storage().append(&key, &value),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use tarpc::context;
    use crate::server::Services;
    use super::*;

    #[tokio::test]
    async fn get_test() {
        let server = Server::new(Arc::new(Services::new()));
        let value = server.get(context::current(),"key".to_string()).await;
        assert!(value.is_none());
    }

    #[tokio::test]
    async fn put_test() {
        let server = Server::new(Arc::new(Services::new()));
        let value = server.put(context::current(),"key".to_string(), "value".to_string(), None).await;
        assert_eq!(value, "value");
    }
}