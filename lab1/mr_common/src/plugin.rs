use crate::KeyValue;

pub trait Plugin {
    fn map(&self, filename: &str, content: &str) -> Vec<KeyValue>;
    fn reduce(&self, key: &str, values: Vec<String>) -> String;
}
