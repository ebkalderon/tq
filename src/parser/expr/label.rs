use pom::parser::*;

use super::tokens;
use crate::ast::tokens::Label;

pub fn label_decl<'a>() -> Parser<'a, u8, Label> {
    let keyword = tokens::keyword_label() + tokens::space();
    let name = tokens::variable() - tokens::space();
    let label = (keyword * name).map(Label::from);
    tokens::space() * label - tokens::space()
}

pub fn label_break<'a>() -> Parser<'a, u8, Label> {
    let keyword = tokens::keyword_break() + tokens::space();
    let name = tokens::variable();
    (keyword * name).map(Label::from)
}
