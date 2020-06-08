mod config;

#[cfg(test)]
mod test_helpers;

pub use config::v1;

#[cfg(test)]
mod tests {
    use super::*;

    const CRATE_NAME: &'static str = "oci_image_spec_rs";

    #[test]
    fn test_exports_visibility() {
        let env_var_type_name = std::any::type_name::<v1::EnvVar>();
        assert!(env_var_type_name.contains(&CRATE_NAME));

        let port_protocol_type_name = std::any::type_name::<v1::PortProtocol>();
        assert!(port_protocol_type_name.contains(&CRATE_NAME));
        let exposed_ports_type_name = std::any::type_name::<v1::ExposedPorts>();
        assert!(exposed_ports_type_name.contains(&CRATE_NAME));

        let parse_error_type_name = std::any::type_name::<v1::ParseError>();
        assert!(parse_error_type_name.contains(&CRATE_NAME));

        let architecture_type_name = std::any::type_name::<v1::Architecture>();
        assert!(architecture_type_name.contains(&CRATE_NAME));
        let os_type_name = std::any::type_name::<v1::OS>();
        assert!(os_type_name.contains(&CRATE_NAME));
        let root_fs_type_name = std::any::type_name::<v1::RootFS>();
        assert!(root_fs_type_name.contains(&CRATE_NAME));
        let config_type_name = std::any::type_name::<v1::Config>();
        assert!(config_type_name.contains(&CRATE_NAME));
        let history_type_name = std::any::type_name::<v1::History>();
        assert!(history_type_name.contains(&CRATE_NAME));

        let root_fs_type_type_name = std::any::type_name::<v1::RootFSType>();
        assert!(root_fs_type_type_name.contains(&CRATE_NAME));
        let volumes_root_fs_type_name = std::any::type_name::<v1::Volumes>();
        assert!(volumes_root_fs_type_name.contains(&CRATE_NAME));

        let image_config_type_name = std::any::type_name::<v1::ImageConfig>();
        assert!(image_config_type_name.contains(&CRATE_NAME));
    }
}
