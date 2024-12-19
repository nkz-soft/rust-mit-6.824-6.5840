use mr_common::plugin::Plugin;
use mr_common::KeyValue;

#[no_mangle]
pub extern "Rust" fn load_plugin() -> Box<dyn Plugin> {
    Box::new(WcPlugin::new())
}

pub struct WcPlugin {}

impl WcPlugin {
    fn new() -> WcPlugin {
        WcPlugin {}
    }
}

impl Plugin for WcPlugin {
    fn map(&self, _filename: &str, content: &str) -> Vec<KeyValue> {
        let mut result = Vec::new();
        let mut word: String = String::new();

        for char in content.chars() {
            if char.is_alphabetic() {
                word += &*char.to_string();
            } else if !word.is_empty() {
                result.push(KeyValue {
                    key: word,
                    value: "1".into(),
                });
                word = String::new();
            }
        }
        result
    }

    fn reduce(&self, _key: &str, values: Vec<String>) -> String {
        values.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_test() {
        let result = WcPlugin::new().map("test.txt", "Hello world,  my  world!");
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn reduce_test() {
        let result = WcPlugin::new().reduce("world", vec!["1".into(), "1".into()]);
        assert_eq!(result, "2");
    }
}
