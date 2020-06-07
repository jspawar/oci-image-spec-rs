mod config;

#[cfg(test)]
mod test_helpers;

pub use config::*;

#[cfg(test)]
mod tests {
  use super::*;

  const CRATE_NAME: &'static str = "oci_image_spec_rs";

  #[test]
  fn test_exports_visibility() {
    // tested splitting apart large `config.rs` file by first creating `EnvVar` type in its own
    // module/file
    // TODO: remove this explanation once refactoring is complete?
    let env_var_type_name = std::any::type_name::<EnvVar>();
    assert!(env_var_type_name.contains(&CRATE_NAME));

    let port_protocol_type_name = std::any::type_name::<PortProtocol>();
    assert!(port_protocol_type_name.contains(&CRATE_NAME));
    let exposed_ports_type_name = std::any::type_name::<ExposedPorts>();
    assert!(exposed_ports_type_name.contains(&CRATE_NAME));

    let parse_error_type_name = std::any::type_name::<ParseError>();
    assert!(parse_error_type_name.contains(&CRATE_NAME));

    let architecture_type_name = std::any::type_name::<Architecture>();
    assert!(architecture_type_name.contains(&CRATE_NAME));
    let os_type_name = std::any::type_name::<OS>();
    assert!(os_type_name.contains(&CRATE_NAME));
    let config_root_fs_type_name = std::any::type_name::<ConfigRootFs>();
    assert!(config_root_fs_type_name.contains(&CRATE_NAME));
    let config_config_type_name = std::any::type_name::<ConfigConfig>();
    assert!(config_config_type_name.contains(&CRATE_NAME));
    let config_history_type_name = std::any::type_name::<ConfigHistory>();
    assert!(config_history_type_name.contains(&CRATE_NAME));

    let root_fs_type_type_name = std::any::type_name::<RootFsType>();
    assert!(root_fs_type_type_name.contains(&CRATE_NAME));

    let config_type_name = std::any::type_name::<Config>();
    assert!(config_type_name.contains(&CRATE_NAME));
  }
}
