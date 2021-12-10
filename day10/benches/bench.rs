#![allow(dead_code)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[path = "../src/main.rs"]
mod main;
fn bench_main(c: &mut Criterion) {
    c.bench_function("part 1 (sample)", |b| {
        b.iter(|| main::part1(black_box(main::read_file("sample"))))
    });
    c.bench_function("part 2 (sample)", |b| {
        b.iter(|| main::part2(black_box(main::read_file("sample"))))
    });
    c.bench_function("part 1 (real)", |b| {
        b.iter(|| main::part1(black_box(main::read_file("input"))))
    });
    c.bench_function("part 2 (real)", |b| {
        b.iter(|| main::part2(black_box(main::read_file("input"))))
    });
}
criterion_group!(benches, bench_main);
criterion_main!(benches);