use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    // start timer
    let start = Instant::now();

    // do the actual work
    let answer = solve(INPUT);

    // stop timer
    let elapsed = start.elapsed();

    // print answer (for AoC)
    println!("{answer}");

    // print time to stderr so it doesn't mess with AoC output
    eprintln!(
        "Time: {:?} (~{:.3} ms)",
        elapsed,
        elapsed.as_secs_f64() * 1000.0
    );
}

fn solve(input: &str) -> i64 {
    let mut pos: i32 = 50;
    let mut count: i64 = 0;

    for raw_line in input.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let (dir, num_str) = line.split_at(1);
        let dist: i32 = num_str.parse().expect("invalid distance");

        match dir {
            "L" => pos = (pos - dist).rem_euclid(100),
            "R" => pos = (pos + dist).rem_euclid(100),
            _ => panic!("invalid direction"),
        }

        if pos == 0 {
            count += 1;
        }
    }

    count
}
