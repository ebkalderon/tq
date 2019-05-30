#![forbid(unsafe_code)]

use std::env;

use tq::parser::parse_filter;

fn main() {
    let f =
        parse_filter(&env::args().nth(1).unwrap_or(".".into())).expect("Failed to parse filter");
    println!("{:?}", f);
}
