use std::fmt::{Debug, Formatter, Result as FmtResult};

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
    cur_step: usize,
}

impl<'s, 't> Debug for Filter<'s, 't> {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.debug_struct("Filter")
            .field("steps", &self.steps)
            .field("cur_step", &self.cur_step)
            .finish()
    }
}

impl<'s, 't> Filter<'s, 't> {
    pub fn new<V>(steps: &'s [SearchStep], values: &'t mut V) -> Self
    where
        V: Iterator<Item = Option<Value>>,
    {
        Filter {
            steps,
            values,
            cur_step: 0,
        }
    }
}
