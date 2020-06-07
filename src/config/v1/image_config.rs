use std::io::Read;
use std::fs::{File};
use std::collections::HashMap;

use crate::config::v1::exposed_ports::{ExposedPorts};
use crate::config::v1::errors::{ParseError};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

// TODO: reorganize/split up this file, but in a way that makes sense for importing it too
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageConfig {
    // required
    pub architecture: Architecture,
    pub os: OS,
    pub rootfs: RootFS,
    // optional
    pub created: Option<DateTime<Utc>>,
    pub author: Option<String>,
    pub config: Option<Config>,
    pub history: Option<Vec<History>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Architecture {
    #[serde(rename = "386")]
    _386,
    Amd64,
    Arm,
    Arm64,
    Mips,
    Mips64,
    Mips64le,
    Mipsle,
    Ppc64,
    Ppc64le,
    S390x,
    Wasm,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OS {
    Aix,
    Android,
    Darwin,
    Dragonfly,
    Freebsd,
    Illumos,
    Js,
    Linux,
    Netbsd,
    Openbsd,
    Plan9,
    Solaris,
    Windows,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootFS {
    #[serde(rename = "type")]
    pub _type: RootFSType,
    // TODO: change this to some sort of type that is basically: `<hash_alg>:<hash>`
    pub diff_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  // TODO: make a struct for `user` like for `ExposedPorts`?
  // pub user: Option<String>,
  pub exposed_ports: Option<ExposedPorts>,
  // pub env: Option<Vec<EnvVar>,
  pub entrypoint: Option<Vec<String>>,
  pub cmd: Option<Vec<String>>,
  // pub volumes: Option<Volumes>,
  pub working_dir: Option<String>,
  pub labels: Option<HashMap<String, String>>,
  // pub stop_signal: Option<OsSignal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct History {
  pub created: Option<DateTime<Utc>>,
  pub author: Option<String>,
  pub created_by: Option<String>,
  pub comment: Option<String>,
  pub empty_layer: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RootFSType {
  Layers,
}

pub fn parse_image_config_file(file: &mut File) -> Result<ImageConfig, ParseError> {
  let mut raw = String::new();
  file.read_to_string(&mut raw)?;

  let config: ImageConfig = serde_json::from_str(&raw)?;
  Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::utils::*;

    mod with_only_required_properties {
        use super::*;

        #[test]
        fn serializes_correctly() {
            let config = ImageConfig {
                architecture: Architecture::_386,
                os: OS::Linux,
                rootfs: RootFS {
                    _type: RootFSType::Layers,
                    diff_ids: vec![],
                },
                created: None,
                author: None,
                config: None,
                history: None,
            };
            let serialized = serde_json::to_string_pretty(&config).unwrap();
            assert_eq!(serialized, r#"{
  "architecture": "386",
  "os": "linux",
  "rootfs": {
    "type": "layers",
    "diff_ids": []
  },
  "created": null,
  "author": null,
  "config": null,
  "history": null
}"#);
        }

        #[test]
        fn parses_correctly() {
            let mut cfg_file = create_temp_file_with_contents("config.json", br#"{
  "architecture": "386",
  "os": "linux",
  "rootfs": {
    "type": "layers",
    "diff_ids": [
      "sha256:bogus-sha"
    ]
  }
}"#);
            let deserialized = parse_image_config_file(&mut cfg_file).unwrap();

            match deserialized.architecture {
              Architecture::_386 => {}
              _ => {panic!("Received unexpected architecture: {:?}", deserialized.architecture)}
            }
            match deserialized.os {
              OS::Linux => {}
              _ => {panic!("Received unexpected OS: {:?}", deserialized.os)}
            }
            match deserialized.rootfs._type {
              RootFSType::Layers => {}
            }
            assert_eq!(deserialized.rootfs.diff_ids.len(), 1);
            assert_eq!(deserialized.rootfs.diff_ids[0], "sha256:bogus-sha");
        }

        #[test]
        fn allows_only_valid_platform_combinations() {
            // TODO: make this test using validator from spec repo as guidance
        }
    }

    mod with_all_optional_properties {
        use super::*;
        use crate::config::v1::exposed_ports::{PortProtocol};

        #[test]
        fn serializes_correctly() {
            let timestamp = Utc::now();
            let mut port_protocol_map = HashMap::new();
            port_protocol_map.insert(8080, Some(PortProtocol::TCP));
            let mut labels = HashMap::new();
            labels.insert("bar.foo".to_string(), "this is a label".to_string());

            let config = ImageConfig {
                architecture: Architecture::_386,
                os: OS::Linux,
                rootfs: RootFS {
                    _type: RootFSType::Layers,
                    diff_ids: vec!["sha256:some-sha".to_string()],
                },
                created: Some(timestamp),
                author: Some("Some One <someone@some.where>".to_string()),
                config: Some(Config{
                  // user: Some(String::from("user")),
                  exposed_ports: Some(ExposedPorts{
                    port_protocol_map: port_protocol_map,
                  }),
                  entrypoint: Some(vec!["/bin/sh".to_string()]),
                  cmd: Some(vec![
                    "-c".to_string(),
                    "echo hello".to_string(),
                  ]),
                  working_dir: Some("/home".to_string()),
                  labels: Some(labels),
                }),
                history: Some(vec![History{
                  created: Some(timestamp),
                  author: Some("Some One <someone@some.where>".to_string()),
                  created_by: Some("/bin/sh".to_string()),
                  comment: Some("this is a comment".to_string()),
                  empty_layer: Some(false),
                }]),
            };

            let serialized = serde_json::to_string_pretty(&config).unwrap();
            let timestamp_str = timestamp.to_rfc3339_opts(SecondsFormat::Micros, true);
            assert_eq!(serialized, format!(r#"{{
  "architecture": "386",
  "os": "linux",
  "rootfs": {{
    "type": "layers",
    "diff_ids": []
  }},
  "created": "{}",
  "author": "Some One <someone@some.where>",
  "config": {{
    "User": "user",
    "ExposedPorts": {{
      "8080/tcp": {{}}
    }},
    "Env": [
      "FOO=BAR"
    ],
    "Entrypoint": [
      "/bin/sh"
    ],
    "Cmd": [
      "-c",
      "echo hello"
    ],
    "Volumes": {{
      "/tmp/foobar": {{}}
    }},
    "WorkingDir": "/home",
    "Labels": {{
      "bar.foo": "this is a label"
    }}
  }},
  "history": [
    {{
      "created": "{}",
      "author": "Some One <someone@some.where>",
      "created_by": "/bin/sh",
      "comment": "this is a comment",
      "empty_layer": false
    }}
  ]
}}"#, timestamp_str, timestamp_str));
        }

        #[test]
        fn parses_correctly() {
            let mut cfg_file = create_temp_file_with_contents("config.json", br#"{
  "architecture": "386",
  "os": "linux",
  "rootfs": {
    "type": "layers",
    "diff_ids": [
      "sha256:bogus-sha"
    ]
  }
}"#);
            let deserialized = parse_image_config_file(&mut cfg_file).unwrap();

            match deserialized.architecture {
              Architecture::_386 => {}
              _ => {panic!("Received unexpected architecture: {:?}", deserialized.architecture)}
            }
            match deserialized.os {
              OS::Linux => {}
              _ => {panic!("Received unexpected OS: {:?}", deserialized.os)}
            }
            match deserialized.rootfs._type {
              RootFSType::Layers => {}
            }
            assert_eq!(deserialized.rootfs.diff_ids.len(), 1);
            assert_eq!(deserialized.rootfs.diff_ids[0], "sha256:bogus-sha");
        }
    }
}
