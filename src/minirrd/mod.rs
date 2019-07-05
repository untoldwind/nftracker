use chrono::NaiveDateTime;
use std::time::Duration;

#[cfg(test)]
mod tests;

pub trait RRDEntry: Default + Clone {
    fn combine(self, other: &Self) -> Self;

    fn interpolate(&self, previous: &Self, index: u64, steps: u64) -> Self;
}

pub struct RRD<E> {
    resolution: chrono::Duration,
    resolution_millis: usize,
    first_timestamp: NaiveDateTime,
    last_timestamp: NaiveDateTime,
    first_index: usize,
    last_index: usize,
    ring: Vec<E>,
}

impl<E: RRDEntry> RRD<E> {
    pub fn new(start: NaiveDateTime, resultion: Duration, retain: Duration) -> Self {
        let resolution_millis = resultion.as_millis() as usize;
        let len = retain.as_millis() as usize / resolution_millis;

        assert!(resolution_millis > 0);
        assert!(len > 0);

        let start_millis =
            start.timestamp_millis() - start.timestamp_millis() % resolution_millis as i64;
        let rounded_timestamp = NaiveDateTime::from_timestamp(
                start_millis / 1_000,
                (start_millis % 1_000) as u32 * 1_000_000,
            );

        RRD {
            resolution: chrono::Duration::microseconds(resolution_millis as i64),
            resolution_millis,
            first_timestamp: rounded_timestamp,
            last_timestamp: rounded_timestamp,
            first_index: 0,
            last_index: 0,
            ring: vec![Default::default(); len],
        }
    }

    pub fn len(&self) -> usize {
        if self.first_index <= self.last_index {
            self.last_index - self.first_index + 1
        } else {
            self.ring.len() - self.first_index + self.last_index + 1
        }
    }

    pub fn first_timestamp(&self) -> NaiveDateTime {
        self.first_timestamp
    }

    pub fn last_timestamp(&self) -> NaiveDateTime {
        self.last_timestamp
    }

    pub fn iter(&self) -> impl Iterator<Item = (NaiveDateTime, &E)> {
        RRDIterator {
            rrd: self,
            position: self.first_index,
            timestamp: self.first_timestamp(),
        }
    }

    pub fn put(&mut self, timestamp: NaiveDateTime, entry: E) -> bool {
        if timestamp < self.first_timestamp {
            return false;
        }
        let offset_from_last = (timestamp - self.last_timestamp).num_milliseconds() / self.resolution_millis as i64;
        if offset_from_last >= self.ring.len() as i64 {
            // So far in future that all previous values become obsolete
            let last = self.ring[self.last_index].clone();
            let first_millis = timestamp.timestamp_millis()
                - timestamp.timestamp_millis() % self.resolution_millis as i64
                - ((self.ring.len() - 1) * self.resolution_millis) as i64;
            self.first_index = 0;
            self.first_timestamp =
                NaiveDateTime::from_timestamp(first_millis / 1_000, (first_millis % 1_000) as u32 * 1_000_000);
            self.last_index = self.ring.len() - 1;
            self.last_timestamp = self.first_timestamp() + self.resolution * (self.ring.len() - 1) as i32;
            for i in 0..self.ring.len() {
                self.ring[i] = entry.interpolate(
                    &last,
                    offset_from_last as u64 - (self.ring.len() - 1 - i) as u64,
                    offset_from_last as u64,
                )
            }
        } else if offset_from_last > 0 {
            // Have to advance last (and potentially first)
            let last = self.ring[self.last_index].clone();
            for i in 1..=offset_from_last as u64 {
                self.advance_last();
                self.ring[self.last_index] = entry.interpolate(&last, i, offset_from_last as u64);
            }
        } else {
            let mut index = self.last_index as i64 + offset_from_last;
            if index < 0 {
                index += self.ring.len() as i64;
            }
            let combined = entry.combine(&self.ring[index as usize]);
            self.ring[index as usize] = combined;
        }
        true
    }

    fn advance_last(&mut self) {
                self.last_index += 1;
                self.last_timestamp += self.resolution;
                if self.last_index >= self.ring.len() {
                    self.last_index = 0;
                }
                if self.first_index == self.last_index {
                    self.advance_first();
                }
    }

    fn advance_first(&mut self) {
        self.first_index += 1;
        self.first_timestamp += self.resolution;
        if self.first_index >= self.ring.len() {
            self.first_index = 0;
        }
    }
}

struct RRDIterator<'a, E> {
    rrd: &'a RRD<E>,
    position: usize,
    timestamp: NaiveDateTime,
}

impl<'a, E> Iterator for RRDIterator<'a, E> {
    type Item = (NaiveDateTime, &'a E);

    fn next(&mut self) -> Option<Self::Item> {
        if self.position == self.rrd.last_index {
            return None;
        }
        self.position += 1;
        if self.position > self.rrd.ring.len() {
            self.position = 0;
        }
        self.timestamp += self.rrd.resolution;
        Some((self.timestamp, &self.rrd.ring[self.position]))
    }
}
