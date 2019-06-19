use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::combinator::{not, peek};
use nom::sequence::terminated;
use nom::IResult;

macro_rules! define_keywords {
    ($($function:ident => $keyword:tt),+) => {
        pub fn keyword(input: &str) -> IResult<&str, &str> {
            let terms = alt(($($function),+));
            terminated(terms, peek(not(alphanumeric1)))(input)
        }

        $(
            pub fn $function(input: &str) -> IResult<&str, &str> {
                tag(stringify!($keyword))(input)
            }
        )+
    };
}

define_keywords! {
    keyword_and => and,
    keyword_as => as,
    keyword_break => break,
    keyword_catch => catch,
    keyword_def => def,
    keyword_elif => elif,
    keyword_else => else,
    keyword_end => end,
    keyword_empty => empty,
    keyword_foreach => foreach,
    keyword_if => if,
    keyword_import => import,
    keyword_include => include,
    keyword_label => label,
    keyword_module => module,
    keyword_or => or,
    keyword_reduce => reduce,
    keyword_then => then,
    keyword_try => try
}
