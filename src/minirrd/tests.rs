use super::*;
use chrono::Utc;
use spectral::prelude::*;
use std::time::Duration;

#[derive(Debug, Default, Clone)]
struct Counter(pub u64);

impl RRDEntry for Counter {
    fn combine(self, other: &Self) -> Self {
        Counter(self.0.max(other.0))
    }

    fn interpolate(&self, previous: &Self, index: u64, steps: u64) -> Self {
        Counter(previous.0 + (self.0 - previous.0) * index as u64 / steps as u64)
    }
}

#[test]
fn test_create_empty() {
    let now = Utc::now().naive_utc();
    let rrd = RRD::<Counter>::new(now, Duration::from_secs(1), Duration::from_secs(600));

    assert_that(&rrd.len()).is_equal_to(1);
}
