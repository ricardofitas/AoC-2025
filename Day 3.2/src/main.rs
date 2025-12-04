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
    let sum: u64 = inputs.par_iter().map(|banks| solve_from_lines(banks)).sum();
    let elapsed_par = start_par.elapsed();

    println!("dummy sum: {sum}");
    eprintln!(
        "Parallel time ({} tasks): {:.3} ms",
        copies,
        elapsed_par.as_secs_f64() * 1000.0
    );
}

fn solve(input: &str) -> u64 {
    let lines: Vec<&str> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    solve_from_lines(&lines)
}

fn solve_from_lines(lines: &[&str]) -> u64 {
    lines.iter().map(|line| max_joltage_12(line.trim())).sum()
}

/// For one bank: choose exactly 12 digits (as a subsequence) so that the
/// resulting 12-digit number is as large as possible.
fn max_joltage_12(line: &str) -> u64 {
    const K: usize = 12;
    let digits: Vec<u8> = line
        .chars()
        .map(|c| c.to_digit(10).expect("non-digit") as u8)
        .collect();

    let n = digits.len();
    assert!(
        n >= K,
        "Bank has fewer than 12 digits: {} (line: {})",
        n,
        line
    );

    let mut result: Vec<u8> = Vec::with_capacity(K);
    let mut start_idx: usize = 0;

    for pos in 0..K {
        let remaining_slots = K - pos;
        let end_idx = n - remaining_slots; // inclusive

        let mut best_digit: u8 = 0;
        let mut best_pos: usize = start_idx;

        for i in start_idx..=end_idx {
            let d = digits[i];
            if d > best_digit {
                best_digit = d;
                best_pos = i;
                if best_digit == 9 {
                    break; // can't do better than 9
                }
            }
        }

        result.push(best_digit);
        start_idx = best_pos + 1;
    }

    let mut val: u64 = 0;
    for d in result {
        val = val * 10 + d as u64;
    }
    val
}
