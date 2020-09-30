use crate::{Agent, Values};
use os_stat_rs::network;
use std::collections::HashMap;

const INTERNAL_SECONDS: u64 = 10;

impl From<(HashMap<String, f64>, HashMap<String, f64>)> for Values {
    fn from((previous, current): (HashMap<String, f64>, HashMap<String, f64>)) -> Self {
        let mut ret = HashMap::new();
        for (key, prev_val) in previous.into_iter() {
            if current.contains_key(&key) {
                let curr_val = current.get(&key).unwrap();
                if prev_val <= *curr_val {
                    ret.insert(
                        format!("{}.delta", key),
                        (curr_val - prev_val) / INTERNAL_SECONDS as f64,
                    );
                }
            }
        }
        Self(ret)
    }
}

fn network_to_hashmap(nw_stats: Vec<network::Network>) -> HashMap<String, f64> {
    let mut value = HashMap::new();
    for network in nw_stats.into_iter() {
        let name = crate::util::sanitize_metric_key(&network.name);
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

impl Agent {
    pub fn get_interfaces_metrics(&self) -> Option<Values> {
        let interval = std::time::Duration::from_secs(INTERNAL_SECONDS);
        let previous = network::get();
        std::thread::sleep(interval);
        let current = network::get();
        match (previous, current) {
            (Ok(previous), Ok(current)) => {
                let previous_metrics = network_to_hashmap(previous);
                let current_metrics = network_to_hashmap(current);
                Some((previous_metrics, current_metrics).into())
            }
            _ => None,
        }
    }
}
