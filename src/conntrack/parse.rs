use nom::bytes::complete::is_not;
use nom::character::complete::{alphanumeric1, char, digit1, space1};
use nom::combinator::{map_res, opt};
use nom::error::ParseError;
use nom::multi::separated_list;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

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

fn key_value<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (&'a str, &'a str), E> {
    separated_pair(is_not("= \t"), char('='), is_not(" \t"))(i)
}

fn parse_line<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Vec<ConntrackEntry<'a>>, E> {
    let (input, transport) = alphanumeric1(input)?;
    let (input, _) = preceded(space1, digit1)(input)?;
    let (input, protocol) = preceded(space1, alphanumeric1)(input)?;
    let (input, _) = preceded(space1, digit1)(input)?;
    let (input, timeout) = map_res(preceded(space1, digit1), str::parse::<u64>)(input)?;
    let (input, key_values) = preceded(space1, separated_list(space1, opt(key_value)))(input)?;
    let mut entries = Vec::with_capacity(2);
    let mut current = ConntrackEntry {
        transport,
        protocol,
        timeout,
        ..Default::default()
    };

    for key_value in key_values {
        match key_value {
            Some(("src", src)) => {
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
            Some(("dst", dst)) => current.dst = dst,
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
