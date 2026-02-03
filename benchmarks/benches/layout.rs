//! Layout benchmarks.

use criterion::{criterion_group, criterion_main, Criterion};

fn layout_benchmark(c: &mut Criterion) {
    c.bench_function("layout_small_doc", |b| {
        b.iter(|| {
            // TODO: Benchmark layout
        })
    });
}

criterion_group!(benches, layout_benchmark);
criterion_main!(benches);
