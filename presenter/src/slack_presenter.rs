use anyhow::Result;
use rusoto_ce::{GetCostAndUsageResponse, Group, ResultByTime};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Amount {
    pub service: String,
    pub amount: String,
}

#[derive(Debug, Clone)]
pub struct Period {
    pub start_date: String,
    pub end_date: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct Field {
    pub title: String,
    pub value: String,
    pub short: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct Attachment {
    pub fallback: String,
    pub color: String,
    pub pretext: String,
    pub author_name: String,
    pub fields: Vec<Field>,
}

#[derive(Serialize, Debug, Clone)]
pub struct SlackBody {
    pub username: String,
    pub channel: String,
    pub text: String,
    pub attachments: Vec<Attachment>,
}

fn get_service(group: &Group) -> String {
    group.keys.as_ref().unwrap().first().unwrap().to_string()
}

fn get_unblended_cost(group: &Group) -> String {
    let cost = group
        .metrics
        .as_ref()
        .unwrap()
        .get("UnblendedCost")
        .unwrap()
        .amount
        .as_ref()
        .unwrap()
        .parse::<String>()
        .unwrap();
    cost
}

fn get_period(result_by_time: &ResultByTime) -> Period {
    let start = result_by_time.time_period.as_ref().unwrap().start.clone();
    let end = result_by_time.time_period.as_ref().unwrap().end.clone();

    Period {
        start_date: start,
        end_date: end,
    }
}

fn create_attachment_from_cost(period: Period, amounts: Vec<Amount>) -> Vec<Attachment> {
    let start = period.start_date;
    let end = period.end_date;
    let fields = amounts
        .iter()
        .map(|amount| Field {
            title: amount.clone().service,
            value: format!("${}", amount.clone().amount),
            short: true,
        })
        .collect();
    let sum: f64 = amounts
        .iter()
        .map(|a| a.amount.parse::<f64>().unwrap())
        .sum();

    let attachments = vec![Attachment {
        fallback: "attachment".to_string(),
        pretext: format!("{} ~ {} のAWS利用料金 は ${}です", start, end, sum),
        color: "#2eb886".to_string(),
        fields,
        author_name: "内訳".to_string(),
    }];

    attachments
}

pub fn deserialize(cost: GetCostAndUsageResponse, channel: String) -> Result<Vec<SlackBody>> {
    let costs_by_service: Vec<SlackBody> = cost
        .results_by_time
        .unwrap()
        .iter()
        .map(|result_by_time| {
            let period = get_period(&result_by_time);
            let groups = result_by_time.groups.as_ref().unwrap();
            let amounts = groups
                .iter()
                .map(|group| Amount {
                    service: Some(get_service(group)).unwrap(),
                    amount: get_unblended_cost(group),
                })
                .collect();

            SlackBody {
                username: "AWS Cost".to_string(),
                channel: channel.clone(),
                attachments: create_attachment_from_cost(period, amounts),
                text: " ".to_string(),
            }
        })
        .collect();

    Ok(costs_by_service)
}
