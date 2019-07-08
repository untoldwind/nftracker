use crate::minirrd::{RRDEntry, RRD};
use chrono::{NaiveDateTime, Utc};
use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct TraficCounter {
    pub bytes: u64,
    pub packets: u64,
}

impl RRDEntry for TraficCounter {
    fn combine(self, other: &Self) -> Self {
        TraficCounter {
            packets: self.packets.max(other.packets),
            bytes: self.bytes.max(other.bytes),
        }
    }

    fn interpolate(&self, previous: &Self, index: u64, steps: u64) -> Self {
        TraficCounter {
            bytes: previous.bytes + (self.bytes - previous.bytes) * index / steps,
            packets: previous.packets + (self.packets - previous.packets) * index / steps,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Trafic {
    in_count: RRD<TraficCounter>,
    out_count: RRD<TraficCounter>,
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
            .put(timestamp, TraficCounter { bytes, packets });
    }

    pub fn put_out(&mut self, timestamp: NaiveDateTime, bytes: u64, packets: u64) {
        self.out_count
            .put(timestamp, TraficCounter { bytes, packets });
    }
}
