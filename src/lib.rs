use mackerel_client::{client::Client, metric};
use std::{collections::HashMap, time::Duration};
use tokio::time;

#[derive(Debug)]
pub struct Values(HashMap<String, f64>);
// &'a str expects host id.
pub struct HostMetricWrapper<'a>(&'a str, Values);

impl<'a> Into<Vec<metric::HostMetricValue>> for HostMetricWrapper<'a> {
    fn into(self) -> Vec<metric::HostMetricValue> {
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
                metric::HostMetricValue {
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
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            let cpu_metric = self.get_cpu_metrics().unwrap();
            let filesystem_metric = self.get_filesystem_metrics();
            let interfaces_metric = self.get_interfaces_metrics().unwrap();
            let loadavg_metric = self.get_loadavg_metric();
            let memory_metric = self.get_memory_metrics();
            let disk_metric = self.get_disk_metrics().unwrap();
            let mut metrics = Values(HashMap::new());
            for v in vec![
                cpu_metric,
                disk_metric,
                filesystem_metric,
                interfaces_metric,
                loadavg_metric,
                memory_metric,
            ] {
                metrics.extend(v.0);
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

mod cpu;
mod disk;
mod filesystem;
mod interface;
mod loadavg;
mod memory;
mod util;
