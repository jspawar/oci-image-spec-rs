use std::io::Read;
use std::fs::{File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // required
    pub architecture: Architecture,
    pub os: OS,
    pub rootfs: ConfigRootFs,
    // optional
    pub created: Option<String>,
    pub author: Option<String>,
    pub config: Option<String>,
    pub history: Option<String>,
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
    pub diff_ids: Vec<String>,
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
  let num_read = file.read_to_string(&mut raw)?;
  println!("received the following number of bytes: {}", num_read);
  println!("received the following file contents: {:?}", raw);

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
            assert_eq!(
                serialized,
                r#"{
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
}"#
            );
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
            assert_eq!(
                serialized,
                r#"{
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
}"#
            );
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
