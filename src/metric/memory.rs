use super::{HostMetric, HostMetricKind, MetricValue};
use crate::Agent;
use os_stat::Memory;

impl From<Memory> for HostMetric {
    fn from(mem: Memory) -> Self {
        let kind = HostMetricKind::Memory;
        let mut value = MetricValue::new();

        value.insert("memory.total".into(), mem.total as f64);
        value.insert("memory.used".into(), mem.used as f64);
        value.insert("memory.swap_total".into(), mem.swap_total as f64);
        value.insert("memory.swap_cached".into(), mem.swap_cached as f64);
        value.insert("memory.swap_free".into(), mem.swap_free as f64);
        if mem.mem_available_enabled {
            value.insert("memory.mem_available".into(), mem.available as f64);
        } else {
            value.insert("memory.buffers".into(), mem.buffers as f64);
            value.insert("memory.cached".into(), mem.cached as f64);
            value.insert("memory.free".into(), mem.free as f64);
        }

        Self { kind, value }
    }
}

impl Agent {
    pub fn get_memory_metrics() -> HostMetric {
        let mem = Memory::get().expect("failed to get memory statistics");
        mem.into()
    }
}
