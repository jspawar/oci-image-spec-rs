use serde::de::{Deserializer, Error, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct EnvVar {
    pub var_name: String,
    pub var_value: String,
}

impl Serialize for EnvVar {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{}={}", self.var_name, self.var_value))
    }
}

impl<'de> Deserialize<'de> for EnvVar {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_string(EnvVarVisitor{})
    }
}
struct EnvVarVisitor;
impl<'de> Visitor<'de> for EnvVarVisitor {
    type Value = EnvVar;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO: what do I put here?
        formatter.write_str("TODO: idk what I put here")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        let tokens = v.split("=").collect::<Vec<&str>>();
        if tokens.len() == 2 {
            Ok(EnvVar{
                var_name: tokens[0].to_string(),
                var_value: tokens[1].to_string(),
            })
        } else {
            Err(Error::custom::<&str>("invalid format for `Env` entry; should be: `VARNAME=VARVALUE`"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod json {
        use super::*;

        #[test]
        fn serializes_correctly() {
            let env_var = EnvVar {
                var_name: "FOO".to_string(),
                var_value: "BAR".to_string(),
            };
            let serialized = serde_json::to_string(&env_var).unwrap();
            assert_eq!(serialized, r#""FOO=BAR""#);
        }

        #[test]
        fn deserializes_correctly() {
            let raw = r#""VAR=VALUE""#;
            let env_var: EnvVar = serde_json::from_str(&raw).unwrap();
            assert_eq!(env_var.var_name, "VAR");
            assert_eq!(env_var.var_value, "VALUE");
        }

        mod with_bad_input {
            use super::*;

            #[test]
            fn deserializes_with_meaningful_error() {
                let raw = r#""FOO=BAR=BAZ""#;
                let result: Result<EnvVar, serde_json::error::Error> = serde_json::from_str(&raw);
                assert!(result.is_err());
                let err_string = result.err().unwrap().to_string();
                assert!(err_string.contains("invalid format for `Env` entry; should be: `VARNAME=VARVALUE`"));
            }
        }
    }
}
