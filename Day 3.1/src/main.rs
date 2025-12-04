use std::time::Instant;
use rayon::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    // Single run
    let start = Instant::now();
    let ans = solve(INPUT);
    let elapsed = start.elapsed();

    println!("{ans}");
    eprintln!("Time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);

    // CPU stress: many copies in parallel
    let copies = 10_000usize;
    let lines: Vec<&str> = INPUT
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect();
    let inputs: Vec<&[&str]> = std::iter::repeat(lines.as_slice())
        .take(copies)
        .collect();

    let start_par = Instant::now();
    let sum: u64 = inputs
        .par_iter()
        .map(|banks| solve_from_lines(banks))
        .sum();
    let elapsed_par = start_par.elapsed();

    println!("dummy sum: {sum}");
    eprintln!(
        "Parallel time ({} tasks): {:.3} ms",
        copies,
        elapsed_par.as_secs_f64() * 1000.0
    );
}

fn solve(input: &str) -> u64 {
    solve_from_lines(
        &input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect::<Vec<_>>(),
    )
}

fn solve_from_lines(lines: &[&str]) -> u64 {
    lines
        .iter()
        .map(|line| max_joltage_for_bank(line.trim()) as u64)
        .sum()
}

/// For one bank (line of digits), find the maximum two-digit number
/// you can form by choosing two positions i < j (order preserved).
fn max_joltage_for_bank(line: &str) -> u32 {
    let digits: Vec<u8> = line
        .chars()
        .map(|c| c.to_digit(10).expect("non-digit") as u8)
        .collect();

    if digits.len() < 2 {
        return 0;
    }

    let n = digits.len();
    // suffix_max[i] = max digit from position i..n-1
    let mut suffix_max = vec![0u8; n];
    suffix_max[n - 1] = digits[n - 1];
    for i in (0..n - 1).rev() {
        suffix_max[i] = suffix_max[i + 1].max(digits[i]);
    }

    let mut best: u32 = 0;
    for i in 0..n - 1 {
        let tens = digits[i];
        let ones = suffix_max[i + 1];
        let val = (tens as u32) * 10 + (ones as u32);
        if val > best {
            best = val;
        }
    }

    best
}
