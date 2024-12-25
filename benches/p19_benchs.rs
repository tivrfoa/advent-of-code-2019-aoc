use criterion::{black_box, criterion_group, criterion_main, Criterion};

use advent_of_code_2019_aoc::y2024::p19::{p2, p2_same_memo, p2_dp, IN};

fn benchmark_p19(c: &mut Criterion) {
    c.bench_function("p2", |b| b.iter(|| p2(IN)));
    c.bench_function("p2_same_memo", |b| b.iter(|| p2_same_memo(IN)));
    c.bench_function("p2_dp", |b| b.iter(|| p2_dp(IN)));
}

criterion_group!(benches, benchmark_p19);
criterion_main!(benches);
