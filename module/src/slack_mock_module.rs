use crate::slack_module::SlackModule;
use async_trait::async_trait;
use presenter::slack::SlackBody;

pub struct MockSlack {}

#[async_trait]
impl SlackModule for MockSlack {
    fn new() -> Self {
        MockSlack {}
    }

    async fn send(&self, body: SlackBody) {
        println!("slackへ料金を送信: body: {:?}", body);
    }
}
