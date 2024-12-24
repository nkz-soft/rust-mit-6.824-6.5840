#[tarpc::service]
pub trait KvServer {
    async fn put(key: String, value: String) -> String;
    async fn get(key: String) -> Option<String>;
    async fn append(key: String, value: String) -> String;
}
