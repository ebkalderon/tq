#![forbid(unsafe_code)]

extern crate env_logger;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate structopt;
extern crate toml;

use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use failure::ResultExt;
use log::LevelFilter;
use pest::Parser;
use structopt::StructOpt;
use toml::Value;

use parser::{FilterParser, Rule};
use opcode::{Opcode, Opcodes};

mod parser;
mod opcode;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "parse error")]
    Parse,
    #[fail(display = "syntax error")]
    Syntax,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "tq", about = "command-line TOML processor")]
struct Opt {
    /// Use verbose output
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbosity: u8,
    #[structopt(short = "r", long = "read", parse(from_os_str))]
    read: Option<PathBuf>,
    /// Filter to apply to the TOML stream
    filter: String,
}

fn main() {
    let opt = Opt::from_args();
    let verbosity = match opt.verbosity {
        0 => None,
        1 => Some(LevelFilter::Error),
        2 => Some(LevelFilter::Warn),
        3 => Some(LevelFilter::Info),
        4 => Some(LevelFilter::Debug),
        _ => Some(LevelFilter::Trace),
    };

    let mut logger = env_logger::Builder::from_default_env();
    if let Some(level) = verbosity {
        logger.filter_level(level);
    }
    logger.init();

    info!("detected the following options: {:?}", opt);

    let toml: String = match opt.read {
        Some(ref path) => fs::read_to_string(path).unwrap(),
        None => {
            use std::io::{self, Read};
            let mut lines = String::new();
            let stdin = io::stdin();
            stdin.lock().read_to_string(&mut lines).unwrap();
            lines
        }
    };

    let pairs = FilterParser::parse(Rule::expr, &opt.filter);
    trace!("filter parser output: {:#?}", pairs);

    let mut opcodes = Vec::new();
    for filter in pairs.expect("Missing filter") {
        for pair in filter.into_inner() {
            match pair.as_rule() {
                Rule::ident => {
                    opcodes.push(Opcode::IndexName(pair.as_str()));
                }
                Rule::index => {
                    let index_kind = pair.into_inner().nth(0).unwrap();
                    match index_kind.as_rule() {
                        Rule::index_array => {
                            let int = index_kind.into_inner().nth(0).unwrap();
                            opcodes.push(Opcode::IndexArray(int.as_str().parse().unwrap()));
                        }
                        Rule::index_object => {
                            let object = index_kind.into_inner().nth(0).unwrap();
                            opcodes.push(Opcode::IndexName(object.as_str().trim_matches('\"')));
                        }
                        Rule::index_iter => {
                            opcodes.push(Opcode::Iterate);
                        }
                        Rule::index_slice => {
                            if index_kind.as_str() == "[:]" {
                                panic!("Invalid range notation");
                            } else if index_kind.as_str().ends_with(":]") {
                                let begin = index_kind.into_inner().nth(0).unwrap();
                                opcodes.push(Opcode::IndexSlice(
                                        Some(begin.as_str().parse().unwrap()),
                                        None,
                                        ));
                            } else if index_kind.as_str().starts_with("[:") {
                                let end = index_kind.into_inner().nth(0).unwrap();
                                opcodes.push(Opcode::IndexSlice(
                                        None,
                                        Some(end.as_str().parse().unwrap()),
                                        ));
                            } else {
                                let range: Vec<_> = index_kind.into_inner().collect();
                                let begin = &range[0];
                                let end = &range[1];
                                opcodes.push(Opcode::IndexSlice(
                                        Some(begin.as_str().parse().unwrap()),
                                        Some(end.as_str().parse().unwrap()),
                                        ));
                            }
                        }
                        Rule::index_invalid => panic!("Index is not a string, integer, or slice"),
                        _ => panic!("Not an index"),
                    }
                }
                _ => panic!("Unexpected rule"),
            }
        }
    }
    debug!("opcodes generated from filter: {:?}", opcodes);

    let input = Value::from_str(&toml).unwrap();
    let opcodes = Opcodes::new(opcodes);
    let outputs = opcodes.execute(vec![Some(input)].into_iter());

    for output in outputs {
        if let Some(value) = output {
            match value {
                ref tbl if value.is_table() => print!("{}", tbl),
                ref array if value.is_array() => {
                    println!("{}", toml::ser::to_string_pretty(array).unwrap())
                }
                _ => println!("{}", value),
            }
        } else {
            println!("null");
        }
    }
}
