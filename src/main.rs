extern crate aoc_7_compiler;

const SOURCE: &'static str = include_str!("../source.txt");

fn main() {
    let output = aoc_7_compiler::run_source(SOURCE);
    println!("{:?}", output);

    println!("Wire 'a' signal: {}", output.get("a").unwrap());
}
