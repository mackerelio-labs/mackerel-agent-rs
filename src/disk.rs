use crate::{util, Agent, Values};
use os_stat::Disk;
use std::{collections::HashMap, time::Duration};

impl Agent {
    pub fn get_disk_metrics() -> Option<Values> {
        let interval = Duration::from_secs(10);
        let previous = Disk::get().expect("failed to get disk statistics");
        std::thread::sleep(interval);
        let current = Disk::get().expect("failed to get disk statistics");
        let mut previous_values = HashMap::new();
        for v in previous {
            let sanitized_device_label = util::sanitize_metric_key(&v.name);
            previous_values.insert(sanitized_device_label, v);
        }
        let mut current_values = HashMap::new();
        for v in current {
            let sanitized_device_label = util::sanitize_metric_key(&v.name);
            current_values.insert(sanitized_device_label, v);
        }
        let mut values = HashMap::new();
        for (device_label, previous) in previous_values {
            match current_values.get(&device_label) {
                None => continue,
                Some(current) => {
                    values.insert(
                        format!("disk.{}.reads.delta", device_label),
                        (current.reads_completed - previous.reads_completed) as f64
                            / interval.as_secs() as f64,
                    );
                    values.insert(
                        format!("disk.{}.writes.delta", device_label),
                        (current.writes_completed - previous.writes_completed) as f64
                            / interval.as_secs() as f64,
                    );
                }
            }
        }
        Some(Values(values))
    }
}
