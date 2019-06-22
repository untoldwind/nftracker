use super::parse::ConntrackEntry;
use std::collections::HashMap;
use std::time::Instant;

pub type Source = String;
pub type Target = String;

#[derive(Debug)]
pub struct Entry {
    pub timestamp: Instant,
    pub in_bytes: u64,
    pub in_packets: u64,
    pub out_bytes: u64,
    pub out_packets: u64,
}

#[derive(Debug, Default)]
pub struct Timeseries {
    pub target: Target,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Default)]
pub struct Table(pub HashMap<Source, Timeseries>);

impl Table {
    pub fn collect(&mut self, conntrack_entry: &ConntrackEntry) -> &mut Self {
        self
    }
}
