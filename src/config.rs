use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    local_subnets: Vec<String>,
    wan_interface: String,
}
