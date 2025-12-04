use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let start = Instant::now();
    let ans = solve(INPUT);
    let elapsed = start.elapsed();

    println!("{ans}");
    eprintln!("Time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);
}

fn solve(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| max_joltage_12(line.trim()))
        .sum()
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
        // We must choose current digit at index i where i in [start_idx, end_idx]
        // so that we still have enough digits left to fill remaining_slots-1 positions.
        let end_idx = n - remaining_slots; // inclusive

        let mut best_digit: u8 = 0;
        let mut best_pos: usize = start_idx;

        for i in start_idx..=end_idx {
            let d = digits[i];
            if d > best_digit {
                best_digit = d;
                best_pos = i;
                if best_digit == 9 {
                    // can't do better than 9; early exit
                    break;
                }
            }
        }

        result.push(best_digit);
        start_idx = best_pos + 1;
    }

    // Convert 12 digits to u64
    let mut val: u64 = 0;
    for d in result {
        val = val * 10 + d as u64;
    }
    val
}
