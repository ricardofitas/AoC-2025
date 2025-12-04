use std::time::Instant;
use rayon::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    // Single run
    let start = Instant::now();
    let removed = solve(INPUT);
    let elapsed = start.elapsed();

    println!("{removed}");
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
    // Parse grid into Vec<Vec<u8>>
    let mut grid: Vec<Vec<u8>> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let h = grid.len();
    if h == 0 {
        return 0;
    }
    let w = grid[0].len();

    let mut total_removed: u64 = 0;

    loop {
        let mut to_remove: Vec<(usize, usize)> = Vec::new();

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
                    to_remove.push((y, x));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (y, x) in to_remove {
            if grid[y][x] == b'@' {
                grid[y][x] = b'.';
                total_removed += 1;
            }
        }
    }

    total_removed
}
