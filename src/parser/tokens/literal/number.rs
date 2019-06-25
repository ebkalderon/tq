use std::f64;
use std::str::{self, FromStr};

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{char, digit1, hex_digit1, oct_digit1, one_of};
use nom::combinator::{map, map_res, opt, recognize};
use nom::multi::many0;
use nom::sequence::{pair, preceded, tuple};
use nom::IResult;

pub fn float(input: &str) -> IResult<&str, f64> {
    alt((float_literal, float_inf_literal, float_nan_literal))(input)
}

fn float_literal(input: &str) -> IResult<&str, f64> {
    let frac = pair(char('.'), digit_sequence);
    let exp = tuple((one_of("eE"), opt(one_of("+-")), digit_sequence));
    let number = tuple((opt(one_of("-+")), digit_sequence, frac, opt(exp)));
    let with_frac = recognize(number);

    let frac = pair(char('.'), digit_sequence);
    let exp = tuple((one_of("eE"), opt(one_of("+-")), digit_sequence));
    let number = tuple((opt(one_of("-+")), digit_sequence, opt(frac), exp));
    let with_exp = recognize(number);

    let float = alt((with_frac, with_exp));
    map_res(float, |s| f64::from_str(&s.replace('_', "")))(input)
}

fn float_inf_literal(input: &str) -> IResult<&str, f64> {
    let positive = preceded(opt(char('+')), map(tag("inf"), |_| f64::INFINITY));
    let negative = preceded(char('-'), map(tag("inf"), |_| f64::NEG_INFINITY));
    alt((positive, negative))(input)
}

fn float_nan_literal(input: &str) -> IResult<&str, f64> {
    let positive = preceded(opt(char('+')), map(tag("nan"), |_| f64::NAN));
    let negative = preceded(char('-'), map(tag("nan"), |_| -f64::NAN));
    alt((positive, negative))(input)
}

pub fn integer(input: &str) -> IResult<&str, i64> {
    alt((
        integer_bin_literal,
        integer_hex_literal,
        integer_oct_literal,
        integer_literal,
    ))(input)
}

fn integer_literal(input: &str) -> IResult<&str, i64> {
    let int = recognize(pair(opt(one_of("+-")), digit_sequence));
    map_res(int, |s: &str| i64::from_str(&s.replace('_', "")))(input)
}

fn integer_bin_literal(input: &str) -> IResult<&str, i64> {
    let prefix = take_while1(|c| c == '0' || c == '1');
    let suffix = take_while1(|c| c == '0' || c == '1');
    let digits = pair(prefix, many0(preceded(char('_'), suffix)));
    let bin = preceded(tag("0b"), recognize(digits));
    map_res(bin, |s: &str| i64::from_str_radix(&s.replace('_', ""), 2))(input)
}

fn integer_hex_literal(input: &str) -> IResult<&str, i64> {
    let digits = pair(hex_digit1, many0(preceded(char('_'), hex_digit1)));
    let hex = preceded(tag("0x"), recognize(digits));
    map_res(hex, |s: &str| i64::from_str_radix(&s.replace('_', ""), 16))(input)
}

fn integer_oct_literal(input: &str) -> IResult<&str, i64> {
    let digits = pair(oct_digit1, many0(preceded(char('_'), oct_digit1)));
    let oct = preceded(tag("0o"), recognize(digits));
    map_res(oct, |s: &str| i64::from_str_radix(&s.replace('_', ""), 8))(input)
}

fn digit_sequence(input: &str) -> IResult<&str, &str> {
    recognize(pair(digit1, many0(preceded(char('_'), digit1))))(input)
}

#[cfg(test)]
mod tests {
    use std::f64::{INFINITY, NAN, NEG_INFINITY};

    use float_cmp::{approx_eq, ApproxEq};
    use nom::combinator::all_consuming;

    use super::*;

    #[test]
    fn integer_literals() {
        let (_, bare_integer) = all_consuming(integer)("123").expect("bare integer failed");
        assert_eq!(bare_integer, 123);

        let (_, negative_integer) = all_consuming(integer)("-42").expect("negative integer failed");
        assert_eq!(negative_integer, -42);

        let (_, positive_integer) = all_consuming(integer)("+21").expect("positive integer failed");
        assert_eq!(positive_integer, 21);
    }

    #[test]
    fn integer_special_literals() {
        let (_, bin_literal) = all_consuming(integer)("0b10101").expect("bin literal failed");
        assert_eq!(bin_literal, 0b10101);

        let (_, hex_literal) = all_consuming(integer)("0xabc").expect("hex literal failed");
        assert_eq!(hex_literal, 0xabc);

        let (_, hex_literal) = all_consuming(integer)("0xABC").expect("upper hex literal failed");
        assert_eq!(hex_literal, 0xABC);

        let (_, oct_literal) = all_consuming(integer)("0o123").expect("octal literal failed");
        assert_eq!(oct_literal, 0o123)
    }

    #[test]
    fn float_literals() {
        let (_, bare_frac_float) = all_consuming(float)("1.23").expect("bare float failed");
        approx_eq!(f64, bare_frac_float, 1.23);

        let (_, neg_frac_float) = all_consuming(float)("-2.5").expect("negative float failed");
        approx_eq!(f64, neg_frac_float, -2.5);

        let (_, pos_frac_float) = all_consuming(float)("+0.01").expect("positive float failed");
        approx_eq!(f64, pos_frac_float, 0.01);

        let (_, bare_exp_float) = all_consuming(float)("6E4").expect("bare exp float failed");
        approx_eq!(f64, bare_exp_float, 6E4);

        let (_, neg_exp_float) = all_consuming(float)("12E-3").expect("negative exp float failed");
        approx_eq!(f64, neg_exp_float, 12E-3);

        let (_, pos_exp_float) = all_consuming(float)("6E+5").expect("positive exp float failed");
        approx_eq!(f64, pos_exp_float, 6E+5);

        let (_, mixed_float) = all_consuming(float)("-3.6E4").expect("frac/exp float failed");
        approx_eq!(f64, mixed_float, -3.6E4);
    }

    #[test]
    fn float_special_literals() {
        let (_, bare_inf_literal) = all_consuming(float)("inf").expect("bare inf failed");
        approx_eq!(f64, bare_inf_literal, INFINITY);

        let (_, neg_inf_literal) = all_consuming(float)("-inf").expect("negative inf failed");
        approx_eq!(f64, neg_inf_literal, NEG_INFINITY);

        let (_, pos_inf_literal) = all_consuming(float)("+inf").expect("positive inf failed");
        approx_eq!(f64, pos_inf_literal, INFINITY);

        let (_, bare_nan_literal) = all_consuming(float)("nan").expect("bare nan failed");
        approx_eq!(f64, bare_nan_literal, NAN);

        let (_, neg_nan_literal) = all_consuming(float)("-nan").expect("negative nan failed");
        approx_eq!(f64, neg_nan_literal, -NAN);

        let (_, pos_nan_literal) = all_consuming(float)("+nan").expect("positive nan failed");
        approx_eq!(f64, pos_nan_literal, NAN);
    }
}
