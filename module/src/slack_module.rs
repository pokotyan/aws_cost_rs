use async_trait::async_trait;
use presenter::slack::SlackBody;
use reqwest::Client;
use serde_json;
use std::env;

pub struct Slack {
    webhook_url: String,
    client: Client,
}

#[async_trait]
pub trait SlackModule {
    fn new() -> Self;
    async fn send(&self, body: SlackBody);
}

#[async_trait]
impl SlackModule for Slack {
    fn new() -> Self {
        let webhook_url = env::var("SLACK_WEBHOOK_URL")
            .unwrap_or_else(|_| panic!("SLACK_WEBHOOK_URL is not found."));

        Slack {
            webhook_url,
            client: Client::new(),
        }
    }

    async fn send(&self, body: SlackBody) {
        let serialized_body = serde_json::to_string(&body).unwrap();

        self.client
            .post(self.webhook_url.as_str())
            .body(serialized_body)
            .send()
            .await
            .unwrap();
    }
}
