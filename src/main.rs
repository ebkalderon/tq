#![forbid(unsafe_code)]

extern crate tq;
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

    // compile
    //
    // load
    //
    // run
}
