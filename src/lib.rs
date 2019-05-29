#![forbid(unsafe_code)]

mod ast;
pub mod parser;

#[cfg(test)]
mod tests {
    use crate::parser::parse_filter;

    #[test]
    fn parse() {
        let boolean = parse_filter("true").unwrap();
        println!("{:?}", boolean);

        let boolean = parse_filter("false").unwrap();
        println!("{:?}", boolean);

        let identity = parse_filter(".").unwrap();
        println!("{:?}", identity);

        let recurse = parse_filter("..").unwrap();
        println!("{:?}", recurse);

        let fields = parse_filter("[13]").unwrap();
        println!("{:?}", fields);

        let fields = parse_filter("{ birthday = [1, 2, 3], '1234' = true }").unwrap();
        println!("{:?}", fields);
    }
}
