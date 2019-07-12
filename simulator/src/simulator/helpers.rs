use rand::distributions::Distribution;
use rand::{Rng, RngCore};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

struct LowerAlpha;

impl Distribution<char> for LowerAlpha {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        const RANGE: u32 = 26;
        const GEN_ASCII_STR_CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

        return GEN_ASCII_STR_CHARSET[(rng.next_u32() % RANGE) as usize] as char;
    }
}

struct LowerAlphanumeric;

impl Distribution<char> for LowerAlphanumeric {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        const RANGE: u32 = 26 + 10;
        const GEN_ASCII_STR_CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";

        return GEN_ASCII_STR_CHARSET[(rng.next_u32() % RANGE) as usize] as char;
    }
}

pub fn random_hostname() -> String {
    let mut rng = rand::thread_rng();
    let mut name = String::with_capacity(8);

    name.push(rng.sample(LowerAlpha));
    name.extend(rng.sample_iter(LowerAlphanumeric).take(7));
    name
}

pub fn random_ipv4(prefix: &[u8]) -> Ipv4Addr {
    let mut rng = rand::thread_rng();
    let mut octets = [0u8; 4];

    for i in 0..4 {
        if i < prefix.len() {
            octets[i] = prefix[i];
        } else {
            octets[i] = (rng.next_u32() & 0xff) as u8;
        }
    }
    Ipv4Addr::from(octets)
}

pub fn random_ipv6(prefix: &[u16]) -> Ipv6Addr {
    let mut rng = rand::thread_rng();
    let mut words = [0u16; 8];

    for i in 0..8 {
        if i < prefix.len() {
            words[i] = prefix[i];
        } else {
            words[i] = (rng.next_u32() & 0xffff) as u16;
        }
    }
    Ipv6Addr::from(words)
}

pub fn random_ip(v4_prefix: &[u8], v6_prefix: &[u16]) -> IpAddr {
    let mut rng = rand::thread_rng();

    if rng.next_u32() & 0x1 == 0 {
        IpAddr::V4(random_ipv4(v4_prefix))
    } else {
        IpAddr::V6(random_ipv6(v6_prefix))
    }
}
