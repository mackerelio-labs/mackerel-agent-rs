use crate::{util, Agent, Values};
use os_stat_rs::disk;
use std::{collections::HashMap, time::Duration};

impl Agent {
    pub fn get_disk_metrics(&self) -> Option<Values> {
        let interval = Duration::from_secs(10);
        let previous = disk::get().expect("failed to get disk statistics");
        std::thread::sleep(interval);
        let current = disk::get().expect("failed to get disk statistics");
        let mut previous_values = HashMap::new();
        for v in &previous {
            previous_values.insert(v.name.clone(), v);
        }
        let mut current_values = HashMap::new();
        for v in &current {
            current_values.insert(v.name.clone(), v);
        }
        let mut values = HashMap::new();
        for (device_label, previous) in previous_values {
            let sanitized_device_label = util::sanitize_metric_key(&device_label);
            match current_values.get(&device_label) {
                None => continue,
                Some(current) => {
                    values.insert(
                        format!("disk.{}.reads.delta", sanitized_device_label),
                        (current.reads_completed - previous.reads_completed) as f64
                            / interval.as_secs() as f64,
                    );
                    values.insert(
                        format!("disk.{}.writes.delta", sanitized_device_label),
                        (current.writes_completed - previous.writes_completed) as f64
                            / interval.as_secs() as f64,
                    );
                }
            }
        }
        Some(Values(values))
    }
}
