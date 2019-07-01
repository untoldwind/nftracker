use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::net::IpAddr;

#[derive(Clone, Debug)]
pub enum Subnet {
    V4(Vec<u8>),
    V6(Vec<u16>),
}

impl Subnet {
    pub fn contains(&self, addr: &IpAddr) -> bool {
        match (self, addr) {
            (Subnet::V4(prefix), IpAddr::V4(v4_addr)) => {
                let octets = v4_addr.octets();
                for (i, p) in prefix.iter().enumerate() {
                    if octets[i] != *p {
                        return false
                    }
                }
                true
            },
            (Subnet::V6(prefix), IpAddr::V6(v6_addr)) => {
                let segments = v6_addr.segments();
                for (i, p) in prefix.iter().enumerate() {
                    if segments[i] != *p {
                        return false
                    }
                }
                true
            },
            _ => false,
        }
    }
}

impl Serialize for Subnet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            Subnet::V4(prefix) => prefix.iter().map(|p| format!("{}.", p)).collect::<String>(),
            Subnet::V6(prefix) => prefix
                .iter()
                .map(|p| format!("{:x}.", p))
                .collect::<String>(),
        };
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Subnet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(SubnetVisitor)
    }
}

struct SubnetVisitor;

impl<'de> de::Visitor<'de> for SubnetVisitor {
    type Value = Subnet;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "IP4 or IP6 subnet prefix")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok((_, subnet)) = super::parse::subnet::<()>(s) {
            Ok(subnet)
        } else {
            Err(de::Error::invalid_value(de::Unexpected::Str(s), &self))
        }
    }
}
