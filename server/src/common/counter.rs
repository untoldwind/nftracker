use super::TrafficRate;
use crate::minirrd::{RRDEntry, RRD};
use chrono::{NaiveDateTime, Utc};
use itertools::Itertools;
use std::time::Duration;

#[derive(Debug, Clone, Copy, Default)]
pub struct TrafficCounter {
    pub bytes: u64,
    pub packets: u64,
}

impl RRDEntry for TrafficCounter {
    fn combine(self, other: &Self) -> Self {
        TrafficCounter {
            packets: self.packets.max(other.packets),
            bytes: self.bytes.max(other.bytes),
        }
    }

    fn interpolate(&self, previous: &Self, index: u64, steps: u64) -> Self {
        TrafficCounter {
            bytes: previous.bytes + (self.bytes - previous.bytes) * index / steps,
            packets: previous.packets + (self.packets - previous.packets) * index / steps,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Trafic {
    in_count: RRD<TrafficCounter>,
    out_count: RRD<TrafficCounter>,
}

impl Trafic {
    pub fn new(retain: Duration) -> Trafic {
        let now = Utc::now().naive_utc();
        Trafic {
            in_count: RRD::new(now, Duration::from_secs(1), retain),
            out_count: RRD::new(now, Duration::from_secs(1), retain),
        }
    }

    pub fn put_in(&mut self, timestamp: NaiveDateTime, bytes: u64, packets: u64) {
        self.in_count
            .put(timestamp, TrafficCounter { bytes, packets });
    }

    pub fn put_out(&mut self, timestamp: NaiveDateTime, bytes: u64, packets: u64) {
        self.out_count
            .put(timestamp, TrafficCounter { bytes, packets });
    }

    pub fn snapshot_in_rates(&self) -> (NaiveDateTime, Vec<TrafficRate>) {
        (
            self.in_count.first_timestamp(),
            self.in_count
                .iter()
                .tuple_windows()
                .map(|(prev, current)| TrafficRate::from_counter(prev, current))
                .collect(),
        )
    }

    pub fn snapshot_out_rates(&self) -> (NaiveDateTime, Vec<TrafficRate>) {
        (
            self.in_count.first_timestamp(),
            self.in_count
                .iter()
                .tuple_windows()
                .map(|(prev, current)| TrafficRate::from_counter(prev, current))
                .collect(),
        )
    }
}
