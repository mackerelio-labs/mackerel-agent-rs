use std::collections::HashMap;

// Metric-name and its own value.
pub type MetricValue = HashMap<String, f64>;

// This enum is used for debugging.
#[derive(Debug, PartialEq)]
pub enum HostMetricKind {
    CPU,
    Disk,
    FileSystem,
    Interface,
    LoadAvg,
    Memory,
    Custom(String),
}

#[derive(Debug, PartialEq)]
pub struct HostMetric {
    pub kind: HostMetricKind,
    pub value: MetricValue,
}

mod cpu;
mod disk;
mod filesystem;
mod interface;
mod loadavg;
mod memory;
