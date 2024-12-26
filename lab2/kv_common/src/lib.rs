use uuid::Uuid;

#[tarpc::service]
pub trait KvServer {
    async fn get(key: String) -> Option<String>;
    async fn put(key: String, value: String, idempotency_key: Option<Uuid>) -> String;
    async fn append(key: String, value: String, idempotency_key: Option<Uuid>) -> String;
}
