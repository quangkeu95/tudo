use derive_more::{Deref, Into};
use ethers::types::Bytes;
use serde::Deserialize;

#[derive(Debug, Deref, Into, Deserialize)]
pub struct FunctionSelector(Bytes);

#[cfg(test)]
mod tests {
    use super::FunctionSelector;
    use ethers::types::Bytes;
    use std::str::FromStr;

    #[test]
    fn can_parse_function_selector() {
        let function_selector: FunctionSelector = serde_yaml::from_str("0x1e3dd18b").unwrap();
        assert_eq!(
            Bytes::from(function_selector),
            Bytes::from_str("0x1e3dd18b").unwrap()
        );
    }
}
