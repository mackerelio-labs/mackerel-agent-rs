use super::{HostMetric, HostMetricKind, MetricValue};
use crate::Agent;
use os_stat::FileSystem;

impl From<Vec<FileSystem>> for HostMetric {
    fn from(fss: Vec<FileSystem>) -> Self {
        let kind = HostMetricKind::FileSystem;
        let mut value = MetricValue::new();
        for fs in fss {
            value.insert(format!("filesystem.{}.size", fs.name), fs.size as f64);
            value.insert(format!("filesystem.{}.used", fs.name), fs.used as f64);
        }
        Self { value, kind }
    }
}

impl Agent {
    pub fn get_filesystem_metrics() -> HostMetric {
        let stats = FileSystem::get().expect("failed to get filesystem metrics");
        stats.into()
    }
}
