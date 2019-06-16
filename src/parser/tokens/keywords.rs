use pom::char_class::alphanum;
use pom::parser::*;

macro_rules! define_keywords {
    ($($function:ident => $keyword:tt),+) => {
        pub fn keyword<'a>() -> Parser<'a, u8, &'a [u8]> {
            ($($function())|+) - -not_a(alphanum)
        }

        $(
            pub fn $function<'a>() -> Parser<'a, u8, &'a [u8]> {
                seq(stringify!($keyword).as_bytes())
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
