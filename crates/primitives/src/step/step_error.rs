use derive_more::Unwrap;
use thiserror::Error;

#[derive(Debug, Error, Unwrap)]
pub enum StepError {
    #[error("call contract error {:#?}", .0)]
    CallContractError(String),
    #[error("send transction error {:#?}", .0)]
    SendTransactionError(String),
}
