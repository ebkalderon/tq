use std::str::{self, FromStr};

use pom::char_class::{digit, hex_digit, oct_digit};
use pom::parser::*;

pub fn float<'a>() -> Parser<'a, u8, f64> {
    float_literal() | float_inf_literal() | float_nan_literal()
}

fn float_literal<'a>() -> Parser<'a, u8, f64> {
    let integer = number_sequence(digit);
    let frac = sym(b'.') + number_sequence(digit);
    let exp = one_of(b"eE") + one_of(b"+-").opt() + number_sequence(digit);
    let number = one_of(b"-+").opt() + integer + frac + exp.opt();
    let with_frac = number.collect();

    let integer = number_sequence(digit);
    let frac = sym(b'.') + number_sequence(digit);
    let exp = one_of(b"eE") + one_of(b"+-").opt() + number_sequence(digit);
    let number = one_of(b"-+").opt() + integer + frac.opt() + exp;
    let with_exp = number.collect();

    let float = with_frac | with_exp;
    float
        .convert(str::from_utf8)
        .map(|digits| digits.split('_').collect::<String>())
        .convert(|digits| f64::from_str(&digits))
}

fn float_inf_literal<'a>() -> Parser<'a, u8, f64> {
    let nan = one_of(b"+-").opt() + seq(b"inf");
    nan.collect().convert(str::from_utf8).convert(f64::from_str)
}

fn float_nan_literal<'a>() -> Parser<'a, u8, f64> {
    let nan = one_of(b"+-").opt() + seq(b"nan");
    nan.collect()
        .convert(str::from_utf8)
        .map(|nan| nan.replace("nan", "NaN"))
        .convert(|nan| f64::from_str(&nan))
}

pub fn integer<'a>() -> Parser<'a, u8, i64> {
    integer_bin_literal() | integer_hex_literal() | integer_oct_literal() | integer_literal()
}

fn integer_literal<'a>() -> Parser<'a, u8, i64> {
    let digits = number_sequence(digit);
    let int = one_of(b"+-").opt() + digits;
    int.collect()
        .convert(str::from_utf8)
        .map(|digits| digits.split('_').collect::<String>())
        .convert(|digits| i64::from_str(&digits))
}

fn integer_bin_literal<'a>() -> Parser<'a, u8, i64> {
    let digits = number_sequence(|c| c == b'0' || c == b'1');
    let bin = seq(b"0b") * digits;
    bin.convert(str::from_utf8)
        .map(|digits| digits.split('_').collect::<String>())
        .convert(|digits| i64::from_str_radix(&digits, 2))
}

fn integer_hex_literal<'a>() -> Parser<'a, u8, i64> {
    let digits = number_sequence(hex_digit);
    let hex = seq(b"0x") * digits;
    hex.convert(str::from_utf8)
        .map(|digits| digits.split('_').collect::<String>())
        .convert(|digits| i64::from_str_radix(&digits, 16))
}

fn integer_oct_literal<'a>() -> Parser<'a, u8, i64> {
    let digits = number_sequence(oct_digit);
    let oct = seq(b"0o") * digits;
    oct.convert(str::from_utf8)
        .map(|digits| digits.split('_').collect::<String>())
        .convert(|digits| i64::from_str_radix(&digits, 8))
}

fn number_sequence<'a, F>(predicate: F) -> Parser<'a, u8, &'a [u8]>
where
    F: Clone + Fn(u8) -> bool + 'a,
{
    let digits = is_a(predicate.clone()).repeat(1..);
    let separator = sym(b'_');
    let more = is_a(predicate).repeat(1..);
    let sequence = digits + (separator * more).repeat(0..);
    sequence.collect()
}
