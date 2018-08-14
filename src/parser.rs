const _GRAMMAR: &str = include_str!("filter.pest");

#[derive(Debug, Parser)]
#[grammar = "filter.pest"]
pub struct FilterParser;

#[cfg(test)]
mod tests {
    use super::*;

    use pest::Parser;
    use pest::iterators::Pair;

    #[test]
    fn filter_identity() {
        let pairs = FilterParser::parse(Rule::expr, ".").unwrap();
        let expr: Vec<Pair<Rule>> = pairs.collect();
        assert_eq!(expr.len(), 1);

        let filter = &expr[0];
        assert_eq!(filter.as_rule(), Rule::expr);
        assert_eq!(filter.as_str(), ".");
    }

    #[test]
    fn identifier() {
        let pairs = FilterParser::parse(Rule::expr, ".hello_I-am_Complicated123").unwrap();
        let expr: Vec<Pair<Rule>> = pairs.collect();
        assert_eq!(expr.len(), 1);

        let filter = &expr[0];
        assert_eq!(filter.as_rule(), Rule::expr);
        assert_eq!(filter.as_str(), ".hello_I-am_Complicated123");

        let identifier = filter.clone().into_inner().nth(0).unwrap();
        assert_eq!(identifier.as_rule(), Rule::ident);
        assert_eq!(identifier.as_str(), "hello_I-am_Complicated123");
    }
}
