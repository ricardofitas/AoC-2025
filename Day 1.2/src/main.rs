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
    let sum: i64 = inputs.par_iter().map(|&txt| solve(txt)).sum();
    let elapsed_par = start_par.elapsed();

    println!("dummy sum: {sum}");
    eprintln!(
        "Parallel time ({} tasks): {:.3} ms",
        copies,
        elapsed_par.as_secs_f64() * 1000.0
    );
}

fn solve(input: &str) -> i64 {
    let mut pos: i32 = 50;
    let mut hits: i64 = 0;

    for raw_line in input.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let (dir, num_str) = line.split_at(1);
        let dist: i64 = num_str.parse().expect("invalid distance");

        match dir {
            "R" => {
                hits += zeros_on_right_move(pos, dist);
                pos = (pos + (dist % 100) as i32).rem_euclid(100);
            }
            "L" => {
                hits += zeros_on_left_move(pos, dist);
                pos = (pos - (dist % 100) as i32).rem_euclid(100);
            }
            _ => panic!("invalid direction"),
        }
    }

    hits
}

fn zeros_on_right_move(start: i32, dist: i64) -> i64 {
    if dist <= 0 {
        return 0;
    }
    let s = (start.rem_euclid(100)) as i64;
    let mut delta = (100 - s) % 100;
    if delta == 0 {
        delta = 100;
    }
    if dist < delta {
        0
    } else {
        1 + (dist - delta) / 100
    }
}

fn zeros_on_left_move(start: i32, dist: i64) -> i64 {
    if dist <= 0 {
        return 0;
    }
    let s = (start.rem_euclid(100)) as i64;
    let mut delta = s % 100;
    if delta == 0 {
        delta = 100;
    }
    if dist < delta {
        0
    } else {
        1 + (dist - delta) / 100
    }
}
