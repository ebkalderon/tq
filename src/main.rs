#![forbid(unsafe_code)]

use std::path::PathBuf;
use std::process;

use structopt::StructOpt;
use tq::ast::Filter;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(default_value = ".", parse(from_str = "filter_or_default"))]
    pub filter: String,
    #[structopt(default_value = "Vec::new")]
    pub files: Vec<PathBuf>,
}

fn filter_or_default(s: &str) -> String {
    if s.is_empty() {
        ".".to_string()
    } else {
        s.to_string()
    }
}

fn main() {
    let opt = Opt::from_args();
    let filter: Filter = opt.filter.parse().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    println!("AST: {:?}", filter);
    println!("Serialized: {}", filter);
}
