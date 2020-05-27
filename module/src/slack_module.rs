use slack_hook::{Attachment, PayloadBuilder, Slack};
use std::env;

pub struct Client {
    slack: Slack,
}

pub trait SlackModule {
    fn new() -> Self;
    fn send(&self, channel: String, user_name: String, text: String, attachments: Vec<Attachment>);
}

impl SlackModule for Client {
    fn new() -> Self {
        let webhook_url = env::var("SLACK_WEBHOOK_URL")
            .unwrap_or_else(|_| panic!("SLACK_WEBHOOK_URL is not found."));

        Client {
            slack: Slack::new(webhook_url.as_str()).unwrap(),
        }
    }

    fn send(&self, channel: String, user_name: String, text: String, attachments: Vec<Attachment>) {
        let p = PayloadBuilder::new()
            .channel(channel)
            .username(user_name)
            .text(text)
            .attachments(attachments)
            .build()
            .unwrap();

        let _ = self.slack.send(&p);
    }
}
