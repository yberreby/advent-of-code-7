extern crate aoc_7_compiler;

const SOURCE: &'static str = include_str!("../source.txt");
const SOURCE_2: &'static str = include_str!("../source2.txt");

fn main() {
    let output = aoc_7_compiler::run_source(SOURCE);
    // println!("{:?}", output);

    println!("Wire 'a' signal: {}", output.get("a").unwrap());

    // Part two.
    let output2 = aoc_7_compiler::run_source(SOURCE_2);
    println!("New wire 'a' signal after override: {}",
             output2.get("a").unwrap());
}
