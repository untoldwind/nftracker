use chrono::NaiveDateTime;
use std::time::Duration;

#[cfg(test)]
mod tests;

pub trait RRDEntry: Default + Clone {
    fn combine(self, other: Self) -> Self;

    fn interpolate(&self, previous: &Self, index: usize, steps: usize) -> Self;
}

pub struct RRD<E> {
    resultion_millis: usize,
    first_timestamp: NaiveDateTime,
    first_index: usize,
    last_index: usize,
    ring: Vec<E>,
}

impl<E: RRDEntry> RRD<E> {
    pub fn new(start: NaiveDateTime, resultion: Duration, retain: Duration) -> Self {
        let resultion_millis = resultion.as_millis() as usize;
        let len = retain.as_millis() as usize / resultion_millis;

        assert!(resultion_millis > 0);
        assert!(len > 0);

        let start_millis =
            start.timestamp_millis() - start.timestamp_millis() % resultion_millis as i64;

        RRD {
            resultion_millis,
            first_timestamp: NaiveDateTime::from_timestamp(
                start_millis / 1_000,
                (start_millis % 1_000) as u32 * 1_000_000,
            ),
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
        if self.first_index <= self.last_index {
            self.first_timestamp()
                + chrono::Duration::microseconds(
                    ((self.last_index - self.first_index) * self.resultion_millis) as i64,
                )
        } else {
            self.first_timestamp()
                + chrono::Duration::microseconds(
                    ((self.ring.len() - self.first_index + self.last_index) * self.resultion_millis)
                        as i64,
                )
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (NaiveDateTime, &E)> {
        RRDIterator {
            rrd: self,
            position: self.first_index,
        }
    }

    pub fn put(&mut self, timestamp: NaiveDateTime, entry: E) -> bool {
        if timestamp < self.first_timestamp {
            return false;
        }
        let offset =
            (timestamp - self.first_timestamp).num_milliseconds() as usize / self.resultion_millis;
        if offset >= 2 * self.ring.len() {
            // So far in future that all previous values become obsolete
            let steps_from_last = (timestamp - self.last_timestamp()).num_milliseconds() as usize
                / self.resultion_millis;
            let last = self.ring[self.last_index].clone();
            let millis = timestamp.timestamp_millis()
                - timestamp.timestamp_millis() % self.resultion_millis as i64
                - ((self.ring.len() - 1) * self.resultion_millis) as i64;
            self.first_index = 0;
            self.first_timestamp =
                NaiveDateTime::from_timestamp(millis / 1_000, (millis % 1_000) as u32 * 1_000_000);
            self.last_index = self.ring.len() - 1;
            for i in 0..self.ring.len() {
                self.ring[i] = entry.interpolate(
                    &last,
                    steps_from_last - (self.ring.len() - 1 - i),
                    steps_from_last,
                )
            }
        }
        if offset < self.ring.len() {
            let mut index = self.first_index + offset;
            if index >= self.ring.len() {
                index -= self.ring.len();
            }
        } else {

        }
        unimplemented!()
    }
}

struct RRDIterator<'a, E> {
    rrd: &'a RRD<E>,
    position: usize,
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
        let timestamp = self.rrd.first_timestamp
            + chrono::Duration::microseconds((self.position * self.rrd.resultion_millis) as i64);
        Some((timestamp, &self.rrd.ring[self.position]))
    }
}
