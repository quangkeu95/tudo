use alloy_primitives::{Address, Bytes, FixedBytes, I256, U256};
use ethers::types::H160;

/// Used to convert Alloy primitives types to ethers-rs types
pub struct AlloyConverter {}

impl AlloyConverter {
    /// Convert from alloy [`Address`] into ethers-rs [`H160`]
    pub fn from_alloy_address(address: &Address) -> H160 {
        let bytes = address.clone().into_array();

        H160::from(bytes)
    }

    /// Convert from alloy [`U256`] into ethers-rs [`U256`].
    pub fn from_alloy_uint(uint: &U256, _size: usize) -> ethers::types::U256 {
        let uint_str = uint.to_string();
        ethers::types::U256::from_dec_str(&uint_str).unwrap()
    }

    /// Convert from alloy [`I256`] into ethers-rs [`I256`]
    pub fn from_alloy_int(alloy_value: &I256, _size: usize) -> ethers::types::I256 {
        let int_str = alloy_value.to_string();
        ethers::types::I256::from_dec_str(&int_str).unwrap()
    }

    /// Convert from alloy [`Bytes`] into ethers-rs [`Bytes`]
    pub fn from_alloy_bytes(bytes: &Bytes) -> ethers::types::Bytes {
        let bytes_vec = bytes.0.to_vec();
        ethers::types::Bytes::from(bytes_vec)
    }

    pub fn from_alloy_fixed_bytes<const N: usize>(bytes: &FixedBytes<N>) -> ethers::types::Bytes {
        let bytes_vec = bytes.0.to_vec();
        ethers::types::Bytes::from(bytes_vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::U256;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn can_convert_from_alloy_uint(alloy_uint in U256::arbitrary()) {
            let ethers_uint = AlloyConverter::from_alloy_uint(&alloy_uint, U256::BITS);
            assert_eq!(alloy_uint.to_string(), ethers_uint.to_string());
        }
        #[test]
        fn can_convert_from_alloy_int(alloy_int in I256::arbitrary()) {
            let ethers_int = AlloyConverter::from_alloy_int(&alloy_int, I256::BITS);
            assert_eq!(alloy_int.to_string(), ethers_int.to_string());
        }
        #[test]
        fn can_convert_from_alloy_bytes(alloy_bytes in Bytes::arbitrary()) {
            let ethers_bytes = AlloyConverter::from_alloy_bytes(&alloy_bytes);
            assert_eq!(alloy_bytes.to_string(), ethers_bytes.to_string());
        }

        #[test]
        fn can_convert_from_alloy_fixed_bytes(alloy_bytes in FixedBytes::<64>::arbitrary()) {
            let ethers_bytes = AlloyConverter::from_alloy_fixed_bytes(&alloy_bytes);
            assert_eq!(alloy_bytes.to_string(), ethers_bytes.to_string());
        }

    }
}
