use std::collections::HashMap;
use std::time::Instant;

pub type Local = String;
pub type Remote = String;

#[derive(Debug)]
pub struct Entry {
    pub timestamp: Instant,
    pub bytes: u64,
    pub packets: u64,
}

#[derive(Debug, Default)]
pub struct Timeseries {
    pub in_entries: Vec<Entry>,
    pub out_entries: Vec<Entry>,
}

#[derive(Debug, Default)]
pub struct Table(pub HashMap<Local, HashMap<Remote, Timeseries>>);

impl Table {
    pub fn push_in(
        &mut self,
        timestamp: Instant,
        local: &str,
        remote: &str,
        bytes: u64,
        packets: u64,
    ) {
        let timeseries = self.upsert_timeseries(local, remote);
        timeseries.in_entries.push(Entry {
            timestamp,
            bytes,
            packets,
        });
    }

    pub fn push_out(
        &mut self,
        timestamp: Instant,
        local: &str,
        remote: &str,
        bytes: u64,
        packets: u64,
    ) {
        let timeseries = self.upsert_timeseries(local, remote);
        timeseries.out_entries.push(Entry {
            timestamp,
            bytes,
            packets,
        });
    }

    fn upsert_timeseries<'a>(&'a mut self, local: &str, remote: &str) -> &'a mut Timeseries {
        if !self.0.contains_key(local) {
            self.0.insert(local.to_string(), Default::default());
        }
        let remotes = self.0.get_mut(local).unwrap();
        if !remotes.contains_key(remote) {
            remotes.insert(remote.to_string(), Default::default());
        }
        remotes.get_mut(remote).unwrap()
    }
}
