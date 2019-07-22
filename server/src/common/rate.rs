use super::TrafficCounter;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Copy, Default)]
pub struct TrafficRate {
    bytes_per_sec: u64,
    packets_per_sec: u64,
}

impl TrafficRate {
    pub fn from_counter(
        prev: (NaiveDateTime, &TrafficCounter),
        current: (NaiveDateTime, &TrafficCounter),
    ) -> TrafficRate {
        let secs = (current.0 - prev.0).num_seconds();

        if secs < 1 {
            return Default::default();
        }

        let bytes_per_sec = if prev.1.bytes < current.1.bytes {
            (current.1.bytes - prev.1.bytes) / secs as u64
        } else {
            current.1.bytes / secs as u64
        };
        let packets_per_sec = if prev.1.packets < current.1.packets {
            (current.1.packets - prev.1.packets) / secs as u64
        } else {
            current.1.packets / secs as u64
        };

        TrafficRate {
            bytes_per_sec,
            packets_per_sec,
        }
    }
}
