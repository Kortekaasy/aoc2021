#![allow(dead_code)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[path = "../src/main.rs"]
mod main;
fn bench_main(c: &mut Criterion) {
    c.bench_function("part 1 (sample)", |b| {
        let (template, rules) = main::parse_input(main::read_file("sample"));
        b.iter(|| main::part1(black_box(&template), black_box(&rules)))
    });
    c.bench_function("part 2 (sample)", |b| {
        let (template, rules) = main::parse_input(main::read_file("sample"));
        b.iter(|| main::part2(black_box(&template), black_box(&rules)))
    });
    c.bench_function("part 1 (real)", |b| {
        let (template, rules) = main::parse_input(main::read_file("input"));
        b.iter(|| main::part1(black_box(&template), black_box(&rules)))
    });
    c.bench_function("part 2 (real)", |b| {
        let (template, rules) = main::parse_input(main::read_file("input"));
        b.iter(|| main::part2(black_box(&template), black_box(&rules)))
    });
}
criterion_group!(benches, bench_main);
criterion_main!(benches);