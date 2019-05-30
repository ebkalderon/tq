use std::str::{self, FromStr};

use pom::char_class::digit;
use pom::parser::*;
use toml::value::Datetime;

pub fn datetime<'a>() -> Parser<'a, u8, Datetime> {
    let time_str = datetime_literal() | date_literal() | time_literal();
    time_str.convert(str::from_utf8).convert(Datetime::from_str)
}

fn date_literal<'a>() -> Parser<'a, u8, &'a [u8]> {
    let year = is_a(digit).repeat(1..);
    let month = is_a(digit).repeat(2);
    let day = is_a(digit).repeat(2);
    (year + (sym(b'-') + month) + (sym(b'-') + day)).collect()
}

fn time_literal<'a>() -> Parser<'a, u8, &'a [u8]> {
    let hours = is_a(digit).repeat(2);
    let minutes = is_a(digit).repeat(2);
    let seconds = is_a(digit).repeat(2);
    let extra = sym(b'.') + is_a(digit).repeat(6);
    (hours + sym(b':') + minutes + sym(b':') + seconds + extra.opt()).collect()
}

fn datetime_literal<'a>() -> Parser<'a, u8, &'a [u8]> {
    let hours = is_a(digit).repeat(2);
    let minutes = is_a(digit).repeat(2);
    let seconds = is_a(digit).repeat(2);
    let base_time = hours + sym(b':') + minutes + sym(b':') + seconds;

    let hours = is_a(digit).repeat(2);
    let minutes = is_a(digit).repeat(2);
    let offset = hours + sym(b':') + minutes;

    let extra = sym(b'.') + is_a(digit).repeat(6);
    let time = base_time + (sym(b'Z').map(Some) | (extra.opt() * (sym(b'-') - offset).opt())).opt();

    (date_literal() + one_of(b"T ") + time).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn datetime_literals() {
        let full = datetime().parse(b"1979-05-27T07:32:00Z").unwrap();
        assert_eq!(full, "1979-05-27T07:32:00Z".parse().unwrap());

        let full = datetime().parse(b"1979-05-27T00:32:00-07:00").unwrap();
        assert_eq!(full, "1979-05-27T00:32:00-07:00".parse().unwrap());

        let full = datetime()
            .parse(b"1979-05-27T00:32:00.999999-07:00")
            .unwrap();
        assert_eq!(full, "1979-05-27T00:32:00.999999-07:00".parse().unwrap());

        let full = datetime().parse(b"1979-05-27 07:32:00Z").unwrap();
        assert_eq!(full, "1979-05-27 07:32:00Z".parse().unwrap());

        let full_local = datetime().parse(b"1979-05-27T07:32:00").unwrap();
        assert_eq!(full_local, "1979-05-27T07:32:00".parse().unwrap());

        let full_local = datetime().parse(b"1979-05-27T00:32:00.999999").unwrap();
        assert_eq!(full_local, "1979-05-27T00:32:00.999999".parse().unwrap());
    }

    #[test]
    fn date_literals() {
        let date = datetime().parse(b"1979-05-27").unwrap();
        assert_eq!(date, "1979-05-27".parse().unwrap());
    }

    #[test]
    fn time_literals() {
        let basic_time = datetime().parse(b"07:32:00").unwrap();
        assert_eq!(basic_time, "07:32:00".parse().unwrap());

        let precise_time = datetime().parse(b"07:32:00.999999").unwrap();
        assert_eq!(precise_time, "07:32:00.999999".parse().unwrap());
    }
}
