use crate::{Agent, Values};
use os_stat::FileSystem;
use std::collections::HashMap;

impl Agent {
    pub fn get_filesystem_metrics() -> Values {
        let stats = FileSystem::get().expect("failed to get filesystem metrics");
        let mut values = HashMap::new();
        for stats_item in stats {
            values.insert(
                format!("filesystem.{}.size", stats_item.name),
                stats_item.size as f64,
            );
            values.insert(
                format!("filesystem.{}.used", stats_item.name),
                stats_item.used as f64,
            );
        }
        Values(values)
    }
}
