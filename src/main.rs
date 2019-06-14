#![forbid(unsafe_code)]

use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::{fs, process};

use structopt::StructOpt;
use toml::{self, Value};
use tq::ast::Filter;
use tq::parser::FilterError;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(default_value = ".")]
    pub filter: String,
    #[structopt(parse(from_os_str))]
    pub files: Vec<PathBuf>,
}

impl Opt {
    pub fn process_input(self) -> Result<(), Box<dyn std::error::Error>> {
        let filter = self.parse_filter()?;
        let values = self.read_toml();

        let stdout = io::stdout();
        let mut lock = stdout.lock();
        writeln!(lock, "Filter: {}\n", filter)?;
        writeln!(lock, "TOML:\n")?;

        for value in values {
            let text = value?;
            writeln!(lock, "{}\n", text)?;
        }

        Ok(())
    }

    fn parse_filter(&self) -> Result<Filter, FilterError> {
        if self.filter.trim().is_empty() {
            ".".parse()
        } else {
            self.filter.parse()
        }
    }

    fn read_toml(
        self,
    ) -> impl Iterator<Item = Result<Value, Box<dyn std::error::Error + 'static>>> {
        let values: Box<dyn Iterator<Item = _>> = if self.files.is_empty() {
            let mut input = String::new();
            let stdin = io::stdin();
            let input = stdin.lock().read_to_string(&mut input).map(|_| input);
            Box::from(std::iter::once(input.map_err(Box::from)))
        } else {
            let iter = self
                .files
                .into_iter()
                .map(|s| fs::read_to_string(s).map_err(Box::from));
            Box::from(iter)
        };

        values.map(|s| s.and_then(|s| toml::from_str(&s).map_err(Box::from)))
    }
}

fn main() {
    let opt = Opt::from_args();

    if let Err(e) = opt.process_input() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
