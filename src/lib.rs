use mackerel_client::client::Client;
use std::{
    collections::HashMap,
    sync::mpsc::{self, channel},
    thread,
    time::Duration,
};
use tokio::time;

#[derive(Debug)]
pub struct Values(HashMap<String, f64>);
// &'a str expects host id.
pub struct HostMetricWrapper<'a>(&'a str, Values);

impl<'a> Into<Vec<mackerel_client::metric::HostMetricValue>> for HostMetricWrapper<'a> {
    fn into(self) -> Vec<mackerel_client::metric::HostMetricValue> {
        use std::time::SystemTime;
        let host_id = self.0;
        let value = self.1;
        let host_metric_value = value.0;
        let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };
        host_metric_value
            .into_iter()
            .map(|hmv| {
                let (name, value) = hmv;
                mackerel_client::metric::HostMetricValue {
                    host_id: host_id.to_owned(),
                    name,
                    value,
                    time: now,
                }
            })
            .collect()
    }
}

impl std::ops::Deref for Values {
    type Target = HashMap<String, f64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for Values {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct Agent {
    pub config: config::Config,
    pub client: Client,
    pub host_id: String,
}

impl Agent {
    pub fn new(config: config::Config, host_id: String) -> Self {
        Self {
            client: Client::new(&config.apikey),
            config,
            host_id,
        }
    }

    pub async fn run(&self) {
        let mut interval = time::interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            let (tx, rx) = channel();
            // TODO: Quit using Values, then use metric::HostMetric.
            type F = Box<dyn Fn() -> Values + Send>;
            let cpu_metric: F = Box::new(|| {
                let metrics = Self::get_cpu_metrics().unwrap();
                // This line is prepared for only solving type-puzzle...
                // After when all metrics are done moving to src/metrics, these line will get useless.
                Values(metrics.value)
            });
            let disk_metric: F = Box::new(|| Values(Self::get_disk_metrics().unwrap().value));
            let filesystem_metric: F = Box::new(|| Values(Self::get_filesystem_metrics().value));
            let interfaces_metric: F =
                Box::new(|| Values(Self::get_interfaces_metrics().unwrap().value));
            let loadavg_metric: F = Box::new(|| Values(Self::get_loadavg_metric().value));
            let memory_metric: F = Box::new(Self::get_memory_metrics);

            let mut metrics = Values(HashMap::new());
            for v in vec![
                cpu_metric,
                disk_metric,
                filesystem_metric,
                interfaces_metric,
                loadavg_metric,
                memory_metric,
            ] {
                let tx = mpsc::Sender::clone(&tx);
                thread::spawn(move || {
                    let val = v();
                    tx.send(val).unwrap();
                });
            }

            // drop tx explicitly because of breaking for ... in rx
            drop(tx);

            for recieved_metrics in rx {
                metrics.extend(recieved_metrics.0);
            }

            self.send_metric(metrics).await;
        }
    }

    async fn send_metric(&self, val: Values) {
        let metric = HostMetricWrapper(&self.host_id, val).into();
        // TODO: error handling.
        let result = self.client.post_metrics(metric).await;
        if result.is_err() {
            dbg!(result.err());
        }
    }
}

pub mod config;
pub mod host_meta;

mod metric;
mod util;
