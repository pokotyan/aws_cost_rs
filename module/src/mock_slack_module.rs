use crate::slack_module::SlackModule;
use slack_hook::Attachment;

pub struct MockClient {}

impl SlackModule for MockClient {
    fn new() -> Self {
        MockClient {}
    }

    fn send(&self, channel: String, user_name: String, text: String, attachments: Vec<Attachment>) {
        println!(
            "slackへメッセージを送信: channel: {}, user_name: {}, text: {}, attachments: {:?}",
            channel, user_name, text, attachments
        );
    }
}
