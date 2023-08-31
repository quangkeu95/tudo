use serde::{Deserialize, Serialize};

/// Step types that can be used in the playbook definition file
#[derive(Debug, Deserialize, Serialize, PartialEq, strum::Display)]
pub enum StepTypes {
    CallContract,
    MockStep,
}

#[cfg(test)]
mod tests {
    use super::StepTypes;

    #[test]
    fn can_deserialize_step_types() {
        let content = r#"
            - CallContract
        "#;

        let _step_types: Vec<StepTypes> = serde_yaml::from_str(&content).unwrap();
    }
}
