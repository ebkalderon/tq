use std::char;
use std::iter::FromIterator;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, none_of};
use nom::combinator::{map, map_res, verify};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, preceded};
use nom::IResult;

use super::take_n;

pub fn string(input: &str) -> IResult<&str, String> {
    let char_string = char_string_without("\\\"\r\n");
    let str_body = alt((char_string, utf16_string, utf32_string));
    let basic = delimited(char('"'), many0(str_body), char('"'));

    let char_string = char_string_without("\\\"");
    let str_body = alt((char_string, utf16_string, utf32_string));
    let basic_multi = delimited(tag("\"\"\""), many0(str_body), tag("\"\"\""));

    let raw_string = map(many1(none_of("\'\r\n")), String::from_iter);
    let literal = delimited(char('\''), many0(raw_string), char('\''));

    let raw_string = map(many1(none_of("\'")), String::from_iter);
    let literal_multi = delimited(tag("'''"), many0(raw_string), tag("'''"));

    let string = alt((basic_multi, basic, literal_multi, literal));
    map(string, |strings| strings.concat())(input)
}

fn char_string_without(exclude_chars: &'static str) -> impl Fn(&str) -> IResult<&str, String> {
    move |input| {
        let special_char = alt((
            char('\\'),
            char('"'),
            char('b'),
            map(char('b'), |_| '\x08'),
            map(char('f'), |_| '\x0C'),
            map(char('n'), |_| '\n'),
            map(char('r'), |_| '\r'),
            map(char('t'), |_| '\t'),
        ));

        let escape_sequence = preceded(char('\\'), special_char);
        let char_string = alt((none_of(exclude_chars), escape_sequence));
        map(many1(char_string), String::from_iter)(input)
    }
}

fn utf16_string(input: &str) -> IResult<&str, String> {
    let code = preceded(tag("\\u"), verify(take_n(4), all_chars_hexdigit));
    let utf16_char = map_res(code, |digits| u16::from_str_radix(digits, 16));
    let decode = |chars| {
        char::decode_utf16(chars)
            .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
            .collect::<String>()
    };

    map(many1(utf16_char), decode)(input)
}

fn utf32_string(input: &str) -> IResult<&str, String> {
    let code = preceded(tag("\\U"), verify(take_n(8), all_chars_hexdigit));
    let utf32_char = map_res(code, |digits| u32::from_str_radix(&digits, 16));
    let decode = |c| char::from_u32(c).unwrap_or(char::REPLACEMENT_CHARACTER);
    map(many1(map(utf32_char, decode)), String::from_iter)(input)
}

fn all_chars_hexdigit(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use nom::combinator::all_consuming;

    use super::*;

    #[test]
    fn basic_strings() {
        let (_, simple) = all_consuming(string)("\"    hello there  \"").unwrap();
        assert_eq!(simple, "    hello there  ");

        let (_, escaped_utf8) = all_consuming(string)("\"  \\u1048 \"").unwrap();
        assert_eq!(escaped_utf8, "  \u{1048} ");

        let (_, escaped_utf32) = all_consuming(string)("\"\\U00001048\"").unwrap();
        assert_eq!(escaped_utf32, "\u{1048}");
    }

    #[test]
    fn raw_strings() {
        let (_, simple) = all_consuming(string)("'    hello there  '").unwrap();
        assert_eq!(simple, "    hello there  ");

        let (_, escaped_utf8) = all_consuming(string)("'  \\u1048 '").unwrap();
        assert_eq!(escaped_utf8, "  \\u1048 ");

        let (_, escaped_utf32) = all_consuming(string)("'\\U00001048'").unwrap();
        assert_eq!(escaped_utf32, "\\U00001048");
    }

    #[test]
    fn basic_multiline_strings() {
        let (_, simple) = all_consuming(string)("\"\"\"one\n\\n two\\r\t\"\"\"").unwrap();
        assert_eq!(simple, "one\n\n two\r\t");

        let (_, escaped_utf8) = all_consuming(string)("\"\"\"  \\u1048 \"\"\"").unwrap();
        assert_eq!(escaped_utf8, "  \u{1048} ");

        let (_, escaped_utf32) = all_consuming(string)("\"\"\"\\U00001048\"\"\"").unwrap();
        assert_eq!(escaped_utf32, "\u{1048}");
    }

    #[test]
    fn raw_multiline_strings() {
        let (_, simple) = all_consuming(string)("'''one\n\\n two\\r\t'''").unwrap();
        assert_eq!(simple, "one\n\\n two\\r\t");

        let (_, escaped_utf8) = all_consuming(string)("'''  \\u1048 '''").unwrap();
        assert_eq!(escaped_utf8, "  \\u1048 ");

        let (_, escaped_utf32) = all_consuming(string)("'''\\U00001048'''").unwrap();
        assert_eq!(escaped_utf32, "\\U00001048");
    }
}
