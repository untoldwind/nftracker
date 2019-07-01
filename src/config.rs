use crate::common::Subnet;
use log::error;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::{ErrorKind, Read, Result};
use std::path::Path;

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
