use std::{collections::HashMap, str::FromStr};

use serde::Deserialize;

use crate::types::VariableName;

use super::{RpcProvider, Variable};

/// Playbook setup that contains predefined variables
#[derive(Debug, Deserialize)]
pub struct Setup {
    pub variables: Option<HashMap<VariableName, Variable>>,
    pub rpc_providers: Option<HashMap<VariableName, RpcProvider>>,
}

impl Setup {
    /// Get variable value by it's name. Return None if the variable is not defined.
    pub fn get_variable<N>(&self, variable_name: N) -> Option<&Variable>
    where
        N: AsRef<str>,
    {
        if let Some(variables) = &self.variables {
            let variable_name = VariableName::from_str(variable_name.as_ref()).ok()?;

            variables.get(&variable_name)
        } else {
            None
        }
    }
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
