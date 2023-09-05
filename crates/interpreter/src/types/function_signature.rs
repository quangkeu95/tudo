use derive_more::{AsRef, From};
use ethers::types::Bytes;
use serde::Deserialize;

/// A Solidity function signature
///
/// # Example
/// ```rust
/// use tudo_interpreter::types::FunctionSignature;
/// use ethers::types::Bytes;
/// use std::str::FromStr;
///
/// let function_signature = FunctionSignature::from("allPairs(uint256)");
/// assert_eq!(
///     Bytes::from(function_signature.as_bytes()),
///     Bytes::from_str("0x1e3dd18b").unwrap()
/// );
/// ```
#[derive(Debug, Clone, Deserialize, From, AsRef)]
pub struct FunctionSignature(String);

impl FunctionSignature {
    /// Convert function signature to 4 bytes function selector
    pub fn as_bytes(&self) -> Bytes {
        Bytes::from(ethers::utils::id(self.0.as_str()))
    }
}

impl From<&str> for FunctionSignature {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::types::Bytes;
    use std::str::FromStr;

    #[test]
    fn can_convert_to_bytes() {
        let function_signature = FunctionSignature::from("allPairs(uint256)");
        assert_eq!(
            function_signature.as_bytes(),
            Bytes::from_str("0x1e3dd18b").unwrap()
        );
    }
}
