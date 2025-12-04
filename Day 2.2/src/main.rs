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
    let inputs: Vec<&str> = std::iter::repeat(INPUT).take(copies).collect();

    let start_par = Instant::now();
    let sum: u64 = inputs.par_iter().map(|&txt| solve(txt)).sum();
    let elapsed_par = start_par.elapsed();

    println!("dummy sum: {sum}");
    eprintln!(
        "Parallel time ({} tasks): {:.3} ms",
        copies,
        elapsed_par.as_secs_f64() * 1000.0
    );
}

fn solve(input: &str) -> u64 {
    let line = input.trim();
    let mut sum: u64 = 0;

    for token in line.split(',') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }

        let (start_s, end_s) = token
            .split_once('-')
            .expect("invalid range format");

        let start: u64 = start_s.parse().expect("invalid start");
        let end: u64 = end_s.parse().expect("invalid end");

        for id in start..=end {
            if is_invalid_id(id) {
                sum += id;
            }
        }
    }

    sum
}

/// Invalid ID (Part 2):
/// Made only of some sequence of digits repeated at least twice.
///
/// e.g.
///   12341234      -> "1234" x 2
///   123123123     -> "123"  x 3
///   1212121212    -> "12"   x 5
///   1111111       -> "1"    x 7
fn is_invalid_id(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    if len < 2 {
        return false;
    }

    // Try all repeat counts k >= 2 such that len = k * block_len
    for k in 2..=len {
        if len % k != 0 {
            continue;
        }

        let block_len = len / k;
        let block = &s[..block_len];

        let mut ok = true;
        let mut i = block_len;
        while i < len {
            if &s[i..i + block_len] != block {
                ok = false;
                break;
            }
            i += block_len;
        }

        if ok {
            return true;
        }
    }

    false
}
