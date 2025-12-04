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
    let grid: Vec<Vec<u8>> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let h = grid.len();
    if h == 0 {
        return 0;
    }
    let w = grid[0].len();

    let mut accessible = 0u64;

    for y in 0..h {
        for x in 0..w {
            if grid[y][x] != b'@' {
                continue;
            }

            let mut neighbor_rolls = 0;

            for dy in -1i32..=1 {
                for dx in -1i32..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let ny = y as i32 + dy;
                    let nx = x as i32 + dx;
                    if ny < 0 || nx < 0 || ny >= h as i32 || nx >= w as i32 {
                        continue;
                    }
                    if grid[ny as usize][nx as usize] == b'@' {
                        neighbor_rolls += 1;
                    }
                }
            }

            if neighbor_rolls < 4 {
                accessible += 1;
            }
        }
    }

    accessible
}

