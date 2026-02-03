//! Parsing benchmarks.

use criterion::{criterion_group, criterion_main, Criterion};

fn parsing_benchmark(c: &mut Criterion) {
    c.bench_function("parse_small_doc", |b| {
        b.iter(|| {
            // TODO: Benchmark parsing
        })
    });
}

criterion_group!(benches, parsing_benchmark);
criterion_main!(benches);
