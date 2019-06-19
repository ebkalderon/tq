use std::str::{self, FromStr};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, one_of};
use nom::combinator::{map_res, opt, recognize, verify};
use nom::sequence::{pair, tuple};
use nom::IResult;
use toml::value::Datetime;

use super::take_n;

pub fn datetime(input: &str) -> IResult<&str, Datetime> {
    let time_str = alt((datetime_literal, date_literal, time_literal));
    map_res(time_str, Datetime::from_str)(input)
}

fn date_literal(input: &str) -> IResult<&str, &str> {
    let year = digit1;
    let month = verify(take_n(2), |c: &str| c.chars().all(|c| c.is_ascii_digit()));
    let day = verify(take_n(2), |c: &str| c.chars().all(|c| c.is_ascii_digit()));
    recognize(tuple((year, char('-'), month, char('-'), day)))(input)
}

fn time_literal(input: &str) -> IResult<&str, &str> {
    let hours = verify(take_n(2), |c: &str| c.chars().all(|c| c.is_ascii_digit()));
    let minutes = verify(take_n(2), |c: &str| c.chars().all(|c| c.is_ascii_digit()));
    let seconds = verify(take_n(2), |c: &str| c.chars().all(|c| c.is_ascii_digit()));
    let extra = verify(take_n(6), |c: &str| c.chars().all(|c| c.is_ascii_digit()));
    let time = tuple((hours, char(':'), minutes, char(':'), seconds));
    recognize(pair(time, opt(pair(char('.'), extra))))(input)
}

fn datetime_literal(input: &str) -> IResult<&str, &str> {
    let hours = verify(take_n(2), |c: &str| c.chars().all(|c| c.is_ascii_digit()));
    let minutes = verify(take_n(2), |c: &str| c.chars().all(|c| c.is_ascii_digit()));
    let seconds = verify(take_n(2), |c: &str| c.chars().all(|c| c.is_ascii_digit()));
    let base_time = tuple((hours, char(':'), minutes, char(':'), seconds));

    let hours = verify(take_n(2), |c: &str| c.chars().all(|c| c.is_ascii_digit()));
    let minutes = verify(take_n(2), |c: &str| c.chars().all(|c| c.is_ascii_digit()));
    let offset = tuple((char('-'), hours, char(':'), minutes));

    let extra = verify(take_n(6), |c: &str| c.chars().all(|c| c.is_ascii_digit()));
    let zone = alt((
        recognize(tag("Z")),
        recognize(pair(opt(pair(char('.'), extra)), opt(offset))),
    ));
    let full_time = pair(base_time, opt(zone));

    recognize(tuple((date_literal, one_of("T "), full_time)))(input)
}

#[cfg(test)]
mod tests {
    use nom::combinator::all_consuming;

    use super::*;

    macro_rules! assert_datetime_parses {
        ($str:expr) => {
            let (_, full) = all_consuming(datetime)($str).expect("datetime() parse failed");
            assert_eq!(full, $str.parse().expect("toml::Datetime parse failed"));
        };
    }

    #[test]
    fn datetime_literals() {
        assert_datetime_parses!("1979-05-27T07:32:00Z");
        assert_datetime_parses!("1979-05-27T00:32:00-07:00");
        assert_datetime_parses!("1979-05-27 07:32:00Z");
        assert_datetime_parses!("1979-05-27T07:32:00");
        assert_datetime_parses!("1979-05-27T00:32:00.999999");
    }

    #[test]
    fn date_literals() {
        assert_datetime_parses!("1979-05-27");
    }

    #[test]
    fn time_literals() {
        assert_datetime_parses!("07:32:00");
        assert_datetime_parses!("07:32:00.999999");
    }
}
