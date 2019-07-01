use crate::common::Timeseries;
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Instant;

pub type Local = IpAddr;
pub type Remote = IpAddr;

#[derive(Debug, Default)]
pub struct Table(pub HashMap<Local, HashMap<Remote, Timeseries>>);

impl Table {
    pub fn push_in(
        &mut self,
        timestamp: Instant,
        local: IpAddr,
        remote: IpAddr,
        bytes: u64,
        packets: u64,
    ) {
        let timeseries = self.upsert_timeseries(local, remote);
        timeseries.push_in(timestamp, bytes, packets);
    }

    pub fn push_out(
        &mut self,
        timestamp: Instant,
        local: IpAddr,
        remote: IpAddr,
        bytes: u64,
        packets: u64,
    ) {
        let timeseries = self.upsert_timeseries(local, remote);
        timeseries.push_out(timestamp, bytes, packets);
    }

    fn upsert_timeseries<'a>(&'a mut self, local: IpAddr, remote: IpAddr) -> &'a mut Timeseries {
        if !self.0.contains_key(&local) {
            self.0.insert(local, Default::default());
        }
        let remotes = self.0.get_mut(&local).unwrap();
        if !remotes.contains_key(&remote) {
            remotes.insert(remote, Default::default());
        }
        remotes.get_mut(&remote).unwrap()
    }
}
