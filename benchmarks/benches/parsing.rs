//! Parsing benchmarks.

use criterion::{Criterion, criterion_group, criterion_main};

fn parsing_benchmark(c: &mut Criterion) {
    c.bench_function("parse_small_doc", |b| {
        b.iter(|| {
            // TODO: Benchmark parsing
        })
    });
}

criterion_group!(benches, parsing_benchmark);
criterion_main!(benches);
