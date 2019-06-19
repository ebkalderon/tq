use nom::combinator::map;
use nom::sequence::{pair, preceded, terminated};
use nom::IResult;

use super::tokens;
use crate::ast::tokens::Label;

pub fn label_decl(input: &str) -> IResult<&str, Label> {
    let keyword = pair(tokens::keyword_label, tokens::space);
    let variable = terminated(tokens::variable, tokens::space);
    map(preceded(keyword, variable), Label::from)(input)
}

pub fn label_break(input: &str) -> IResult<&str, Label> {
    let keyword = pair(tokens::keyword_break, tokens::space);
    let variable = terminated(tokens::variable, tokens::space);
    map(preceded(keyword, variable), Label::from)(input)
}
