use std::collections::HashMap;

use serde::Deserialize;

use crate::types::VariableName;

use super::{RpcProvider, Variable};

/// Playbook setup that contains predefined variables
#[derive(Debug, Deserialize)]
pub struct Setup {
    pub variables: Option<HashMap<VariableName, Variable>>,
    pub rpc_providers: Option<HashMap<VariableName, RpcProvider>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn can_deserialize_setup_variables() {
        env::set_var("SECRET_ENV", "A secret");
        env::set_var("KEY_1", "value 1");
        let yaml = r#"
            variables:
                SECRET: ${SECRET_ENV}
                KEY_1: A key with env value ${KEY_1}
        "#;

        let _setup: Setup = serde_yaml::from_str(yaml).unwrap();
    }

    #[test]
    fn can_deserialize_rpc_providers() {
        let yaml = r#"
            rpc_providers:
                PROVIDER_1: 
                    chain_rpc_url: "https://eth.llamarpc.com"
                PROVIDER_2:
                    chain_rpc_url: "https://eth.llamarpc.com"
                    provider_type: HttpWithBasicAuth
                    username: test
                    password: test
        "#;

        let _setup: Setup = serde_yaml::from_str(yaml).unwrap();
    }
}
