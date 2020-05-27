use anyhow::Result;
use rusoto_ce::{GetCostAndUsageResponse, Group, ResultByTime};
use slack_hook::{Attachment, Field, HexColor, SlackText};

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

#[derive(Debug, Clone)]
pub struct Cost {
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
            value: SlackText::new(format!("${}", amount.clone().amount)),
            short: Some(true),
        })
        .collect();
    let sum: f64 = amounts
        .iter()
        .map(|a| a.amount.parse::<f64>().unwrap())
        .sum();

    let attachments = vec![Attachment {
        fallback: SlackText::new("attachment"),
        text: None,
        pretext: Some(SlackText::new(format!(
            "{} ~ {} のAWS利用料金 は ${}です",
            start, end, sum
        ))),
        color: Some(HexColor::default()),
        fields: Some(fields),
        author_name: Some(SlackText::new("内訳")),
        author_link: None,
        author_icon: None,
        title: None,
        title_link: None,
        image_url: None,
        thumb_url: None,
        footer: None,
        footer_icon: None,
        ts: None,
        mrkdwn_in: None,
    }];

    attachments
}

pub fn deserialize(cost: GetCostAndUsageResponse) -> Result<Vec<Cost>> {
    let costs_by_service: Vec<Cost> = cost
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

            Cost {
                attachments: create_attachment_from_cost(period, amounts),
            }
        })
        .collect();

    Ok(costs_by_service)
}
