use nom::bytes::complete::is_not;
use nom::character::complete::{alphanumeric1, char, digit1, space1, hex_digit1};
use nom::combinator::{map_res, map};
use nom::error::{ParseError, ErrorKind};
use nom::multi::separated_list;
use nom::sequence::{preceded};
use nom::{IResult, Err};
use nom::branch::alt;

#[derive(Debug, Default)]
pub struct ConntrackEntry<'a> {
    pub transport: &'a str,
    pub protocol: &'a str,
    pub timeout: u64,
    pub src: &'a str,
    pub dst: &'a str,
    pub bytes: u64,
    pub packets: u64,
}

fn key_value_pair<'a, O, E: ParseError<&'a str>, F>(value_parse: F) -> impl Fn(&'a str) -> IResult<&'a str, (&'a str, O), E> 
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
    let mut remain = digit1(input)?.0;

    for _ in 0..3 {
        remain = char('.')(remain)?.0;
        remain = digit1(remain)?.0;
    }

    Ok((remain, &input[..(input.len() - remain.len())]))
}

fn ipv6<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    let mut remain = hex_digit1(input)?.0;
    
    for _ in 0..3 {
        remain = char('.')(remain)?.0;
        remain = hex_digit1(remain)?.0;
    }

    Ok((remain, &input[..(input.len() - remain.len())]))
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
        map(key_value_pair(map_res(digit1, str::parse::<u64>)), |(key, value)| Value::Number(key, value)),
        map(is_not(" \t"), Value::Any),
    ))(i)
}

fn parse_line<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Vec<ConntrackEntry<'a>>, E> {
    let (input, transport) = alphanumeric1(input)?;
    let (input, _) = preceded(space1, digit1)(input)?;
    let (input, protocol) = preceded(space1, alphanumeric1)(input)?;
    let (input, _) = preceded(space1, digit1)(input)?;
    let (input, timeout) = map_res(preceded(space1, digit1), str::parse::<u64>)(input)?;
    let (input, key_values) = preceded(space1, separated_list(space1, key_value))(input)?;
    let mut entries = Vec::with_capacity(2);
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
            _ => (),
        }
    }
    entries.push(current);
    Ok((input, entries))
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;
    use nom::error::VerboseError;

    #[test]
    fn test_parse_list() {
        let input = r#"ipv4     2 udp      17 27 src=192.168.3.56 dst=192.168.3.1 sport=51556 dport=53 packets=2 bytes=142 src=192.168.3.1 dst=192.168.3.56 sport=53 dport=51556 packets=2 bytes=416 [ASSURED] mark=0 zone=0 use=2"#;

        println!("{:?}", parse_line::<VerboseError<&str>>(input));
    }
}
