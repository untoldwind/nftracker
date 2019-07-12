use crate::common::Subnet;
use log::error;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::{ErrorKind, Read, Result};
use std::path::Path;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub local_subnets: Vec<Subnet>,
    pub wan_interface: String,
    #[serde(default = "default_conntrack_file")]
    pub conntrack_file: String,
    #[serde(default = "default_device_file")]
    pub device_file: String,
    #[serde(default = "default_lease_file")]
    pub leases_file: String,
    #[serde(default = "default_retain_data", with = "humantime_serde")]
    pub retain_data: Duration,
}

fn default_device_file() -> String {
    "/proc/net/dev".to_string()
}

fn default_conntrack_file() -> String {
    "/proc/net/nf_conntrack".to_string()
}

fn default_lease_file() -> String {
    "/var/lib/misc/dnsmasq.leases".to_string()
}

fn default_retain_data() -> Duration {
    Duration::from_secs(300)
}

impl Config {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Config> {
        let mut file = File::open(path)?;
        let mut config_toml = String::new();
        file.read_to_string(&mut config_toml)?;

        match toml::from_str::<Config>(&config_toml) {
            Ok(config) => Ok(config),
            Err(error) => {
                error!("Config file: {}", error);
                Err(ErrorKind::InvalidData.into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Subnet;
    use spectral::prelude::*;
    use std::time::Duration;

    #[test]
    fn test_decode_config_simple() {
        let simple = r#"
            local_subnets = ["192.168.3.", "1234::"]
            wan_interface = "eth0"
        "#;

        let config = toml::from_str::<Config>(simple).unwrap();

        assert_that(&config.local_subnets).is_equal_to(vec![
            Subnet::V4(vec![192, 168, 3]),
            Subnet::V6(vec![0x1234]),
        ]);
        assert_that(&config.wan_interface).is_equal_to("eth0".to_string());
        assert_that(&config.conntrack_file).is_equal_to("/proc/net/nf_conntrack".to_string());
        assert_that(&config.device_file).is_equal_to("/proc/net/dev".to_string());
        assert_that(&config.leases_file).is_equal_to("/var/lib/misc/dnsmasq.leases".to_string());
        assert_that(&config.retain_data).is_equal_to(Duration::from_secs(300));
    }

    #[test]
    fn test_decode_config_full() {
        let full = r#"
            local_subnets = ["192.168.3.", "1234::"]
            wan_interface = "eth0"
            conntrack_file = "/da/conntrack"
            device_file = "/da/device"
            leases_file = "/da/leases"
            retain_data = "10m"
        "#;

        let config = toml::from_str::<Config>(full).unwrap();

        assert_that(&config.local_subnets).is_equal_to(vec![
            Subnet::V4(vec![192, 168, 3]),
            Subnet::V6(vec![0x1234]),
        ]);
        assert_that(&config.wan_interface).is_equal_to("eth0".to_string());
        assert_that(&config.conntrack_file).is_equal_to("/da/conntrack".to_string());
        assert_that(&config.device_file).is_equal_to("/da/device".to_string());
        assert_that(&config.leases_file).is_equal_to("/da/leases".to_string());
        assert_that(&config.retain_data).is_equal_to(Duration::from_secs(600));
    }
}
