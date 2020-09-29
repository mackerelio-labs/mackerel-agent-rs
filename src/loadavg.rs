use crate::{Agent, Values};
use os_stat_rs::loadavg;
use std::collections::HashMap;

impl Agent {
    pub fn get_loadavg_metric(&self) -> Values {
        let loadavg_stats = loadavg::get();
        let mut values = HashMap::new();
        values.insert("loadavg1".into(), loadavg_stats.loadavg1);
        values.insert("loadavg5".into(), loadavg_stats.loadavg5);
        values.insert("loadavg15".into(), loadavg_stats.loadavg15);
        Values(values)
    }
}
