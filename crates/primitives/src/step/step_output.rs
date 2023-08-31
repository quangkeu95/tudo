use crate::{CallContractOutput, SendTransactionOutput};
use derive_more::{From, Unwrap};

#[derive(Debug, From, Unwrap)]
pub enum StepOutput {
    CallContractOutput(CallContractOutput),
    SendTransactionOutput(SendTransactionOutput),
    None,
}