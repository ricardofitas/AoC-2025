use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let start = Instant::now();
    let ans = solve(INPUT);
    let elapsed = start.elapsed();

    println!("{ans}");
    eprintln!("Time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);
}

fn solve(input: &str) -> i64 {
    // Collect lines and compute width
    let mut lines: Vec<&str> = Vec::new();
    let mut width: usize = 0;

    for line in input.lines() {
        // Don't trim, we need spaces
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
        None => return 0, // no operators?! then nothing to do
    };

    // Determine which columns are completely blank (all spaces)
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
    let mut group_start: usize = 0;

    for c in 0..width {
        if !col_blank[c] {
            if !in_group {
                in_group = true;
                group_start = c;
            }
        } else {
            if in_group {
                problems.push((group_start, c - 1));
                in_group = false;
            }
        }
    }
    if in_group {
        problems.push((group_start, width - 1));
    }

    // For each problem, extract operator and numbers, then evaluate
    let mut grand_total: i64 = 0;

    for (start_col, end_col) in problems {
        // Find the operator in the operator row within this column range
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
            None => continue, // no operator for this chunk; skip
        };

        // Collect all numbers for this problem
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

        let res = match op {
            '+' => nums.iter().sum::<i64>(),
            '*' => nums.iter().product::<i64>(),
            _ => continue,
        };

        grand_total += res;
    }

    grand_total
}
