use anyhow::Context;
use log::info;
use mr_common::plugin::Plugin;
use mr_common::{Configuration, KeyValue, Task};
use std::collections::HashMap;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

pub struct PluginHolder {
    lib: Option<libloading::Library>,
}

impl PluginHolder {
    pub fn new() -> Self {
        PluginHolder { lib: None }
    }

    pub unsafe fn load_lib(&mut self, name: PathBuf) -> anyhow::Result<()> {
        if self.lib.is_none() {
            self.lib = Option::from(libloading::Library::new(name)?);
        }
        Ok(())
    }

    pub unsafe fn map(&self, task: &Task, configuration: &Configuration) -> anyhow::Result<()> {
        info!("Starting map for task {}", task.id);

        let file_name = task.file.as_ref().context("File name is not set")?;

        let mut full_path = configuration.path_to_files().clone();
        full_path = full_path.join(file_name);

        let content = std::fs::read_to_string(full_path)?;
        let kv_list = self.load_plugin()?.map(file_name, &content);

        let task_id = task.id;
        let reduce_task_num = configuration.reduce_task_num() as usize;

        let mut writers = Vec::with_capacity(reduce_task_num);
        for reduce_task in 0..reduce_task_num {
            let file = File::create(format!("mr-{task_id}-{reduce_task}"))?;
            writers.push((BufWriter::new(file), Vec::new()));
        }

        for kv in kv_list {
            let reduce_idx = Self::hash(kv.key.as_str(), reduce_task_num) as usize;
            writers[reduce_idx].1.push(kv);
        }

        for mut writer in writers {
            serde_json::to_writer(&mut writer.0, &writer.1)?;
            writer.0.flush()?;
        }

        info!("Finished map for task {}", task.id);
        Ok(())
    }

    pub unsafe fn reduce(&self, task: &Task, configuration: &Configuration) -> anyhow::Result<()> {
        info!("Starting reduce for task {}", task.id);

        let task_id = task.parent.context("Parent task is not set")?;

        let reduce_task_num = configuration.reduce_task_num() as usize;

        let mut intermediate_key_values: HashMap<String, Vec<String>> = HashMap::new();

        for reduce_task in 0..reduce_task_num {
            info!("Loading key values from mr-{task_id}-{reduce_task}");
            let file = File::open(format!("mr-{task_id}-{reduce_task}"))?;

            let content: Vec<KeyValue> = serde_json::from_reader(BufReader::new(file))?;
            for kv in content {
                intermediate_key_values
                    .entry(kv.key)
                    .and_modify(|v: &mut Vec<String>| v.push(kv.value.clone()))
                    .or_insert(vec![kv.value.clone()]);
            }
        }

        let mut sorted_keys = intermediate_key_values
            .keys()
            .cloned()
            .collect::<Vec<String>>();
        sorted_keys.sort();

        let file = File::create(format!("mr-out-{task_id}"))?;
        let mut writer = BufWriter::new(file);

        for key in sorted_keys {
            let kv_list = intermediate_key_values
                .get(&key)
                .context("Key is not found")?;

            let key_len = self.load_plugin()?.reduce(&key.clone(), kv_list.clone());
            writeln!(writer, "{} {}", key, key_len)?;
            writer.flush()?;
        }

        info!("Finished reduce for task {}", task.id);
        Ok(())
    }

    unsafe fn load_plugin(&self) -> anyhow::Result<Box<dyn Plugin>> {
        let plugin_instance: libloading::Symbol<extern "Rust" fn() -> Box<dyn Plugin>> =
            unsafe { self.lib.as_ref().unwrap().get(b"load_plugin") }?;
        Ok(plugin_instance())
    }

    fn hash(str: &str, reduce_task_num: usize) -> u32 {
        let mut s = DefaultHasher::new();
        str.hash(&mut s);
        s.finish() as u32 % reduce_task_num as u32
    }
}
