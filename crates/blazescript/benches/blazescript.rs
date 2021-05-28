use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Sample", |b| b.iter(|| println!("No benchmark!")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
