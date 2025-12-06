use std::time::Instant;
use rayon::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    // Single run (like before)
    let start = Instant::now();
    let ans = solve(INPUT);
    let elapsed = start.elapsed();

    println!("{ans}");
    eprintln!("Time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);

    // CPU stress: run the full solver many times in parallel
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
    // Collect lines and compute width
    let mut lines: Vec<&str> = Vec::new();
    let mut width: usize = 0;

    for line in input.lines() {
        lines.push(line);
        if line.len() > width {
            width = line.len();
        }
    }

    let height = lines.len();
    if height == 0 {
        return 0;
    }

    // Build a grid of chars, padding with spaces
    let mut grid: Vec<Vec<char>> = Vec::with_capacity(height);
    for &line in &lines {
        let mut row: Vec<char> = line.chars().collect();
        if row.len() < width {
            row.resize(width, ' ');
        }
        grid.push(row);
    }

    // Find operator row (bottom-most row with '+' or '*')
    let mut op_row: Option<usize> = None;
    for r in (0..height).rev() {
        if grid[r].iter().any(|&ch| ch == '+' || ch == '*') {
            op_row = Some(r);
            break;
        }
    }
    let op_row = match op_row {
        Some(r) => r,
        None => return 0,
    };

    // Which columns are completely blank?
    let mut col_blank = vec![true; width];
    for c in 0..width {
        for r in 0..height {
            if grid[r][c] != ' ' {
                col_blank[c] = false;
                break;
            }
        }
    }

    // Find contiguous non-blank column ranges = problems
    let mut problems: Vec<(usize, usize)> = Vec::new();
    let mut in_group = false;
    let mut group_start = 0usize;

    for c in 0..width {
        if !col_blank[c] {
            if !in_group {
                in_group = true;
                group_start = c;
            }
        } else if in_group {
            problems.push((group_start, c - 1));
            in_group = false;
        }
    }
    if in_group {
        problems.push((group_start, width - 1));
    }

    // Evaluate each problem
    let mut grand_total: i64 = 0;

    for (start_col, end_col) in problems {
        // Find operator in operator row
        let mut op: Option<char> = None;
        for c in start_col..=end_col {
            let ch = grid[op_row][c];
            if ch == '+' || ch == '*' {
                op = Some(ch);
                break;
            }
        }
        let op = match op {
            Some(ch) => ch,
            None => continue,
        };

        // Collect numbers above
        let mut nums: Vec<i64> = Vec::new();
        for r in 0..op_row {
            let mut s = String::new();
            for c in start_col..=end_col {
                let ch = grid[r][c];
                if ch.is_ascii_digit() {
                    s.push(ch);
                }
            }
            if !s.is_empty() {
                let val: i64 = s.parse().expect("invalid integer in problem");
                nums.push(val);
            }
        }

        if nums.is_empty() {
            continue;
        }

        let val = match op {
            '+' => nums.iter().sum::<i64>(),
            '*' => nums.iter().product::<i64>(),
            _ => continue,
        };

        grand_total += val;
    }

    grand_total
}
