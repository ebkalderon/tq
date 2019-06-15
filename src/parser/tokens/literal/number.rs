use std::f64;
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
        .map(|digits| digits.replace('_', ""))
        .convert(|digits| f64::from_str(&digits))
}

fn float_inf_literal<'a>() -> Parser<'a, u8, f64> {
    let positive = sym(b'+').opt() * seq(b"inf").map(|_| f64::INFINITY);
    let negative = sym(b'-') * seq(b"inf").map(|_| f64::NEG_INFINITY);
    positive | negative
}

fn float_nan_literal<'a>() -> Parser<'a, u8, f64> {
    let positive = sym(b'+').opt() * seq(b"nan").map(|_| f64::NAN);
    let negative = sym(b'-') * seq(b"nan").map(|_| -f64::NAN);
    positive | negative
}

pub fn integer<'a>() -> Parser<'a, u8, i64> {
    integer_bin_literal() | integer_hex_literal() | integer_oct_literal() | integer_literal()
}

fn integer_literal<'a>() -> Parser<'a, u8, i64> {
    let digits = number_sequence(digit);
    let int = one_of(b"+-").opt() + digits;
    int.collect()
        .convert(str::from_utf8)
        .map(|digits| digits.replace('_', ""))
        .convert(|digits| i64::from_str(&digits))
}

fn integer_bin_literal<'a>() -> Parser<'a, u8, i64> {
    let digits = number_sequence(|c| c == b'0' || c == b'1');
    let bin = seq(b"0b") * digits;
    bin.convert(str::from_utf8)
        .map(|digits| digits.replace('_', ""))
        .convert(|digits| i64::from_str_radix(&digits, 2))
}

fn integer_hex_literal<'a>() -> Parser<'a, u8, i64> {
    let digits = number_sequence(hex_digit);
    let hex = seq(b"0x") * digits;
    hex.convert(str::from_utf8)
        .map(|digits| digits.replace('_', ""))
        .convert(|digits| i64::from_str_radix(&digits, 16))
}

fn integer_oct_literal<'a>() -> Parser<'a, u8, i64> {
    let digits = number_sequence(oct_digit);
    let oct = seq(b"0o") * digits;
    oct.convert(str::from_utf8)
        .map(|digits| digits.replace('_', ""))
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

#[cfg(test)]
mod tests {
    use std::f64::{EPSILON, INFINITY, NAN, NEG_INFINITY};

    use float_cmp::ApproxEq;

    use super::*;

    const FLOAT_ULPS: i64 = 2;

    #[test]
    fn integer_literals() {
        let bare_integer = integer().parse(b"123").expect("bare integer failed");
        assert_eq!(bare_integer, 123);

        let negative_integer = integer().parse(b"-42").expect("negative integer failed");
        assert_eq!(negative_integer, -42);

        let positive_integer = integer().parse(b"+1337").expect("positive integer failed");
        assert_eq!(positive_integer, 1337);
    }

    #[test]
    fn integer_special_literals() {
        let bin_literal = integer().parse(b"0b10101").expect("hex literal failed");
        assert_eq!(bin_literal, 0b10101);

        let hex_literal = integer().parse(b"0xabc").expect("hex literal failed");
        assert_eq!(hex_literal, 0xabc);

        let hex_literal = integer().parse(b"0xABC").expect("upper hex literal failed");
        assert_eq!(hex_literal, 0xABC);

        let oct_literal = integer().parse(b"0o123").expect("octal literal failed");
        assert_eq!(oct_literal, 0o123)
    }

    #[test]
    fn float_literals() {
        let bare_frac_float = float().parse(b"1.23").expect("bare float failed");
        assert!(bare_frac_float.approx_eq(&1.23, FLOAT_ULPS as f64 * EPSILON, FLOAT_ULPS));

        let neg_frac_float = float().parse(b"-2.5").expect("negative float failed");
        assert!(neg_frac_float.approx_eq(&-2.5, FLOAT_ULPS as f64 * EPSILON, FLOAT_ULPS));

        let pos_frac_float = float().parse(b"+0.01").expect("positive float failed");
        assert!(pos_frac_float.approx_eq(&0.01, FLOAT_ULPS as f64 * EPSILON, FLOAT_ULPS));

        let bare_exp_float = float().parse(b"6E4").expect("bare exp float failed");
        assert!(bare_exp_float.approx_eq(&6E4, FLOAT_ULPS as f64 * EPSILON, FLOAT_ULPS));

        let neg_exp_float = float().parse(b"12E-3").expect("negative exp float failed");
        assert!(neg_exp_float.approx_eq(&12E-3, FLOAT_ULPS as f64 * EPSILON, FLOAT_ULPS));

        let pos_exp_float = float().parse(b"6E+5").expect("positive exp float failed");
        assert!(pos_exp_float.approx_eq(&6E+5, FLOAT_ULPS as f64 * EPSILON, FLOAT_ULPS));

        let mixed_float = float().parse(b"-3.6E4").expect("frac/exp float failed");
        assert!(mixed_float.approx_eq(&-3.6E4, FLOAT_ULPS as f64 * EPSILON, FLOAT_ULPS));
    }

    #[test]
    fn float_special_literals() {
        let bare_inf_literal = float().parse(b"inf").expect("bare inf literal failed");
        assert!(bare_inf_literal.approx_eq(&INFINITY, FLOAT_ULPS as f64 * EPSILON, FLOAT_ULPS));

        let neg_inf_literal = float().parse(b"-inf").expect("negative inf literal failed");
        assert!(neg_inf_literal.approx_eq(&NEG_INFINITY, FLOAT_ULPS as f64 * EPSILON, FLOAT_ULPS));

        let pos_inf_literal = float().parse(b"+inf").expect("positive inf literal failed");
        assert!(pos_inf_literal.approx_eq(&INFINITY, FLOAT_ULPS as f64 * EPSILON, FLOAT_ULPS));

        let bare_nan_literal = float().parse(b"nan").expect("bare nan literal failed");
        assert!(bare_nan_literal.approx_eq(&NAN, FLOAT_ULPS as f64 * EPSILON, FLOAT_ULPS));

        let neg_nan_literal = float().parse(b"-nan").expect("negative nan literal failed");
        assert!(neg_nan_literal.approx_eq(&-NAN, FLOAT_ULPS as f64 * EPSILON, FLOAT_ULPS));

        let pos_nan_literal = float().parse(b"+nan").expect("positive nan literal failed");
        assert!(pos_nan_literal.approx_eq(&NAN, FLOAT_ULPS as f64 * EPSILON, FLOAT_ULPS));
    }
}
