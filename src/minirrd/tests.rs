use super::*;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};
use spectral::prelude::*;
use std::time::Duration;

#[derive(Debug, Default, Clone, PartialEq, Eq, Copy)]
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
    assert_that(&rrd.first_timestamp()).is_equal_to(rrd.last_timestamp());
    assert_that(&(rrd.first_timestamp() - now).num_milliseconds().abs()).is_less_than(1000);
}

#[test]
fn test_fill_single_points() {
    let start = NaiveDateTime::new(
        NaiveDate::from_ymd(2000, 1, 1),
        NaiveTime::from_hms(0, 0, 0),
    );
    let mut rrd = RRD::<Counter>::new(start, Duration::from_secs(1), Duration::from_secs(600));

    for i in 0..500 {
        let timestamp = start + chrono::Duration::milliseconds(500) * (2 * i + 1);
        rrd.put(timestamp, Counter(100 * i as u64));
    }

    assert_that(&rrd.first_timestamp()).is_equal_to(start);
    assert_that(&rrd.last_timestamp()).is_equal_to(start + chrono::Duration::seconds(499));
    assert_that(&rrd.len()).is_equal_to(500);
    assert_that(&rrd.iter().count()).is_equal_to(500);

    for (i, (timestamp, counter)) in rrd.iter().enumerate() {
        let expected_timestamp = start + chrono::Duration::seconds(1) * i as i32;
        let expected_counter = Counter(100 * i as u64);

        assert_that(&timestamp).is_equal_to(expected_timestamp);
        assert_that(&counter).is_equal_to(&expected_counter);

        if let Some((actual_timestamp, actual_counter)) = rrd.get(i) {
            assert_that(&actual_timestamp).is_equal_to(expected_timestamp);
            assert_that(&actual_counter).is_equal_to(&expected_counter);
        } else {
            panic!("No point at {}", i)
        }
    }

    for i in 500..1000 {
        let timestamp = start + chrono::Duration::milliseconds(500) * (2 * i + 1);
        rrd.put(timestamp, Counter(100 * i as u64));
    }

    assert_that(&rrd.len()).is_equal_to(600);
    assert_that(&rrd.iter().count()).is_equal_to(600);
    assert_that(&rrd.first_timestamp()).is_equal_to(start + chrono::Duration::seconds(400));
    assert_that(&rrd.last_timestamp()).is_equal_to(start + chrono::Duration::seconds(999));

    for (i, (timestamp, counter)) in rrd.iter().enumerate() {
        let expected_timestamp = start + chrono::Duration::seconds(1) * (i + 400) as i32;
        let expected_counter = Counter(100 * (i + 400) as u64);

        assert_that(&timestamp).is_equal_to(expected_timestamp);
        assert_that(&counter).is_equal_to(&expected_counter);

        if let Some((actual_timestamp, actual_counter)) = rrd.get(i) {
            assert_that(&actual_timestamp).is_equal_to(expected_timestamp);
            assert_that(&actual_counter).is_equal_to(&expected_counter);
        } else {
            panic!("No point at {}", i)
        }
    }
}

#[test]
fn test_fill_double_points() {
    let start = NaiveDateTime::new(
        NaiveDate::from_ymd(2000, 1, 1),
        NaiveTime::from_hms(0, 0, 0),
    );
    let mut rrd = RRD::<Counter>::new(start, Duration::from_secs(1), Duration::from_secs(600));

    for i in 0..1000 {
        let timestamp = start + chrono::Duration::milliseconds(250) * (2 * i + 1);
        rrd.put(timestamp, Counter(100 * i as u64));
    }

    assert_that(&rrd.first_timestamp()).is_equal_to(start);
    assert_that(&rrd.last_timestamp()).is_equal_to(start + chrono::Duration::seconds(499));
    assert_that(&rrd.len()).is_equal_to(500);
    assert_that(&rrd.iter().count()).is_equal_to(500);

    for (i, (timestamp, counter)) in rrd.iter().enumerate() {
        let expected_timestamp = start + chrono::Duration::seconds(1) * i as i32;
        let expected_counter = Counter(100 * (2 * i + 1) as u64);

        assert_that(&timestamp).is_equal_to(expected_timestamp);
        assert_that(&counter).is_equal_to(&expected_counter);

        if let Some((actual_timestamp, actual_counter)) = rrd.get(i) {
            assert_that(&actual_timestamp).is_equal_to(expected_timestamp);
            assert_that(&actual_counter).is_equal_to(&expected_counter);
        } else {
            panic!("No point at {}", i)
        }
    }
}
