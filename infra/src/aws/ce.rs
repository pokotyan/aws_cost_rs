use anyhow::Result;
use chrono::Local;
use rusoto_ce::{
    CostExplorer, CostExplorerClient, DateInterval, GetCostAndUsageRequest,
    GetCostAndUsageResponse, GroupDefinition,
};

#[derive(Debug)]
pub struct GetCostRequest {
    pub start_date: String,
    pub end_date: String,
}

impl Default for GetCostRequest {
    fn default() -> GetCostRequest {
        let now = Local::now();

        GetCostRequest {
            start_date: now.format("%Y-%m-01").to_string(),
            end_date: now.format("%Y-%m-%d").to_string(),
        }
    }
}

pub async fn get_cost(
    client: &CostExplorerClient,
    input: Option<GetCostRequest>,
) -> Result<GetCostAndUsageResponse> {
    let req = input.unwrap_or_else(|| GetCostRequest::default());

    let req = GetCostAndUsageRequest {
        granularity: Some("MONTHLY".to_owned()),
        metrics: Some(vec!["UnblendedCost".to_string()]), // https://qiita.com/tamura_CD/items/4a9a412faf379b334986
        time_period: DateInterval {
            start: req.start_date,
            end: req.end_date,
        },
        filter: None,
        group_by: Some(vec![GroupDefinition {
            key: Some("SERVICE".to_string()),
            type_: Some("DIMENSION".to_string()),
        }]),
        next_page_token: None,
    };

    let res = client.get_cost_and_usage(req).await.unwrap();

    Ok(res)
}
