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
    // Only the first section (before blank line) matters in Part 2.
    let ranges_section = input.split("\n\n").next().unwrap_or("");

    // Parse fresh ranges as (start, end), inclusive.
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for line in ranges_section.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some((a_str, b_str)) = line.split_once('-') {
            let a: u64 = a_str.trim().parse().expect("invalid range start");
            let b: u64 = b_str.trim().parse().expect("invalid range end");
            let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
            ranges.push((lo, hi));
        }
    }

    if ranges.is_empty() {
        return 0;
    }

    // Sort by start, then end
    ranges.sort_by_key(|&(s, e)| (s, e));

    // Merge overlapping ranges
    let mut merged: Vec<(u64, u64)> = Vec::new();
    let mut current = ranges[0];

    for &(s, e) in ranges.iter().skip(1) {
        if s <= current.1 + 1 {
            // Overlaps or touches: extend current
            if e > current.1 {
                current.1 = e;
            }
        } else {
            // Disjoint: push current, start new
            merged.push(current);
            current = (s, e);
        }
    }
    merged.push(current);

    // Sum lengths of merged ranges (inclusive)
    let mut total: u64 = 0;
    for (s, e) in merged {
        total += e - s + 1;
    }

    total
}
