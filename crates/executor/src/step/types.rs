use derive_more::From;
use enum_dispatch::enum_dispatch;
use ethers::providers::Middleware;

use super::{CallContract, CallContractOutput};

#[derive(Debug)]
pub enum StepTypes {}

#[enum_dispatch(Step)]
pub enum CallContractStep<M>
where
    M: Middleware,
{
    CallContract(CallContract<M>),
}

#[derive(Debug, From)]
pub enum StepOutput {
    CallContractOutput(CallContractOutput),
    MockOutput(String),
}
