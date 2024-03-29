use super::Lease;
use crate::common::parse::ip_addr;
use log::debug;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{char, digit1, hex_digit1, space1};
use nom::combinator::recognize;
use nom::error::{ParseError, VerboseError};
use nom::multi::many0_count;
use nom::sequence::preceded;
use nom::IResult;
use std::io::{self, BufRead, BufReader, Read};

fn hostname<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    alt((
        tag("*"),
        take_while1(|ch: char| ch.is_alphanumeric() || ch == '-' || ch == '_'),
    ))(input)
}

fn mac_like<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    recognize(preceded(
        hex_digit1,
        many0_count(preceded(char(':'), hex_digit1)),
    ))(input)
}

fn parse_line<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Lease, E> {
    let (input, _) = digit1(input)?;
    let (input, _) = preceded(space1, mac_like)(input)?;
    let (input, addr) = preceded(space1, ip_addr)(input)?;
    let (input, name) = preceded(space1, hostname)(input)?;
    let (input, client_id) = preceded(space1, mac_like)(input)?;

    Ok((
        input,
        Lease {
            name: name.to_string(),
            addr,
            client_id: client_id.to_string(),
        },
    ))
}

pub fn parse<I, V, C>(input: I, mut initial: C, visitor: V) -> io::Result<C>
where
    I: Read,
    V: Fn(C, Lease) -> C,
{
    let buf_reader = BufReader::new(input);

    for line_result in buf_reader.lines() {
        let line = line_result?;

        match parse_line::<VerboseError<&str>>(&line) {
            Ok((_, lease)) => initial = visitor(initial, lease),
            Err(error) => {
                debug!("Invalid conntrack entry: {:?}", error);
            }
        }
    }
    Ok(initial)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::VerboseError;
    use spectral::prelude::*;
    use std::net::IpAddr;

    #[test]
    fn test_parse_line_ipv4() {
        let input = r#"1562979553 24:5e:be:12:34:56 192.168.3.86 brick 01:24:5e:be:12:34:56"#;
        let (remain, lease) = parse_line::<VerboseError<&str>>(input).unwrap();

        assert_that(&remain).is_equal_to("");
        assert_that(&lease.name).is_equal_to("brick".to_string());
        assert_that(&lease.addr).is_equal_to("192.168.3.86".parse::<IpAddr>().unwrap());
        assert_that(&lease.client_id).is_equal_to("01:24:5e:be:12:34:56".to_string());
    }

    #[test]
    fn test_parse_line_ipv6() {
        let input = r#"1561852704 224934210 1234::28a thunder 00:04:2e:3b:43:05:a5:df:ad:a0:32:bb:a8:a8:d3:12:34:56"#;
        let (remain, lease) = parse_line::<VerboseError<&str>>(input).unwrap();

        assert_that(&remain).is_equal_to("");
        assert_that(&lease.name).is_equal_to("thunder".to_string());
        assert_that(&lease.addr).is_equal_to("1234::28a".parse::<IpAddr>().unwrap());
        assert_that(&lease.client_id)
            .is_equal_to("00:04:2e:3b:43:05:a5:df:ad:a0:32:bb:a8:a8:d3:12:34:56".to_string());
    }
}
