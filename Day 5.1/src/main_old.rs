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
    // Split into two sections: ranges and ids
    let mut sections = input.split("\n\n");

    let ranges_section = sections.next().unwrap_or("");
    let ids_section = sections.next().unwrap_or("");

    // Parse fresh ranges as (start, end), inclusive
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for line in ranges_section.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // format: a-b
        if let Some((a_str, b_str)) = line.split_once('-') {
            let a: u64 = a_str.trim().parse().expect("invalid range start");
            let b: u64 = b_str.trim().parse().expect("invalid range end");
            let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
            ranges.push((lo, hi));
        }
    }

    // Optionally, we could merge overlapping ranges, but it's not required.
    // Simple O(N*M) checking is enough for AoC-scale input.

    let mut fresh_count: u64 = 0;

    for line in ids_section.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let id: u64 = line.parse().expect("invalid id");

        let mut is_fresh = false;
        for (start, end) in &ranges {
            if id >= *start && id <= *end {
                is_fresh = true;
                break;
            }
        }

        if is_fresh {
            fresh_count += 1;
        }
    }

    fresh_count
}
