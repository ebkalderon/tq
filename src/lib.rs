#![forbid(unsafe_code)]
#![recursion_limit = "128"]

pub mod ast;
pub mod parser;

#[cfg(test)]
mod tests {
    use crate::parser::parse_filter;

    #[test]
    fn parse() {
        let boolean = parse_filter("true").unwrap();
        println!("{}", boolean);

        let boolean = parse_filter("false").unwrap();
        println!("{}", boolean);

        let identity = parse_filter(".foo.bar[\"hello\"][] | 12 + ..").unwrap();
        println!("{}", identity);

        let recurse = parse_filter("..").unwrap();
        println!("{}", recurse);

        let fields = parse_filter("[13, 12, blah]").unwrap();
        println!("{}", fields);

        let fields = parse_filter("{ birthday = [1, 2, 3], hello = true }").unwrap();
        println!("{}", fields);

        let function = parse_filter("def thing: def foo($f): .; foo(.); thing | thing").unwrap();
        println!("{}", function);
    }
}
