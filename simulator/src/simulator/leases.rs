use super::ConntrackSimulator;
use std::fs::File;
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::collections::HashMap;

pub struct LeasesSimulator<'a> {
    file: &'a str,
}

impl<'a> LeasesSimulator<'a> {
    pub fn new(file: &'a str) -> LeasesSimulator<'a> {
        LeasesSimulator { file }
    }

    pub fn dump(&self, conntrack_simulator: &ConntrackSimulator) -> io::Result<()> {
        let mut f = File::create(self.file)?;
        let mut ipv4_map : HashMap<String, Ipv4Addr> = Default::default();
        let mut ipv6_map : HashMap<String, Ipv6Addr> = Default::default();
        
        for target in conntrack_simulator.targets() {
            match target.source_ip_addr {
                IpAddr::V4(ipv4) => {
                    ipv4_map.insert(target.source_hostname.clone(), ipv4);
                }
                IpAddr::V6(ipv6) => {
                    ipv6_map.insert(target.source_hostname.clone(), ipv6);
                }
            }
        }

        for (hostname, ipv4) in ipv4_map {
            writeln!(f, "1562986769 74:c2:46:12:34:56 {} {} 01:74:c2:46:12:34:56", ipv4, hostname)?;
        }
        writeln!(f, "duid 00:01:00:01:24:99:3a:37:00:01:2e:12:34:56")?;
        for (hostname, ipv6) in ipv6_map {
            writeln!(f, "1561852704 224934210 {} {} 00:04:2e:3b:43:05:a5:df:ad:a0:32:bb:a8:a8:d3:12:34:56", ipv6, hostname)?;
        }

        Ok(())
    }
}
