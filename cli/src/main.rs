use infra::AWS;
use module::slack::Slack;
use structopt::StructOpt;
mod get_cost;
use get_cost::*;

#[derive(StructOpt, Debug)]
#[structopt()]
enum Cli {
    Cost {
        #[structopt(short = "s", long = "start-date")]
        start: Option<String>,
        #[structopt(short = "e", long = "end-date")]
        end: Option<String>,
        #[structopt(short = "c", long = "channel")]
        channel: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    match Cli::from_args() {
        Cli::Cost {
            start,
            end,
            channel,
        } => {
            let get_cost: GetCost<AWS, Slack> = UseCase::new().await;

            get_cost.run(start, end, channel).await;
        }
    }
}
