use super::{helpers, Rate, Traffic};
use rand::{thread_rng, RngCore};
use std::fs::File;
use std::io::{self, Write};
use std::net::IpAddr;

pub struct ConntrackTarget {
    target_ip_addr: IpAddr,
    source_ip_addr: IpAddr,
    source_hostname: String,
    in_traffic: Traffic,
    in_rate: Rate,
    out_traffic: Traffic,
    out_rate: Rate,
}

impl ConntrackTarget {
    fn random() -> ConntrackTarget {
        let target_ip_addr = helpers::random_ip(&[123], &[0x2345]);
        let source_ip_addr = match target_ip_addr {
            IpAddr::V4(_) => IpAddr::V4(helpers::random_ipv4(&[192, 168, 1])),
            IpAddr::V6(_) => IpAddr::V6(helpers::random_ipv6(&[0x1234])),
        };

        ConntrackTarget {
            target_ip_addr,
            source_ip_addr,
            source_hostname: helpers::random_hostname(),
            in_traffic: Default::default(),
            in_rate: Rate::random(10_000_000),
            out_traffic: Default::default(),
            out_rate: Rate::random(1_000_000),
        }
    }

    fn tick(&mut self) {
        self.in_traffic += &self.in_rate;
        self.out_traffic += &self.out_rate;
    }
}

pub struct ConntrackSimulator<'a> {
    file: &'a str,
    targets: Vec<ConntrackTarget>,
    in_offset: Traffic,
    out_offset: Traffic,
}

impl<'a> ConntrackSimulator<'a> {
    pub fn new(file: &'a str) -> ConntrackSimulator<'a> {
        ConntrackSimulator {
            file,
            targets: vec![],
            in_offset: Default::default(),
            out_offset: Default::default(),
        }
    }

    pub fn tick(&mut self) {
        let mut rng = thread_rng();

        if rng.next_u32() % 100 < 2 || self.targets.len() < 3 {
            self.targets.push(ConntrackTarget::random());
        }
        if rng.next_u32() % 100 < 2 && self.targets.len() > 3 {
            let removed = self
                .targets
                .remove(rng.next_u32() as usize % self.targets.len());
            self.in_offset += removed.in_traffic;
            self.out_offset += removed.out_traffic;
        }
        self.targets.iter_mut().for_each(ConntrackTarget::tick);
    }

    pub fn dump(&self) -> io::Result<()> {
        let mut f = File::create(self.file)?;

        for target in &self.targets {
            let protocol = match target.source_ip_addr {
                IpAddr::V4(_) => "ipv4",
                IpAddr::V6(_) => "ipv6",
            };

            writeln!(f,
                "{}     2 tcp      6 431741 ESTABLISHED src={} dst={} sport=50054 dport=443 packets={} bytes={} src={} dst={} sport=443 dport=50054 packets={} bytes={} [ASSURED] mark=0 zone=0 use=2", 
                protocol, target.source_ip_addr, target.target_ip_addr, target.in_traffic.packets, target.in_traffic.bytes,
                target.target_ip_addr,  target.source_ip_addr, target.out_traffic.packets, target.out_traffic.bytes)?;
        }

        Ok(())
    }

    pub fn total_in_traffic(&self) -> Traffic {
        let mut traffic = self
            .targets
            .iter()
            .map(|t| t.in_traffic)
            .collect::<Traffic>();
        traffic += self.in_offset;
        traffic
    }

    pub fn total_out_traffic(&self) -> Traffic {
        let mut traffic = self
            .targets
            .iter()
            .map(|t| t.out_traffic)
            .collect::<Traffic>();
        traffic += self.out_offset;
        traffic
    }
}
