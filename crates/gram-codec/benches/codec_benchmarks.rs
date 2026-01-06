// Placeholder for codec benchmarks - to be implemented in Phase 6
use criterion::{criterion_group, criterion_main, Criterion};

fn placeholder_benchmark(c: &mut Criterion) {
    c.bench_function("placeholder", |b| b.iter(|| 1 + 1));
}

criterion_group!(benches, placeholder_benchmark);
criterion_main!(benches);
