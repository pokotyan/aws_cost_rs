use async_trait::async_trait;
use infra::{AwsRepository, GetCostAndUsageResponse, GetCostRequest};
use module::slack::SlackModule;
use presenter::slack::deserialize;

pub struct GetCost<T, U>
where
    T: AwsRepository,
    U: SlackModule,
{
    aws_repository: T,
    slack: U,
}

#[async_trait]
pub trait UseCase {
    async fn new() -> Self;
    async fn run(&self, start: Option<String>, end: Option<String>, channel: Option<String>);
}

#[async_trait]
impl<T: Sync + Send + AwsRepository, U: Sync + Send + SlackModule> UseCase for GetCost<T, U> {
    async fn new() -> Self {
        let aws_repository = AwsRepository::new().await;
        let slack = SlackModule::new();

        GetCost {
            aws_repository,
            slack,
        }
    }

    async fn run(&self, start: Option<String>, end: Option<String>, channel: Option<String>) {
        let cost: GetCostAndUsageResponse;
        let channel = channel.unwrap_or_else(|| "#cost".to_string());

        if start.is_none() || end.is_none() {
            cost = self.aws_repository.get_cost(None).await.unwrap();
        } else {
            let req = GetCostRequest {
                start_date: start.unwrap(),
                end_date: end.unwrap(),
            };

            cost = self.aws_repository.get_cost(Some(req)).await.unwrap();
        }

        let body_list = deserialize(cost, channel.clone()).unwrap();

        for body in body_list {
            self.slack.send(body).await;
        }
    }
}
