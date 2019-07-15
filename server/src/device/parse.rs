use log::error;
use nom::character::complete::{alphanumeric1, char, digit1, space1};
use nom::combinator::map_res;
use nom::error::{ParseError, VerboseError};
use nom::multi::count;
use nom::sequence::{preceded, terminated};
use nom::IResult;
use std::io::{self, BufRead, BufReader, Read};

pub struct InterfaceStats<'a> {
    pub interface: &'a str,
    pub receive_bytes: u64,
    pub receive_packets: u64,
    pub transmit_bytes: u64,
    pub transmit_packets: u64,
}

fn parse_line<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, InterfaceStats<'a>, E> {
    let (input, interface) = terminated(alphanumeric1, char(':'))(input)?;
    let (input, receive_bytes) = map_res(preceded(space1, digit1), str::parse::<u64>)(input)?;
    let (input, receive_packets) = map_res(preceded(space1, digit1), str::parse::<u64>)(input)?;
    let (input, _) = count(preceded(space1, digit1), 6)(input)?;
    let (input, transmit_bytes) = map_res(preceded(space1, digit1), str::parse::<u64>)(input)?;
    let (input, transmit_packets) = map_res(preceded(space1, digit1), str::parse::<u64>)(input)?;
    let (input, _) = count(preceded(space1, digit1), 6)(input)?;

    Ok((
        input,
        InterfaceStats {
            interface: interface.trim(),
            receive_bytes,
            receive_packets,
            transmit_bytes,
            transmit_packets,
        },
    ))
}

pub fn parse<I, V, C>(input: I, mut initial: C, visitor: V) -> io::Result<C>
where
    I: Read,
    V: Fn(C, &InterfaceStats<'_>) -> C,
{
    let buf_reader = BufReader::new(input);

    for (i, line_result) in buf_reader.lines().enumerate() {
        if i < 2 {
            continue;
        }
        let line = line_result?;
        match parse_line::<VerboseError<&str>>(&line) {
            Ok((_, stats)) => initial = visitor(initial, &stats),
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

    #[test]
    fn test_parse_line() {
        let input = r#"enp3s0:  505360    1457    0    0    0     0          0       141   317888    1577    0    0    0     0       0          0"#;
        let (remain, stats) = parse_line::<VerboseError<&str>>(input).unwrap();

        assert_that(&remain).is_equal_to("");
        assert_that(&stats.interface).is_equal_to("enp3s0");
        assert_that(&stats.receive_bytes).is_equal_to(505360);
        assert_that(&stats.receive_packets).is_equal_to(1457);
        assert_that(&stats.transmit_bytes).is_equal_to(317888);
        assert_that(&stats.transmit_packets).is_equal_to(1577);
    }
}
