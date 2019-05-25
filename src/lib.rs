#![forbid(unsafe_code)]

mod ast;
pub mod parser;

#[cfg(test)]
mod tests {
    use crate::parser::parse_filter;

    #[test]
    fn parse() {
        let string = parse_filter("'''oethu\ntoehueooethuteohueto\\U00001048'''").unwrap();
        println!("{:?}", string);

        let integer = parse_filter("-1_2345").unwrap();
        println!("{:?}", integer);

        let integer = parse_filter("+123_45").unwrap();
        println!("{:?}", integer);

        let float = parse_filter("123.45").unwrap();
        println!("{:?}", float);

        let float = parse_filter("-1_23.1_23").unwrap();
        println!("{:?}", float);

        let float = parse_filter("nan").unwrap();
        println!("{:?}", float);

        let boolean = parse_filter("true").unwrap();
        println!("{:?}", boolean);

        let boolean = parse_filter("false").unwrap();
        println!("{:?}", boolean);

        let string = parse_filter("\"false\"").unwrap();
        println!("{:?}", string);

        let integer = parse_filter("0o123").unwrap();
        println!("{:?}", integer);

        let integer = parse_filter("0x123").unwrap();
        println!("{:?}", integer);

        let integer = parse_filter("0b101").unwrap();
        println!("{:?}", integer);
    }
}
