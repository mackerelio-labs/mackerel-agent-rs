use metric::{HostMetric, MetricValue};
use std::{
    sync::mpsc::{self, channel},
    thread,
    time::Duration,
};
use tokio::time;

const INTERVAL: Duration = Duration::from_secs(60);

// &'a str expects host id.
#[derive(Debug, PartialEq)]
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
    // When failed to post metric, agent will heap up the metric for next time posting.
    metric: Vec<mackerel_client::metric::HostMetricValue>,
}

impl Agent {
    pub fn new(config: config::Config, host_id: String) -> Self {
        Self {
            client: Box::new(client::Client::new(&config.apikey)),
            config,
            host_id,
            metric: vec![],
        }
    }

    pub async fn run(&mut self) {
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

    async fn send_metric(&mut self, val: MetricValue) {
        let mut metric: Vec<_> = HostMetricWrapper(&self.host_id, val).into();
        metric.extend(self.metric.clone());
        let result = self.client.post_metrics(metric.clone()).await;
        // If Ok, then heaped metric must be empty, else extend it.
        // TODO: Drop metric if it is too ancient.
        self.metric = if result.is_ok() { vec![] } else { metric };
    }
}

pub mod config;
pub mod host_meta;

mod client;
mod metric;
mod util;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::Clientable;
    use futures::future::ready;
    use mackerel_client::errors::{Error, ErrorKind};
    use mockall::predicate::*;
    use reqwest::StatusCode;

    impl Agent {
        fn new_with_clientable(client: Box<dyn Clientable>) -> Self {
            Self {
                client,
                config: config::Config::default(),
                host_id: "host_id_1".to_string(),
                metric: vec![],
            }
        }
    }

    #[tokio::test]
    async fn heaping_metric() {
        let mut heaped_metric = MetricValue::new();
        heaped_metric.insert("cpu.user.percentage".to_string(), 20f64);

        let v: Vec<_> = HostMetricWrapper("host_id_1", heaped_metric.clone()).into();
        let mut mocked_client = client::MockClientable::new();

        // Test case for heaping up metric.
        mocked_client
            .expect_post_metrics()
            .with(eq(v.clone()))
            .times(1)
            .returning(move |_| {
                Box::pin(ready(Err(Error(
                    ErrorKind::ApiError(StatusCode::BAD_GATEWAY, "Bad gateway".to_string()),
                    error_chain::State::default(),
                ))))
            });
        let mut client = Agent::new_with_clientable(Box::new(mocked_client));
        client.send_metric(heaped_metric.clone()).await;
        assert_eq!(client.metric, v);

        // Test case for succeeding to post metric.
        heaped_metric.insert("cpu.guest.percentage".to_string(), 30f64);
        let v: Vec<_> = HostMetricWrapper("host_id_1", heaped_metric.clone()).into();
        let mut mocked_client = client::MockClientable::new();
        mocked_client
            .expect_post_metrics()
            .with(eq(v.clone()))
            .times(1)
            .returning(move |_| Box::pin(ready(Ok(()))));
        let mut client = Agent::new_with_clientable(Box::new(mocked_client));
        client.send_metric(heaped_metric.clone()).await;
        assert_eq!(client.metric, vec![]);
    }
}
