use rand_distr::Normal;
use rand::{thread_rng, Rng, RngCore};
use std::iter::FromIterator;
use std::ops;

#[derive(Debug)]
pub struct Rate {
    bytes_per_sec: u64,
    packets_per_sec: u64,
}

impl Rate {
    pub fn random(bytes_per_sec: u64) -> Rate {
        let mut rng = thread_rng();

        let bytes_per_sec = rng.next_u64() % bytes_per_sec + bytes_per_sec / 100;
        let packets_per_sec = bytes_per_sec / 1_000;

        Rate {
            bytes_per_sec,
            packets_per_sec,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Traffic {
    pub bytes: u64,
    pub packets: u64,
}

impl ops::AddAssign for Traffic {
    fn add_assign(&mut self, rhs: Traffic) {
        self.bytes += rhs.bytes;
        self.packets += rhs.packets;
    }
}

impl ops::AddAssign<&Rate> for Traffic {
    fn add_assign(&mut self, rhs: &Rate) {
        let mut rng = thread_rng();

        self.bytes += rng
            .sample(Normal::new(
                rhs.bytes_per_sec as f64,
                rhs.bytes_per_sec as f64 / 4.0,
            ).unwrap())
            .max(0.0) as u64;
        self.packets += rng
            .sample(Normal::new(
                rhs.packets_per_sec as f64,
                rhs.packets_per_sec as f64 / 4.0,
            ).unwrap())
            .max(0.0) as u64;
    }
}

impl FromIterator<Traffic> for Traffic {
    fn from_iter<T: IntoIterator<Item = Traffic>>(iter: T) -> Self {
        let mut traffic = Default::default();

        for item in iter {
            traffic += item
        }
        traffic
    }
}
