use derive_builder::Builder;
use ethers::abi::Token;
use ethers::prelude::Middleware;
use ethers::types::{BlockId, Bytes, TransactionRequest, H160};
use thiserror::Error;
use tudo_primitives::Step;

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
    type Output = Bytes;
    type Error = CallContractError<M>;

    async fn execute(&self, input: &Self::Input) -> Result<Self::Output, Self::Error> {
        let tx_request = TransactionRequest::new()
            .to(input.contract_address)
            .data(input.calldata.clone());
        let result = self
            .middleware
            .call(&tx_request.into(), input.block)
            .await
            .map_err(|e| CallContractError::MiddlewareError(e))?;
        Ok(result)
    }
}

#[derive(Debug, Builder)]
pub struct CallContractInput {
    pub contract_address: H160,
    pub calldata: Bytes,
    #[builder(default)]
    pub block: Option<BlockId>,
}

impl CallContractInput {
    /// Build raw calldata in [Bytes] from function signature and args
    ///
    /// # Example
    /// ```rust
    /// use tudo_executor::step::CallContractInput;
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

#[derive(Debug, Error)]
pub enum CallContractError<M>
where
    M: Middleware,
{
    #[error("Middleware error {0}")]
    MiddlewareError(M::Error),
}

#[cfg(test)]
mod tests {
    use claims::*;
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
    async fn can_execute_call_contract() {
        let rpc_url = "https://eth.llamarpc.com";
        let provider = Provider::try_from(rpc_url).unwrap();
        // uniswap v3 swap router 02
        let contract_address = "0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45"
            .parse::<H160>()
            .unwrap();

        let calldata = CallContractInput::build_calldata("WETH9()", &vec![]);

        let call_contract_step = CallContract::new(provider);

        let input = CallContractInputBuilder::default()
            .contract_address(contract_address)
            .calldata(calldata)
            .build()
            .unwrap();
        let return_data = call_contract_step.execute(&input).await.unwrap();

        let result = ethers::abi::decode(&vec![ParamType::Address], &return_data).unwrap();
        let result = result[0].to_owned().into_address();
        assert_some!(result);
        assert_eq!(
            result.unwrap(),
            "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
                .parse::<H160>()
                .unwrap()
        );
    }
}
