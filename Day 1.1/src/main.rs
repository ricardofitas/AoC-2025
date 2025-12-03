use std::time::Instant;
use rayon::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    // make N copies of the input so we have enough work
    let copies = 10_000;
    let inputs: Vec<&str> = std::iter::repeat(INPUT).take(copies).collect();

    let start = Instant::now();

    // parallel over many tasks
    let sum: i64 = inputs
        .par_iter()
        .map(|&txt| solve(txt))
        .sum();

    let elapsed = start.elapsed();

    println!("dummy sum: {sum}");
    eprintln!(
        "Parallel time ({} tasks): {:.3} ms",
        copies,
        elapsed.as_secs_f64() * 1000.0
    );
}

fn solve(input: &str) -> i64 {
    // your AoC solver – pure & stateless
    // (for benchmarking you can even add extra work)
    let mut acc = 0i64;
    for line in input.lines() {
        acc += line.len() as i64;
    }
    acc
}