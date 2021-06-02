use blazex::compile;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_compile(cnt: &'static str) {
    compile(
        String::new(),
        String::from(cnt),
        true,
        false,
        String::new(),
        true,
    )
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Sample", |b| {
        b.iter(|| {
            bench_compile(
                r#"
                var a = 0;
                for i = 1 to 100 step 1 {
                    for j = 100 to 1 step -1 {
                        a += (i * j);
                    }
                }
                a
            "#,
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
