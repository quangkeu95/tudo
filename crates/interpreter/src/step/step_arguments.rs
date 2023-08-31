use serde::Deserialize;

mod call_contract;
pub use call_contract::*;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum StepArguments {
    CallContract(CallContract),
}
