use std::time::Duration;

use bf::{
    compiler::x86_64::CompileOptions,
    optimizer::Optimization::{self, *},
    *,
};
use criterion::{criterion_group, criterion_main, Criterion};

const MANDELBROT_BF: &str = include_str!("../programs/mandelbrot.bf");

extern "C" fn putchar(_c: i32) {}
extern "C" fn getchar() -> i32 {
    0
}

fn run(optimizations: &[Optimization]) {
    let program = parser::parse(MANDELBROT_BF).unwrap();
    let optimized = optimizer::optimize(program, optimizations);
    let compiled = compiler::x86_64::compile(
        &optimized,
        CompileOptions {
            putchar: Some(putchar),
            getchar: Some(getchar),
        },
    );
    runtime::native::run(&compiled);
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("mandelbrot");
    group
        .sample_size(20)
        .warm_up_time(Duration::from_secs(10))
        .measurement_time(Duration::from_secs(60));

    group.bench_function("no optimization", |b| b.iter(|| run(&[])));

    group.bench_function("consecutive_inc_dec", |b| {
        b.iter(|| run(&[ConsecutiveIncDec]))
    });

    group.bench_function("mul_loop", |b| b.iter(|| run(&[MulLoop])));

    group.bench_function("consecutive_inc_dec, mul_loop", |b| {
        b.iter(|| run(&[ConsecutiveIncDec, MulLoop]))
    });

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
