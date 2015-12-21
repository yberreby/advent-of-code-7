#![feature(test)]

const SOURCE: &'static str = include_str!("../source.txt");
const SOURCE_2: &'static str = include_str!("../source2.txt");

extern crate test;
extern crate aoc_7_compiler;

use test::Bencher;

#[bench]
fn bench_source_1(b: &mut Bencher) {
    b.iter(|| aoc_7_compiler::run_source(SOURCE));
}


#[bench]
fn bench_source_2(b: &mut Bencher) {
    b.iter(|| aoc_7_compiler::run_source(SOURCE_2));
}
