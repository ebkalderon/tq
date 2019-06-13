#![forbid(unsafe_code)]

use std::path::PathBuf;
use std::process;

use structopt::StructOpt;
use tq::parser::parse_filter;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(default_value = ".", parse(from_str = "read_filter"))]
    pub filter: String,
    #[structopt(default_value = "Vec::new")]
    pub files: Vec<PathBuf>,
}

fn read_filter(s: &str) -> String {
    if s.is_empty() {
        ".".to_string()
    } else {
        s.to_string()
    }
}

fn main() {
    let opt = Opt::from_args();
    let filter = match parse_filter(&opt.filter) {
        Ok(filter) => filter,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    println!("AST: {}", filter);
    println!("Serialized: {:?}", filter);
}
