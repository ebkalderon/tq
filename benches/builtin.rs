use std::fs;
use std::str::FromStr;

use criterion::{criterion_group, criterion_main, Criterion};
use tq::ast::Module;

const BUILTIN_FILE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/builtin.tq");

fn parse_builtin(b: &mut Criterion) {
    let module = fs::read_to_string(BUILTIN_FILE_PATH).expect("builtin.tq not found");

    b.bench_function("parse builtin", move |b| {
        b.iter(|| Module::from_str(&module).expect("Failed to parse builtin module"));
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = parse_builtin
}

criterion_main!(benches);
