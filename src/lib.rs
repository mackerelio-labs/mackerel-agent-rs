use mackerel_client::client::Client;
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
        let mut interval = time::interval(INTERVAL);
        loop {
            interval.tick().await;
            let (tx, rx) = channel();

            type F = Box<dyn Send + FnOnce() -> HostMetric>;
            let cpu_metric: F = Box::new(|| Self::get_cpu_metrics().unwrap());
            let disk_metric: F = Box::new(|| Self::get_disk_metrics().unwrap());
            let filesystem_metric: F = Box::new(Self::get_filesystem_metrics);
            let interfaces_metric: F = Box::new(|| Self::get_interfaces_metrics().unwrap());
            let loadavg_metric: F = Box::new(Self::get_loadavg_metric);
            let memory_metric: F = Box::new(Self::get_memory_metrics);
            let custom_metric: F = self.get_custom();

            for v in vec![
                cpu_metric,
                disk_metric,
                filesystem_metric,
                interfaces_metric,
                loadavg_metric,
                memory_metric,
                custom_metric,
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

    fn get_custom(&self) -> Box<dyn Send + FnOnce() -> HostMetric> {
        use std::process::Command;
        let kind = metric::HostMetricKind::Custom("custom".into());

        if let Some(command) = self.config.custom.clone() {
            Box::new(move || {
                let output = Command::new("sh")
                    .arg("-c")
                    .arg(command)
                    .output()
                    .expect("failed to execute process");
                let val = String::from_utf8(output.stdout).unwrap();
                let mut value = MetricValue::new();
                if let Ok(val) = val.trim_end().parse::<f64>() {
                    value.insert("custom.metric.value".into(), val);
                };
                HostMetric { kind, value }
            })
        } else {
            Box::new(move || HostMetric {
                kind,
                value: metric::MetricValue::new(),
            })
        }
    }
}

pub mod config;
pub mod host_meta;

mod metric;
mod util;

#[test]
fn agent_get_custom() {
    // case for subprocess exited normally.
    let mut cfg = config::Config::new();
    cfg.custom = Some("/bin/echo 10".into());
    let agent = Agent::new(cfg, "host_id".into());
    let cls = agent.get_custom();

    let expected = HostMetric {
        kind: metric::HostMetricKind::Custom("custom".into()),
        value: vec![("custom.metric.value".into(), 10f64)]
            .into_iter()
            .collect(),
    };
    assert_eq!(cls(), expected);

    // case for subprocess exited unormally.
    let mut cfg = config::Config::new();
    cfg.custom = Some("/bin/echo hello, world".into());
    let agent = Agent::new(cfg, "host_id".into());
    let cls = agent.get_custom();

    let expected = HostMetric {
        kind: metric::HostMetricKind::Custom("custom".into()),
        value: metric::MetricValue::new(),
    };
    assert_eq!(cls(), expected);
}
