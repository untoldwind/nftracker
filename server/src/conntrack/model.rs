use crate::common::Trafic;
use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;

pub type Local = IpAddr;
pub type Remote = IpAddr;

#[derive(Debug)]
pub struct Table {
    retain: Duration,
    pub connections: HashMap<Local, HashMap<Remote, Trafic>>,
}

impl Table {
    pub fn new(retain: Duration) -> Table {
        Table {
            retain,
            connections: HashMap::new(),
        }
    }

    pub fn push_in(
        &mut self,
        timestamp: NaiveDateTime,
        local: IpAddr,
        remote: IpAddr,
        bytes: u64,
        packets: u64,
    ) {
        let trafic = self.upsert_timeseries(local, remote);
        trafic.put_in(timestamp, bytes, packets);
    }

    pub fn push_out(
        &mut self,
        timestamp: NaiveDateTime,
        local: IpAddr,
        remote: IpAddr,
        bytes: u64,
        packets: u64,
    ) {
        let traffic = self.upsert_timeseries(local, remote);
        traffic.put_out(timestamp, bytes, packets);
    }

    fn upsert_timeseries<'a>(&'a mut self, local: IpAddr, remote: IpAddr) -> &'a mut Trafic {
        if !self.connections.contains_key(&local) {
            self.connections.insert(local, HashMap::new());
        }
        let remotes = self.connections.get_mut(&local).unwrap();
        if !remotes.contains_key(&remote) {
            remotes.insert(remote, Trafic::new(self.retain));
        }
        remotes.get_mut(&remote).unwrap()
    }
}
