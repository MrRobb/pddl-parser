use criterion::{criterion_group, criterion_main, Criterion};
use pddl_parser::domain::Domain;
use std::fs::read_to_string;

fn bench(c: &mut Criterion) {
    let domain_file = read_to_string("tests/domain.pddl").unwrap();
    c.bench_function("Domain::parse", |b| b.iter(|| Domain::parse(&domain_file)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
