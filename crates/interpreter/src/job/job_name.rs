use derive_more::Deref;
use serde::Deserialize;
use serde_valid::Validate;

/// JobName can only contains alphanumeric, `_` or `-` characters, up to a maximum of 200 characters.
#[derive(Debug, Deref, Deserialize, Validate, Eq, PartialEq, Hash, Clone)]
pub struct JobName(#[validate(pattern = r#"^[a-zA-Z0-9][a-zA-Z0-9_-]{1,199}$"#)] String);

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn valid_job_names() -> impl Strategy<Value = String> {
        proptest::string::string_regex("[a-zA-Z0-9][a-zA-Z0-9_-]{1,199}").unwrap()
    }
    proptest! {

        #[test]
        fn can_parse_job_name(test_case in valid_job_names()) {

            let _job_name: JobName = serde_yaml::from_str(&test_case).unwrap();
        }
    }
}
