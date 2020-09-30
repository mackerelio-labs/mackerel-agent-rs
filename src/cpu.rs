use crate::{Agent, Values};
use os_stat::CPU;
use std::{collections::HashMap, time::Duration};

impl From<(CPU, CPU)> for Values {
    // https://github.com/mackerelio/mackerel-agent/blob/d9e3082a32b96c17560a375e5e78babcb0f34e8d/metrics/linux/cpuusage.go#L31-L75
    fn from((previous, current): (CPU, CPU)) -> Self {
        let mut value = HashMap::new();
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

        macro_rules! val_insert_if_bigger {
            ($stat_count:expr, $field:ident) => {
                if current.stat_count >= $stat_count {
                    val_insert!($field);
                }
            };
        }

        val_insert_if_bigger!(5, iowait);
        val_insert_if_bigger!(6, irq);
        val_insert_if_bigger!(7, softirq);
        val_insert_if_bigger!(8, steal);
        val_insert_if_bigger!(9, guest);
        Self(value)
    }
}

impl Agent {
    pub fn get_cpu_metrics() -> Option<Values> {
        let interval = Duration::from_secs(10);
        let previous = CPU::get();
        std::thread::sleep(interval);
        let current = CPU::get();
        match (previous, current) {
            (Ok(previous), Ok(current)) => Some((previous, current).into()),
            _ => None,
        }
    }
}
