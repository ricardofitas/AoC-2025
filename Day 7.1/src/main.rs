use std::collections::VecDeque;
use std::time::Instant;
use rayon::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    // Single run
    let start = Instant::now();
    let answer = solve(INPUT);
    let elapsed = start.elapsed();

    println!("{answer}");
    eprintln!("Time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);

    // Parallel stress test: run solve 10_000 times in parallel
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

/// Simulate the tachyon beams and count how many times the beam is split.
///
/// - Beam starts at 'S', moving downward.
/// - '.' -> beam continues down.
/// - '^' -> incoming beam stops; two new beams spawn at same row, left and right.
/// - Beams exiting bottom disappear.
/// - A "split" is counted when a beam *enters* a '^' cell from above.
/// - We avoid reprocessing the same cell by keeping a visited[r][c] grid.
fn solve(input: &str) -> u64 {
    // Build grid and locate 'S'
    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut start: Option<(usize, usize)> = None;

    for (r, line) in input.lines().enumerate() {
        let bytes = line.as_bytes();
        if let Some(c) = bytes.iter().position(|&b| b == b'S') {
            start = Some((r, c));
        }
        grid.push(bytes.to_vec());
    }

    let h = grid.len();
    if h == 0 {
        return 0;
    }
    let w = grid[0].len();

    let (sr, sc) = start.expect("No 'S' found in input");

    let mut visited = vec![vec![false; w]; h];
    let mut queue = VecDeque::new();

    // Beam head starts at S
    queue.push_back((sr, sc));

    let mut splits: u64 = 0;

    while let Some((r, c)) = queue.pop_front() {
        let nr = r + 1;
        if nr >= h {
            continue; // exits
        }
        let nc = c;

        if visited[nr][nc] {
            continue;
        }
        visited[nr][nc] = true;

        match grid[nr][nc] {
            b'^' => {
                // splitter hit
                splits += 1;
                if nc > 0 {
                    queue.push_back((nr, nc - 1));
                }
                if nc + 1 < w {
                    queue.push_back((nr, nc + 1));
                }
            }
            _ => {
                // empty (or S, etc.)
                queue.push_back((nr, nc));
            }
        }
    }

    splits
}
