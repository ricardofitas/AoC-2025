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
        .map(|line| max_joltage_for_bank(line.trim()) as u64)
        .sum()
}

/// For one bank (line of digits), find the maximum two-digit number
/// you can form by choosing two positions i < j and using digits[i] as tens,
/// digits[j] as ones (order preserved, no rearranging).
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

    // For each position i as tens, best ones digit is max over positions > i
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
