mod slack_mock_module;
mod slack_module;

pub mod slack {
    pub use crate::slack_mock_module::*;
    pub use crate::slack_module::*;
}
