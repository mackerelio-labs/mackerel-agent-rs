use super::super::Agent;
use super::{HostMetric, HostMetricKind, MetricValue};
use os_stat::CPU;
use std::{thread, time::Duration};

// TODO: investigate how many seconds are used in mackerel-agent
const DURATION: Duration = Duration::from_secs(10);

impl From<(CPU, CPU)> for HostMetric {
    // https://github.com/mackerelio/mackerel-agent/blob/d9e3082a32b96c17560a375e5e78babcb0f34e8d/metrics/linux/cpuusage.go#L31-L75
    fn from((previous, current): (CPU, CPU)) -> Self {
        let kind = HostMetricKind::CPU;
        let mut value = MetricValue::new();
        let total_diff = (current.total - previous.total) as f64;
        let cpu_count = current.cpu_count as f64;

        macro_rules! val_insert_inner {
            ($key:expr, $val:expr) => {
                value.insert(
                    $key.into(),
                    $val as f64 * cpu_count as f64 * 100.0 / total_diff,
                );
            };
        }
        val_insert_inner!(
            "cpu.user.percentage",
            (current.user - current.guest) - (previous.user - previous.guest)
        );

        macro_rules! val_insert {
            ($field:ident) => {
                let field = stringify!($field);
                let key = format!("cpu.{}.percentage", field);
                val_insert_inner!(key, current.$field - previous.$field)
            };
        }

        val_insert!(nice);
        val_insert!(system);
        val_insert!(idle);
        val_insert!(iowait);
        val_insert!(irq);
        val_insert!(softirq);
        val_insert!(steal);
        val_insert!(guest);

        Self { kind, value }
    }
}

impl Agent {
    pub fn get_cpu_metrics() -> Option<HostMetric> {
        let previous = CPU::get();
        thread::sleep(DURATION);
        let current = CPU::get();
        match (previous, current) {
            (Ok(previous), Ok(current)) => Some((previous, current).into()),
            _ => None,
        }
    }
}
