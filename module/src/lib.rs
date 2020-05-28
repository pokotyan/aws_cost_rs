mod mock_slack_module;
mod slack_module;

pub mod slack {
    pub use crate::mock_slack_module::*;
    pub use crate::slack_module::*;
}
