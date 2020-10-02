use super::{HostMetric, HostMetricKind, MetricValue};
use crate::{util, Agent};
use os_stat::Network;
use std::time::Duration;

// TODO: investigate how many seconds are used in mackerel-agent
const DURATION: Duration = Duration::from_secs(10);
type Networks = Vec<Network>;

impl From<(Networks, Networks)> for HostMetric {
    fn from((previous, current): (Networks, Networks)) -> Self {
        fn network_to_metricvalue(nw_stats: Networks) -> MetricValue {
            let mut value = MetricValue::new();
            for network in nw_stats.into_iter() {
                let name = util::sanitize_metric_key(&network.name);
                value.insert(
                    format!("interface.{}.rxBytes", name),
                    network.rx_bytes as f64,
                );
                value.insert(
                    format!("interface.{}.txBytes", name),
                    network.tx_bytes as f64,
                );
            }
            value
        }
        let previous = network_to_metricvalue(previous);
        let current = network_to_metricvalue(current);

        let kind = HostMetricKind::Interface;
        let mut value = MetricValue::new();
        for (key, prev_val) in previous.into_iter() {
            if current.contains_key(&key) {
                let curr_val = current.get(&key).unwrap();
                if prev_val <= *curr_val {
                    value.insert(
                        format!("{}.delta", key),
                        (curr_val - prev_val) / DURATION.as_secs_f64(),
                    );
                }
            }
        }
        Self { kind, value }
    }
}

impl Agent {
    pub fn get_interfaces_metrics() -> Option<HostMetric> {
        let previous = Network::get();
        std::thread::sleep(DURATION);
        let current = Network::get();
        match (previous, current) {
            (Ok(previous), Ok(current)) => Some((previous, current).into()),
            _ => None,
        }
    }
}
