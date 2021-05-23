use blazescript::run_program;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Classes", |b| {
        b.iter(|| {
            run_program(
                String::from("class"),
                String::from(
                    r#"
class Klass {
	var a = [0];

	fun() => {
		soul.a = [69];
		soul.editA(69420);
	}

	fun editA(x) => {
		soul.a = [soul.a[0], x];
		return soul;
	}
}

new Klass().a
                "#,
                ),
            );
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
