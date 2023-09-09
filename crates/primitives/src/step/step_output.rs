use crate::{CallContractOutput, SendTransactionOutput};
use derive_more::{From, Unwrap};

#[non_exhaustive]
#[derive(Debug, From, Unwrap, Clone)]
pub enum StepOutput {
    CallContractOutput(CallContractOutput),
    SendTransactionOutput(SendTransactionOutput),
    None,
}
