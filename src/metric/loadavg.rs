use super::{HostMetric, HostMetricKind, MetricValue};
use crate::Agent;
use os_stat::LoadAvg;

impl From<LoadAvg> for HostMetric {
    fn from(loadavg_stats: LoadAvg) -> Self {
        let kind = HostMetricKind::LoadAvg;
        let mut value = MetricValue::new();
        value.insert("loadavg1".into(), loadavg_stats.loadavg1);
        value.insert("loadavg5".into(), loadavg_stats.loadavg5);
        value.insert("loadavg15".into(), loadavg_stats.loadavg15);
        Self { kind, value }
    }
}

impl Agent {
    pub fn get_loadavg_metric() -> HostMetric {
        let loadavg_stats = LoadAvg::get();
        loadavg_stats.into()
    }
}
