use super::{HostMetric, HostMetricKind, MetricValue};
use crate::{util, Agent};
use os_stat::Disk;
use std::{collections::HashMap, time::Duration};

type Disks = Vec<Disk>;

// TODO: investigate how many seconds are used in mackerel-agent
const INTERVAL: Duration = Duration::from_secs(10);

impl From<(Disks, Disks)> for HostMetric {
    fn from((previous, current): (Disks, Disks)) -> Self {
        let kind = HostMetricKind::Disk;
        let previous_values: HashMap<_, _> = previous
            .into_iter()
            .map(|disk| {
                let sanitized_device_label = util::sanitize_metric_key(&disk.name);
                (sanitized_device_label, disk)
            })
            .collect();
        let current_values: HashMap<_, _> = current
            .into_iter()
            .map(|disk| {
                let sanitized_device_label = util::sanitize_metric_key(&disk.name);
                (sanitized_device_label, disk)
            })
            .collect();

        let mut value = MetricValue::new();
        for (device_label, previous) in previous_values {
            match current_values.get(&device_label) {
                None => continue,
                Some(current) => {
                    value.insert(
                        format!("disk.{}.reads.delta", device_label),
                        (current.reads_completed - previous.reads_completed) as f64
                            / INTERVAL.as_secs() as f64,
                    );
                    value.insert(
                        format!("disk.{}.writes.delta", device_label),
                        (current.writes_completed - previous.writes_completed) as f64
                            / INTERVAL.as_secs() as f64,
                    );
                }
            }
        }

        Self { kind, value }
    }
}
impl Agent {
    // TODO: When failed to get, returns None.
    pub fn get_disk_metrics() -> Option<HostMetric> {
        let previous = Disk::get().expect("failed to get disk statistics");
        std::thread::sleep(INTERVAL);
        let current = Disk::get().expect("failed to get disk statistics");
        Some((previous, current).into())
    }
}
