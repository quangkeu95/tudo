use ethers::abi::Token;
use ethers::types::Bytes;

/// Build raw calldata in [`Bytes`] from function signature and args
///
/// # Example
/// ```rust
/// use tudo_primitives::utils::*;
/// use ethers::abi::Token;
/// use ethers::types::{Bytes, H160, U256};
/// use std::str::FromStr;
///
/// let usdt = H160::repeat_byte(1);
/// let uni = H160::repeat_byte(2);
/// let sender = H160::repeat_byte(3);
/// let calldata = build_calldata(
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
