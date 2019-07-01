use std::time::Instant;

#[derive(Debug)]
pub struct TimeseriesEntry {
    pub timestamp: Instant,
    pub bytes: u64,
    pub packets: u64,
}

#[derive(Debug, Default)]
pub struct Timeseries {
    pub in_entries: Vec<TimeseriesEntry>,
    pub out_entries: Vec<TimeseriesEntry>,
}

impl Timeseries {
    pub fn push_in(&mut self, timestamp: Instant, bytes: u64, packets: u64) {
        self.in_entries.push(TimeseriesEntry {
            timestamp,
            bytes,
            packets,
        })
    }

    pub fn push_out(&mut self, timestamp: Instant, bytes: u64, packets: u64) {
        self.out_entries.push(TimeseriesEntry {
            timestamp,
            bytes,
            packets,
        })
    }

    pub fn prune(&mut self, older_than: Instant) {
        let mut i = 0;
        while i != self.in_entries.len() {
            if self.in_entries[i].timestamp < older_than {
                self.in_entries.remove(i);
            } else {
                i += 1;
            }
        }

        i = 0;
        while i != self.out_entries.len() {
            if self.out_entries[i].timestamp < older_than {
                self.out_entries.remove(i);
            } else {
                i += 1;
            }
        }
    }
}
