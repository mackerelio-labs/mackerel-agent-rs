use metric::{HostMetric, MetricValue};
use std::{
    sync::mpsc::{self, channel},
    thread,
    time::Duration,
};
use tokio::time;

const INTERVAL: Duration = Duration::from_secs(60);

// &'a str expects host id.
pub struct HostMetricWrapper<'a>(&'a str, MetricValue);

impl<'a> Into<Vec<mackerel_client::metric::HostMetricValue>> for HostMetricWrapper<'a> {
    fn into(self) -> Vec<mackerel_client::metric::HostMetricValue> {
        use std::time::SystemTime;
        let host_id = self.0;
        let value = self.1;
        let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };
        value
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

pub struct Agent {
    pub config: config::Config,
    pub client: Box<dyn client::Clientable>,
    pub host_id: String,
}

impl Agent {
    pub fn new(config: config::Config, host_id: String) -> Self {
        Self {
            client: Box::new(client::Client::new(&config.apikey)),
            config,
            host_id,
        }
    }

    pub async fn run(&self) {
        let mut interval = time::interval(INTERVAL);
        loop {
            interval.tick().await;
            let (tx, rx) = channel();

            type F = Box<dyn Fn() -> HostMetric + Send>;
            let cpu_metric: F = Box::new(|| Self::get_cpu_metrics().unwrap());
            let disk_metric: F = Box::new(|| Self::get_disk_metrics().unwrap());
            let filesystem_metric: F = Box::new(Self::get_filesystem_metrics);
            let interfaces_metric: F = Box::new(|| Self::get_interfaces_metrics().unwrap());
            let loadavg_metric: F = Box::new(Self::get_loadavg_metric);
            let memory_metric: F = Box::new(Self::get_memory_metrics);

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

            // drop tx explicitly because mpsc::Reciever waits until all senders dropping.
            // https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html#method.iter
            drop(tx);

            let metrics = rx.into_iter().fold(MetricValue::new(), |mut acc, metric| {
                acc.extend(metric.value);
                acc
            });
            self.send_metric(metrics).await;
        }
    }

    async fn send_metric(&self, val: MetricValue) {
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

mod client;
mod metric;
mod util;
