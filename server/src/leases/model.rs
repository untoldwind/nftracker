use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct Lease {
    pub name: String,
    pub addr: IpAddr,
    pub client_id: String,
}
