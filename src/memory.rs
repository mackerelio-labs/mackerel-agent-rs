use crate::{Agent, Values};
use os_stat_rs::memory;
use std::collections::HashMap;

impl Agent {
    pub fn get_memory_metrics(&self) -> Values {
        let mem = memory::get().expect("failed to get memory statistics");
        let mut values = HashMap::new();
        values.insert("memory.total".into(), mem.total as f64);
        values.insert("memory.used".into(), mem.used as f64);
        values.insert("memory.swap_total".into(), mem.swap_total as f64);
        values.insert("memory.swap_cached".into(), mem.swap_cached as f64);
        values.insert("memory.swap_free".into(), mem.swap_free as f64);
        if mem.mem_available_enabled {
            values.insert("memory.mem_available".into(), mem.available as f64);
        } else {
            values.insert("memory.buffers".into(), mem.buffers as f64);
            values.insert("memory.cached".into(), mem.cached as f64);
            values.insert("memory.free".into(), mem.free as f64);
        }
        Values(values)
    }
}
