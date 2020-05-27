use async_trait::async_trait;
use dotenv::dotenv;
use rusoto_ce::CostExplorerClient;
use rusoto_core::Region;
use rusoto_credential::{EnvironmentProvider, ProvideAwsCredentials};
mod ce;

pub use ce::GetCostRequest;
pub use rusoto_ce::GetCostAndUsageResponse;

pub struct AWS {
    cost_explorer_client: CostExplorerClient,
}

#[async_trait]
pub trait AwsRepository {
    async fn new() -> Self;
    async fn get_cost(
        &self,
        input: Option<ce::GetCostRequest>,
    ) -> Result<GetCostAndUsageResponse, ()>;
}

#[async_trait]
impl AwsRepository for AWS {
    async fn new() -> Self {
        dotenv().ok();
        EnvironmentProvider::default().credentials().await.unwrap();

        AWS {
            cost_explorer_client: CostExplorerClient::new(Region::UsEast1),
        }
    }

    async fn get_cost(
        &self,
        input: Option<ce::GetCostRequest>,
    ) -> Result<GetCostAndUsageResponse, ()> {
        let res = ce::get_cost(&self.cost_explorer_client, input)
            .await
            .unwrap();

        Ok(res)
    }
}
