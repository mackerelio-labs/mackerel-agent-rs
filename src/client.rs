use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

// This trait is defined because of mocking tests.
#[async_trait]
#[cfg_attr(test, automock)]
pub trait Clientable {
    // see: https://github.com/Krout0n/mackerel-client-rs/blob/e49fc4fb535363db30aa876ccbe030ad7b59a8b0/src/host.rs#L129-L138
    async fn post_metrics(
        &self,
        param: Vec<mackerel_client::metric::HostMetricValue>,
    ) -> Result<(), mackerel_client::errors::Error>;
}

// Wrapper of mackerel_client::client::Client.
// This type is defined for satisfying Clientable.
pub struct Client(pub mackerel_client::client::Client);

impl Client {
    pub fn new(api_key: &str) -> Self {
        Self(mackerel_client::client::Client::new(api_key))
    }
}

#[async_trait]
impl Clientable for Client {
    async fn post_metrics(
        &self,
        param: Vec<mackerel_client::metric::HostMetricValue>,
    ) -> Result<(), mackerel_client::errors::Error> {
        self.0.post_metrics(param).await
    }
}
