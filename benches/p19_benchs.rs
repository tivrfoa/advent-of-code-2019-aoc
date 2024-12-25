use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Import your functions from src/y2024/p19.rs
use advent_of_code_2019_aoc::y2024::p19::{p2, p2_dp}; // Replace `my_project` with your crate name

fn benchmark_p19(c: &mut Criterion) {
    let input = black_box(100); // Example input; replace with appropriate test input
    c.bench_function("p2", |b| b.iter(|| p2(input)));
    c.bench_function("p2_dp", |b| b.iter(|| p2_dp(input)));
}

criterion_group!(benches, benchmark_p19);
criterion_main!(benches);
