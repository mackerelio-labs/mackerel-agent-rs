use crate::{Agent, Values};
use os_stat::LoadAvg;
use std::collections::HashMap;

impl Agent {
    pub fn get_loadavg_metric() -> Values {
        let loadavg_stats = LoadAvg::get();
        let mut values = HashMap::new();
        values.insert("loadavg1".into(), loadavg_stats.loadavg1);
        values.insert("loadavg5".into(), loadavg_stats.loadavg5);
        values.insert("loadavg15".into(), loadavg_stats.loadavg15);
        Values(values)
    }
}
