use std::iter::FromIterator;
use std::{char, u32};

use pom::char_class::hex_digit;
use pom::parser::*;

pub fn string<'a>() -> Parser<'a, u8, String> {
    let char_string = char_string_without(b"\\\"\r\n");
    let basic = sym(b'"') * (char_string | utf16_string() | utf32_string()).repeat(0..) - sym(b'"');

    let char_string = char_string_without(b"\\\"");
    let basic_multi = seq(b"\"\"\"") * (char_string | utf16_string() | utf32_string()).repeat(0..)
        - seq(b"\"\"\"");

    let raw_string = none_of(b"\'\r\n").repeat(1..).convert(String::from_utf8);
    let literal = sym(b'\'') * raw_string.repeat(0..) - sym(b'\'');

    let raw_string = none_of(b"\'").repeat(1..).convert(String::from_utf8);
    let literal_multi = seq(b"'''") * raw_string.repeat(0..) - seq(b"'''");

    let string = basic_multi | basic | literal_multi | literal;
    string.map(|strings| strings.concat())
}

fn char_string_without<'a>(exclude_chars: &'static [u8]) -> Parser<'a, u8, String> {
    let special_char = sym(b'\\')
        | sym(b'"')
        | sym(b'b').map(|_| b'\x08')
        | sym(b'f').map(|_| b'\x0C')
        | sym(b'n').map(|_| b'\n')
        | sym(b'r').map(|_| b'\r')
        | sym(b't').map(|_| b'\t');
    let escape_sequence = sym(b'\\') * special_char;
    let char_string = none_of(exclude_chars) | escape_sequence;
    char_string.repeat(1..).convert(String::from_utf8)
}

fn utf16_string<'a>() -> Parser<'a, u8, String> {
    let utf16_char = seq(b"\\u")
        * is_a(hex_digit)
            .repeat(4)
            .convert(String::from_utf8)
            .convert(|digits| u16::from_str_radix(&digits, 16));

    utf16_char.repeat(1..).map(|chars| {
        char::decode_utf16(chars)
            .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
            .collect::<String>()
    })
}

fn utf32_string<'a>() -> Parser<'a, u8, String> {
    let utf32_char = seq(b"\\U")
        * is_a(hex_digit)
            .repeat(8)
            .convert(String::from_utf8)
            .convert(|digits| u32::from_str_radix(&digits, 16));

    utf32_char
        .map(|c| char::from_u32(c).unwrap_or(char::REPLACEMENT_CHARACTER))
        .repeat(1..)
        .map(String::from_iter)
}
