use super::Step;
use derive_builder::Builder;
use ethers::abi::{ParamType, Token};
use ethers::prelude::Middleware;
use ethers::types::{BlockId, Bytes, TransactionRequest, H160};
use thiserror::Error;

/// CallContract which implements [`Step`] trait to call contract and return [`Bytes`] data or ABI decoded return data.
#[derive(Debug)]
pub struct CallContract<M>
where
    M: Middleware,
{
    pub middleware: M,
}

impl<M> CallContract<M>
where
    M: Middleware,
{
    /// Constructor to create [`CallContract`] instance
    pub fn new(middleware: M) -> Self {
        Self { middleware }
    }
}

#[async_trait::async_trait]
impl<M> Step for CallContract<M>
where
    M: Middleware,
{
    type Input = CallContractInput;
    type Output = CallContractOutput;
    type Error = CallContractError<M>;

    async fn execute(&self, input: &mut Self::Input) -> Result<Self::Output, Self::Error> {
        let tx_request = TransactionRequest::new()
            .to(input.contract_address)
            .data(input.calldata.clone());
        let bytes_result = self
            .middleware
            .call(&tx_request.into(), input.block)
            .await
            .map_err(|e| CallContractError::MiddlewareError(e))?;

        if let Some(return_data_types) = &input.return_data_types {
            let decoded_return_data = ethers::abi::decode(return_data_types, &bytes_result)?;
            Ok(CallContractOutput::Tokens(decoded_return_data))
        } else {
            Ok(CallContractOutput::Bytes(bytes_result))
        }
    }
}

/// CallContract input parameters to execute call to read value from a contract
#[derive(Debug, Builder)]
pub struct CallContractInput {
    pub contract_address: H160,
    pub calldata: Bytes,
    #[builder(default)]
    pub block: Option<BlockId>,
    #[builder(default)]
    pub return_data_types: Option<Vec<ParamType>>,
}

impl CallContractInput {
    /// Build raw calldata in [`Bytes`] from function signature and args
    ///
    /// # Example
    /// ```rust
    /// use tudo_primitives::step::CallContractInput;
    /// use ethers::abi::Token;
    /// use ethers::types::{Bytes, H160, U256};
    /// use std::str::FromStr;
    ///
    /// let usdt = H160::repeat_byte(1);
    /// let uni = H160::repeat_byte(2);
    /// let sender = H160::repeat_byte(3);
    /// let calldata = CallContractInput::build_calldata(
    /// "swapExactTokensForTokens(uint256,uint256,address[],address)",
    ///     &vec![
    ///         Token::Uint(U256::from(100000)),
    ///         Token::Uint(U256::from(99000)),
    ///         Token::Array(vec![Token::Address(usdt), Token::Address(uni)]),
    ///         Token::Address(sender),
    ///     ],
    /// );
    /// assert_eq!(calldata, Bytes::from_str("0x472b43f300000000000000000000000000000000000000000000000000000000000186a000000000000000000000000000000000000000000000000000000000000182b800000000000000000000000000000000000000000000000000000000000000800000000000000000000000000303030303030303030303030303030303030303000000000000000000000000000000000000000000000000000000000000000200000000000000000000000001010101010101010101010101010101010101010000000000000000000000000202020202020202020202020202020202020202").unwrap());
    /// ```
    pub fn build_calldata<S>(function_signature: S, args: &[Token]) -> Bytes
    where
        S: AsRef<str>,
    {
        let function_selector = ethers::utils::id(function_signature);
        let encoded_args = ethers::abi::encode(args);
        let call_data = [Bytes::from(function_selector), Bytes::from(encoded_args)].concat();
        Bytes::from(call_data)
    }
}

/// CallContract output which in [`Bytes`] or [`Vec<Token>`]
#[derive(Debug)]
pub enum CallContractOutput {
    Bytes(Bytes),
    Tokens(Vec<Token>),
}

/// CallContract error
#[derive(Debug, Error)]
pub enum CallContractError<M>
where
    M: Middleware,
{
    #[error("Middleware error {0}")]
    MiddlewareError(M::Error),
    #[error(transparent)]
    AbiError(#[from] ethers::abi::Error),
}

#[cfg(test)]
mod tests {
    use ethers::{
        abi::ParamType,
        prelude::*,
        providers::Provider,
        types::{Bytes, H160},
    };
    use std::str::FromStr;

    use super::*;

    #[test]
    fn can_build_calldata() {
        let usdt = H160::repeat_byte(1);
        let uni = H160::repeat_byte(2);
        let sender = H160::repeat_byte(3);
        let calldata = CallContractInput::build_calldata(
            "swapExactTokensForTokens(uint256,uint256,address[],address)",
            &vec![
                Token::Uint(U256::from(100000)),
                Token::Uint(U256::from(99000)),
                Token::Array(vec![Token::Address(usdt), Token::Address(uni)]),
                Token::Address(sender),
            ],
        );
        assert_eq!(calldata, Bytes::from_str("0x472b43f300000000000000000000000000000000000000000000000000000000000186a000000000000000000000000000000000000000000000000000000000000182b800000000000000000000000000000000000000000000000000000000000000800000000000000000000000000303030303030303030303030303030303030303000000000000000000000000000000000000000000000000000000000000000200000000000000000000000001010101010101010101010101010101010101010000000000000000000000000202020202020202020202020202020202020202").unwrap());
    }

    #[tokio::test]
    async fn can_execute_call_contract_and_return_bytes() {
        let rpc_url = "https://eth.llamarpc.com";
        let provider = Provider::try_from(rpc_url).unwrap();
        // uniswap v3 swap router 02
        let contract_address = "0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45"
            .parse::<H160>()
            .unwrap();

        let calldata = CallContractInput::build_calldata("WETH9()", &vec![]);

        let call_contract_step = CallContract::new(provider);

        let mut input = CallContractInputBuilder::default()
            .contract_address(contract_address)
            .calldata(calldata)
            .build()
            .unwrap();
        let return_data = call_contract_step.execute(&mut input).await.unwrap();

        match return_data {
            CallContractOutput::Bytes(b) => {
                assert_eq!(
                    b,
                    Bytes::from_str(
                        "0x000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
                    )
                    .unwrap()
                );
            }
            _ => {
                panic!("invalid return data");
            }
        }
    }

    #[tokio::test]
    async fn can_execute_call_contract_and_decode_return_data() {
        let rpc_url = "https://eth.llamarpc.com";
        let provider = Provider::try_from(rpc_url).unwrap();
        // uniswap v3 swap router 02
        let contract_address = "0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45"
            .parse::<H160>()
            .unwrap();

        let calldata = CallContractInput::build_calldata("WETH9()", &vec![]);

        let call_contract_step = CallContract::new(provider);

        let mut input = CallContractInputBuilder::default()
            .contract_address(contract_address)
            .calldata(calldata)
            .return_data_types(Some(vec![ParamType::Address]))
            .build()
            .unwrap();
        let return_data = call_contract_step.execute(&mut input).await.unwrap();

        match return_data {
            CallContractOutput::Tokens(tokens) => {
                let expected_data = vec![Token::Address(
                    "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
                        .parse::<H160>()
                        .unwrap(),
                )];

                assert_eq!(tokens, expected_data);
            }
            _ => {
                panic!("invalid return data");
            }
        }
    }
}
