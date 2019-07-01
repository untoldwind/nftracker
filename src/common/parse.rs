use super::Subnet;
use nom::branch::alt;
use nom::character::complete::{char, digit1, hex_digit1};
use nom::combinator::{map, map_res};
use nom::error::{ErrorKind, ParseError};
use nom::multi::many_m_n;
use nom::sequence::{preceded, terminated};
use nom::{Err, IResult};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub fn ipv4_subnet<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Subnet, E> {
    let (input, prefix) = many_m_n(1, 3, map_res(terminated(digit1, char('.')), str::parse::<u8>))(input)?;

    Ok((input, Subnet::V4(prefix)))
}

pub fn ipv6_subnet<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Subnet, E> {
    let (input, prefix) = many_m_n(1, 7, map_res(terminated(hex_digit1, char(':')), |s| {
        u16::from_str_radix(s, 16)
    }))(input)?;

    Ok((input, Subnet::V6(prefix)))
}

pub fn subnet<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Subnet, E> {
    alt((ipv4_subnet, ipv6_subnet))(input)
}

pub fn ipv4_addr<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Ipv4Addr, E> {
    let mut octets = [0u8; 4];

    let (input, octet) = map_res(digit1, str::parse::<u8>)(input)?;
    octets[0] = octet;
    let (input, octet) = map_res(preceded(char('.'), digit1), str::parse::<u8>)(input)?;
    octets[1] = octet;
    let (input, octet) = map_res(preceded(char('.'), digit1), str::parse::<u8>)(input)?;
    octets[2] = octet;
    let (input, octet) = map_res(preceded(char('.'), digit1), str::parse::<u8>)(input)?;
    octets[3] = octet;

    Ok((input, Ipv4Addr::from(octets)))
}

pub fn ipv6_addr<'a, E: ParseError<&'a str>>(orginal: &'a str) -> IResult<&'a str, Ipv6Addr, E> {
    let mut head_segments = Vec::<u16>::with_capacity(8);
    let mut tail_segments = Vec::<u16>::with_capacity(8);
    let mut append_head = true;

    let (mut input, word) = map_res(hex_digit1, |s| u16::from_str_radix(s, 16))(orginal)?;
    head_segments.push(word);

    while let Ok((remain, _)) = char::<&'a str, E>(':')(input) {
        input = remain;
        if append_head {
            if let Ok((remain, _)) = char::<&'a str, E>(':')(input) {
                input = remain;
                append_head = false;
            }
        }
        let (remain, word) = map_res(hex_digit1, |s| u16::from_str_radix(s, 16))(input)?;
        input = remain;
        if append_head {
            head_segments.push(word);
        } else {
            tail_segments.push(word);
        }
    }
    if head_segments.len() + tail_segments.len() < 2 {
        return Err(Err::Error(E::from_char(orginal, ':')));
    }
    if head_segments.len() + tail_segments.len() > 8 {
        return Err(Err::Error(E::from_error_kind(orginal, ErrorKind::TooLarge)));
    }
    let mut segments = [0u16; 8];
    segments[0..head_segments.len()].copy_from_slice(&head_segments);
    if !tail_segments.is_empty() {
        segments[(8 - tail_segments.len())..].copy_from_slice(&tail_segments);
    }

    Ok((input, Ipv6Addr::from(segments)))
}

pub fn ip_addr<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, IpAddr, E> {
    alt((map(ipv4_addr, IpAddr::V4), map(ipv6_addr, IpAddr::V6)))(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::error::VerboseError;
    use spectral::prelude::*;

    #[test]
    fn test_ipv4_addr() {
        let input = "1.12.123.234";
        let (remain, addr) = ipv4_addr::<VerboseError<&str>>(input).unwrap();

        assert_that(&remain).is_equal_to("");
        assert_that(&addr.octets()).is_equal_to([1u8, 12u8, 123u8, 234u8])
    }

    #[test]
    fn test_ipv6_addr() {
        let input = "1234:5678::4321";
        let (remain, addr) = ipv6_addr::<VerboseError<&str>>(input).unwrap();

        assert_that(&remain).is_equal_to("");
        assert_that(&addr.segments()).is_equal_to([
            0x1234u16, 0x5678u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0x4321u16,
        ]);

        let input = "1234:2345:3456:4567:4321:5432:6543:7654";
        let (remain, addr) = ipv6_addr::<VerboseError<&str>>(input).unwrap();

        assert_that(&remain).is_equal_to("");
        assert_that(&addr.segments()).is_equal_to([
            0x1234u16, 0x2345u16, 0x3456u16, 0x4567u16, 0x4321u16, 0x5432u16, 0x6543u16, 0x7654u16,
        ]);
    }

    #[test]
    fn test_ip_addr() {
        let input = "123.34.4.56";
        let (remain, addr) = ip_addr::<VerboseError<&str>>(input).unwrap();

        assert_that(&remain).is_equal_to("");
        assert_that(&addr.is_ipv4()).is_true();

        let input = "1234:5678::4321";
        let (remain, addr) = ip_addr::<VerboseError<&str>>(input).unwrap();

        assert_that(&remain).is_equal_to("");
        assert_that(&addr.is_ipv6()).is_true();
    }
}
