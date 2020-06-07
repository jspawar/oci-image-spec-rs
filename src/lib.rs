mod config;

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

    let config_type_name = std::any::type_name::<Config>();
    assert!(config_type_name.contains(&CRATE_NAME));
  }
}
