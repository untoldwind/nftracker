use crate::minivec::MiniVec;
use log::error;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::character::complete::{alphanumeric1, char, digit1, hex_digit1, space1};
use nom::combinator::{map, map_res, recognize};
use nom::error::{ErrorKind, ParseError, VerboseError};
use nom::multi::{count, separated_list};
use nom::sequence::preceded;
use nom::{Err, IResult};
use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug, Default, Clone, Copy)]
pub struct ConntrackEntry<'a> {
    pub transport: &'a str,
    pub protocol: &'a str,
    pub timeout: u64,
    pub src: &'a str,
    pub sport: u16,
    pub dst: &'a str,
    pub dport: u16,
    pub bytes: u64,
    pub packets: u64,
}

fn key_value_pair<'a, O, E: ParseError<&'a str>, F>(
    value_parse: F,
) -> impl Fn(&'a str) -> IResult<&'a str, (&'a str, O), E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    move |input: &'a str| {
        let (input, key) = is_not("= \t")(input)?;
        let (input, _) = char('=')(input)?;
        let (input, value) = value_parse(input)?;

        if input.is_empty() || "= \t".contains(input.chars().next().unwrap()) {
            Ok((input, (key, value)))
        } else {
            Err(Err::Error(E::from_error_kind(input, ErrorKind::Complete)))
        }
    }
}

fn ipv4<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    recognize(preceded(digit1, count(preceded(char('.'), digit1), 3)))(input)
}

fn ipv6<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    recognize(preceded(
        hex_digit1,
        count(preceded(char(':'), hex_digit1), 7),
    ))(input)
}

#[derive(Debug)]
enum Value<'a> {
    Addr(&'a str, &'a str),
    Number(&'a str, u64),
    Any(&'a str),
}

fn key_value<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Value, E> {
    alt((
        map(key_value_pair(ipv4), |(key, value)| Value::Addr(key, value)),
        map(key_value_pair(ipv6), |(key, value)| Value::Addr(key, value)),
        map(
            key_value_pair(map_res(digit1, str::parse::<u64>)),
            |(key, value)| Value::Number(key, value),
        ),
        map(is_not(" \t"), Value::Any),
    ))(i)
}

fn parse_line<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, MiniVec<ConntrackEntry<'a>>, E> {
    let (input, transport) = alphanumeric1(input)?;
    let (input, _) = preceded(space1, digit1)(input)?;
    let (input, protocol) = preceded(space1, alphanumeric1)(input)?;
    let (input, _) = preceded(space1, digit1)(input)?;
    let (input, timeout) = map_res(preceded(space1, digit1), str::parse::<u64>)(input)?;
    let (input, key_values) = preceded(space1, separated_list(space1, key_value))(input)?;
    let mut entries: MiniVec<ConntrackEntry<'a>> = Default::default();
    let mut current = ConntrackEntry {
        transport,
        protocol,
        timeout,
        ..Default::default()
    };

    for key_value in key_values {
        match key_value {
            Value::Addr("src", src) => {
                if !current.src.is_empty() {
                    entries.push(current);
                    current = ConntrackEntry {
                        transport,
                        protocol,
                        timeout,
                        ..Default::default()
                    };
                }
                current.src = src;
            }
            Value::Addr("dst", dst) => current.dst = dst,
            Value::Number("sport", sport) => current.sport = sport as u16,
            Value::Number("dport", dport) => current.dport = dport as u16,
            Value::Number("bytes", bytes) => current.bytes = bytes,
            Value::Number("packets", packets) => current.packets = packets,
            _ => (),
        }
    }
    entries.push(current);
    Ok((input, entries))
}

pub fn parse<I, V, C>(input: I, mut initial: C, visitor: V) -> io::Result<C>
where
    I: Read,
    V: Fn(C, &ConntrackEntry<'_>) -> C,
{
    let buf_reader = BufReader::new(input);

    for line_result in buf_reader.lines() {
        let line = line_result?;
        match parse_line::<VerboseError<&str>>(&line) {
            Ok((_, entries)) => initial = entries.visit(initial, &visitor),
            Err(error) => {
                error!("Invalid conntrack entry: {:?}", error);
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
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_parse_line() {
        let input = r#"ipv4     2 udp      17 27 src=192.168.3.56 dst=192.168.3.1 sport=51556 dport=53 packets=2 bytes=142 src=192.168.3.1 dst=192.168.3.56 sport=53 dport=51556 packets=2 bytes=416 [ASSURED] mark=0 zone=0 use=2"#;
        let (remain, mut entries) = parse_line::<VerboseError<&str>>(input).unwrap();

        assert_that(&remain).is_equal_to("");
        assert_that(&entries.len()).is_equal_to(2);

        let second = entries.pop().unwrap();
        let first = entries.pop().unwrap();

        assert_that(&first.src).is_equal_to("192.168.3.56");
        assert_that(&first.dst).is_equal_to("192.168.3.1");
        assert_that(&first.bytes).is_equal_to(142);
        assert_that(&second.src).is_equal_to("192.168.3.1");
        assert_that(&second.dst).is_equal_to("192.168.3.56");
        assert_that(&second.bytes).is_equal_to(416);
    }

    #[test]
    fn test_parse_conntrack_file() {
        let file = File::open("fixtures/nf_conntrack").unwrap();
        let count = parse(file, 0, |count, entry| count + entry.bytes);

        assert_that(&count).is_ok_containing(192841);
    }
}
