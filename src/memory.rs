use crate::{Agent, Values};
use os_stat_rs::memory;
use std::{collections::HashMap, time::Duration};

impl Agent {
    pub async fn get_memory_metrics(&self) -> Option<Values> {
        let mem = memory::get().expect("failed to get memory statistics");
        let mut values = HashMap::new();
        values.insert("memory.total".into(), mem.total as f64);
        values.insert("memory.used".into(), mem.used as f64);
        values.insert("memory.swap_total".into(), mem.swap_total as f64);
        values.insert("memory.swap_cached".into(), mem.swap_cached as f64);
        values.insert("memory.swap_free".into(), mem.swap_free as f64);
        if mem.mem_available {
            values.insert("memory.mem_available", mem.available);
        } else {
            values.insert("memory.buffers", mem.buffers);
            values.insert("memory.cached", mem.cached);
            values.insert("memory.free", mem.free);
        }
        Some(Values(values))
    }
}
