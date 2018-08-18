use std::iter::FromIterator;

use toml::Value;

use super::Error;

#[derive(Debug)]
pub enum Opcode<'a> {
    IndexArray(usize),
    IndexName(&'a str),
    Iterate,
    IndexSlice(Option<usize>, Option<usize>),
}

#[derive(Debug)]
pub struct Opcodes<'a>(Vec<Opcode<'a>>);

impl<'a> Opcodes<'a> {
    pub fn new<I: IntoIterator<Item = Opcode<'a>>>(opcodes: I) -> Self {
        Opcodes(opcodes.into_iter().collect())
    }

    pub fn execute<I>(self, toml: I) -> Vec<Option<Value>>
    where
        I: IntoIterator<Item = Option<Value>> + 'static
    {
        let Opcodes(opcodes) = self;
        execute(&opcodes, &mut toml.into_iter())
    }
}

fn execute(opcodes: &[Opcode], toml: &mut Iterator<Item = Option<Value>>) -> Vec<Option<Value>> {
    if let Some(opcode) = opcodes.first() {
        match *opcode {
            Opcode::IndexArray(idx) => {
                let mut toml = index_array(idx, toml);
                let remaining = &opcodes[1..];
                execute(remaining, &mut toml)
            }
            Opcode::Iterate => {
                let mut toml = iterate(toml);
                let remaining = &opcodes[1..];
                execute(remaining, &mut toml)
            }
            Opcode::IndexName(name) => {
                let mut toml = index_name(name, toml);
                let remaining = &opcodes[1..];
                execute(remaining, &mut toml)
            }
            Opcode::IndexSlice(begin, end) => {
                let mut toml = toml.into_iter().map(|value| {
                    value.map(|value| {
                        if value.is_array() {
                            let inner = value.as_array().unwrap();
                            let begin = begin.unwrap_or(0);
                            let end = if end.unwrap_or(inner.len()) >= inner.len() {
                                inner.len()
                            } else {
                                end.unwrap_or(inner.len())
                            };

                            Value::Array(Vec::from(&inner[begin..end]))
                        } else if value.is_str() {
                            let inner = value.as_str().unwrap();
                            let begin = begin.unwrap_or(0);
                            let end = if end.unwrap_or(inner.len()) >= inner.len() {
                                inner.len()
                            } else {
                                end.unwrap_or(inner.len())
                            };

                            Value::String(inner[begin..end].to_string())
                        } else {
                            panic!(format!("Cannot slice index a {}", value.type_str()));
                        }
                    })
                });

                let remaining = &opcodes[1..];
                execute(remaining, &mut toml)
            },
        }
    } else {
        toml.collect()
    }
}

fn index_array<'a>(index: usize, toml: &'a mut Iterator<Item = Option<Value>>) -> impl Iterator<Item = Option<Value>> + 'a {
    toml.into_iter().map(move |opt| {
        opt.map(|value| {
            if value.is_array() {
                value
            } else if value.is_str() {
                panic!("Cannot index string with number");
            } else {
                panic!(format!("Cannot index {} with number", value.type_str()));
            }
        }).and_then(|value| value.get(index).cloned())
    })
}

fn index_name<'a>(name: &'a str, toml: &'a mut Iterator<Item = Option<Value>>) -> impl Iterator<Item = Option<Value>> + 'a {
    toml
        .into_iter()
        .map(move |value| value.and_then(|v| v.get(name).cloned()))
}

fn iterate(toml: &mut Iterator<Item = Option<Value>>) -> impl Iterator<Item = Option<Value>> {
    let mut values = Vec::new();

    for v in toml {
        if let Some(value) = v {
            if value.is_array() {
                values.extend(
                    value.as_array().cloned().unwrap().into_iter().map(Some),
                );
            } else if value.is_table() {
                values
                    .extend(value.as_table().unwrap().values().cloned().map(Some));
            } else {
                panic!("Only arrays and objects can be turned into iterators");
            }
        }
    }   

    values.into_iter()
}
