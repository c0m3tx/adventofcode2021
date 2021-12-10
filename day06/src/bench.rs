use criterion::{criterion_group, criterion_main, Criterion};

mod fishes;
use fishes::{input_to_array, iterative_step, optimized_step};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("iterative version", |b| {
        b.iter(|| {
            let mut school = vec![3, 4, 3, 1, 2];

            for _ in 0..200 {
                school = iterative_step(school);
            }
        })
    });
    c.bench_function("optimized version", |b| {
        b.iter(|| {
            let mut school = input_to_array(&vec![3, 4, 3, 1, 2]);

            for _ in 0..200 {
                school = optimized_step(school);
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
