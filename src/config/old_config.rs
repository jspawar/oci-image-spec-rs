use std::io::Read;
use std::fs::{File};
use std::collections::HashMap;

use crate::config::exposed_ports::{ExposedPorts};

use serde::{Deserialize, Serialize};

use chrono::prelude::*;

// TODO: reorganize/split up this file, but in a way that makes sense for importing it too
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // required
    pub architecture: Architecture,
    pub os: OS,
    pub rootfs: ConfigRootFs,
    // optional
    pub created: Option<DateTime<Utc>>,
    pub author: Option<String>,
    pub config: Option<ConfigConfig>,
    pub history: Option<Vec<ConfigHistory>>,
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
pub struct ConfigRootFs {
    #[serde(rename = "type")]
    pub _type: RootFsType,
    // TODO: change this to some sort of type that is basically: `<hash_alg>:<hash>`
    pub diff_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigConfig {
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
pub struct ConfigHistory {
  pub created: Option<DateTime<Utc>>,
  pub author: Option<String>,
  pub created_by: Option<String>,
  pub comment: Option<String>,
  pub empty_layer: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RootFsType {
  Layers,
}

#[derive(Debug)]
pub enum ParseError {
  IOError(std::io::Error),
  SerdeError(serde_json::error::Error),
}
impl From<std::io::Error> for ParseError {
  fn from(error: std::io::Error) -> Self {
    ParseError::IOError(error)
  }
}
impl From<serde_json::error::Error> for ParseError {
  fn from(error: serde_json::error::Error) -> Self {
    ParseError::SerdeError(error)
  }
}

pub fn parse_v1_config_file(file: &mut File) -> Result<Config, ParseError> {
  let mut raw = String::new();
  file.read_to_string(&mut raw)?;

  let config: Config = serde_json::from_str(&raw)?;
  Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: move this somewhere else?
    mod test_helpers {
        use super::*;
        use std::io::{Seek, Write};
        use std::fs::OpenOptions;

        pub fn assert_map_len<K, V>(map: &HashMap<K, V>, expected: usize) {
          assert_eq!(map.len(), expected);
        }

        // TODO: pass in `K` or `&K`?
        // TODO: pass in `V` or `&V`?
        pub fn assert_map_contains<K, V>(map: &HashMap<K, V>, key: K, val: V)
          where K: std::cmp::Eq + std::hash::Hash,
                V: std::cmp::PartialEq + std::fmt::Debug,
        {
          assert_eq!(map.contains_key(&key), true);
          assert_eq!(map[&key], val);
        }

        // TODO: return ref to file?
        pub fn create_temp_file(name: &'static str) -> File {
            let mut tmp_path = std::env::temp_dir();
            tmp_path.push("oci-image-spec-rs-tests");
            std::fs::create_dir_all(&tmp_path).unwrap();
            tmp_path.push(name);

            OpenOptions::new()
              .read(true)
              .write(true)
              .create(true)
              .open(tmp_path)
              .unwrap()
        }

        // TODO: return ref to file?
        pub fn create_temp_config_file(name: &'static str, contents: &[u8]) -> File {
          let mut cfg_file = create_temp_file(name);
          cfg_file.write_all(contents).unwrap();
          cfg_file.seek(std::io::SeekFrom::Start(0)).unwrap();
          cfg_file
        }
    }

    mod with_only_required_properties {
        use super::*;

        #[test]
        fn serializes_correctly() {
            let config = Config {
                architecture: Architecture::_386,
                os: OS::Linux,
                rootfs: ConfigRootFs {
                    _type: RootFsType::Layers,
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
            let mut cfg_file = test_helpers::create_temp_config_file("config.json", br#"{
  "architecture": "386",
  "os": "linux",
  "rootfs": {
    "type": "layers",
    "diff_ids": [
      "sha256:bogus-sha"
    ]
  }
}"#);
            let deserialized = parse_v1_config_file(&mut cfg_file).unwrap();

            match deserialized.architecture {
              Architecture::_386 => {}
              _ => {panic!("Received unexpected architecture: {:?}", deserialized.architecture)}
            }
            match deserialized.os {
              OS::Linux => {}
              _ => {panic!("Received unexpected OS: {:?}", deserialized.os)}
            }
            match deserialized.rootfs._type {
              RootFsType::Layers => {}
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
        use crate::config::exposed_ports::{PortProtocol};

        #[test]
        fn serializes_correctly() {
            let timestamp = Utc::now();
            let mut port_protocol_map = HashMap::new();
            port_protocol_map.insert(8080, Some(PortProtocol::TCP));
            let mut labels = HashMap::new();
            labels.insert("bar.foo".to_string(), "this is a label".to_string());

            let config = Config {
                architecture: Architecture::_386,
                os: OS::Linux,
                rootfs: ConfigRootFs {
                    _type: RootFsType::Layers,
                    diff_ids: vec!["sha256:some-sha".to_string()],
                },
                created: Some(timestamp),
                author: Some("Some One <someone@some.where>".to_string()),
                config: Some(ConfigConfig{
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
                history: Some(vec![ConfigHistory{
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
            let mut cfg_file = test_helpers::create_temp_config_file("config.json", br#"{
  "architecture": "386",
  "os": "linux",
  "rootfs": {
    "type": "layers",
    "diff_ids": [
      "sha256:bogus-sha"
    ]
  }
}"#);
            let deserialized = parse_v1_config_file(&mut cfg_file).unwrap();

            match deserialized.architecture {
              Architecture::_386 => {}
              _ => {panic!("Received unexpected architecture: {:?}", deserialized.architecture)}
            }
            match deserialized.os {
              OS::Linux => {}
              _ => {panic!("Received unexpected OS: {:?}", deserialized.os)}
            }
            match deserialized.rootfs._type {
              RootFsType::Layers => {}
            }
            assert_eq!(deserialized.rootfs.diff_ids.len(), 1);
            assert_eq!(deserialized.rootfs.diff_ids[0], "sha256:bogus-sha");
        }
    }
}
