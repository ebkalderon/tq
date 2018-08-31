use toml::Value;

#[derive(Debug)]
pub enum SearchStep<'a> {
    IndexArray(usize),
    IndexName(&'a str),
    Iterate,
    IndexSlice(Option<usize>, Option<usize>),
}

pub struct Filter<'s, 't> {
    steps: &'s [SearchStep<'s>],
    values: &'t mut Iterator<Item = Option<Value>>,
}

impl<'s, 't> Filter<'s, 't> {
    pub fn new<V: Iterator<Item = Option<Value>>>(
        steps: &'s [SearchStep],
        values: &'t mut V,
    ) -> Self {
        Filter { steps, values }
    }
}
