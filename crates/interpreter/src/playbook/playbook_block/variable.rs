use derive_more::Into;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;

lazy_static! {
    static ref ENV_REGEX: Regex = Regex::new(r"\$\{([^}]*)\}").unwrap();
}

/// A Variable that value is a String and can contains environment variables in format `${ENV_VAR}`
#[derive(Debug, Into)]
pub struct Variable(String);

impl<'de> Deserialize<'de> for Variable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut variable_str = String::deserialize(deserializer)?;
        let mut env_map = HashMap::<String, String>::new();

        for word in ENV_REGEX.captures_iter(&variable_str) {
            let env_var_name = &word[1];
            let env_var = std::env::var(env_var_name).map_err(serde::de::Error::custom)?;
            env_map.insert(format!("${{{}}}", env_var_name), env_var);
        }

        for (env_var_name, env_value) in env_map.iter() {
            variable_str = variable_str.replace(env_var_name, env_value);
        }

        variable_str = variable_str
            .replace(r#"\$"#, "$")
            .replace(r#"\{"#, "{")
            .replace(r#"\}"#, "}");

        Ok(Self(variable_str))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;

    #[test]
    fn can_deserialize_variable() {
        env::set_var("KEY_ONE", "VALUE_1");
        env::set_var("KEY_$_2", "VALUE_2");

        let yaml =
            r#"i wanna read env vars ${KEY_ONE} and ${KEY_$_2}, not \$\{KEY_3\} or \$\{KEY_4\}"#;
        let expected = r#"i wanna read env vars VALUE_1 and VALUE_2, not ${KEY_3} or ${KEY_4}"#;

        let actual: Variable = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(expected, Into::<String>::into(actual));
    }

    #[test]
    fn can_deserialize_env_variable_only() {
        env::set_var(
            "SECRET",
            r"PM4tV*WefJAjKwxvBEZw&22vnKG#$RUFZWY!MrzKQ8MWVqomBP&6Pff%SVs82WK",
        );

        let yaml = r#"${SECRET}"#;
        let expected = r#"PM4tV*WefJAjKwxvBEZw&22vnKG#$RUFZWY!MrzKQ8MWVqomBP&6Pff%SVs82WK"#;
        let actual: Variable = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(expected, Into::<String>::into(actual));
    }
}
