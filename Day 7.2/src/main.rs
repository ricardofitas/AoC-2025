use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    // Timed single run
    let start = Instant::now();
    let answer = solve(INPUT);
    let elapsed = start.elapsed();

    println!("{answer}");
    eprintln!("Time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);
}

/// Quantum tachyon manifold (Part 2):
/// We count how many *timelines* a single particle ends up on.
/// Each time a particle hits '^' from above, time splits:
/// - the incoming path stops,
/// - one timeline continues from (row+1, col-1) if in bounds,
/// - one timeline continues from (row+1, col+1) if in bounds.
///
/// Implementation:
/// - Find S (start row sr, col sc).
/// - dp[r][c] = number of timelines whose particle is at cell (r, c).
/// - Initialize dp[sr][sc] = 1.
/// - For r = sr..h-1, for c = 0..w-1:
///     * let ways = dp[r][c].
///     * particle moves to row r+1; if r+1 >= h, add ways to answer.
///     * otherwise, if grid[r+1][c] == '^', split into left/right;
///       else, continue straight down to (r+1, c).
fn solve(input: &str) -> u128 {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut start: Option<(usize, usize)> = None;

    for (r, line) in input.lines().enumerate() {
        let bytes = line.as_bytes().to_vec();
        if let Some(c) = bytes.iter().position(|&b| b == b'S') {
            start = Some((r, c));
        }
        grid.push(bytes);
    }

    let h = grid.len();
    if h == 0 {
        return 0;
    }
    let w = grid[0].len();

    let (sr, sc) = start.expect("No 'S' found in input");

    // dp[r][c]: number of timelines at (r, c)
    let mut dp = vec![vec![0u128; w]; h];
    dp[sr][sc] = 1;

    let mut result: u128 = 0;

    for r in sr..h {
        for c in 0..w {
            let ways = dp[r][c];
            if ways == 0 {
                continue;
            }

            let nr = r + 1;
            if nr >= h {
                // Particle leaves the manifold → timeline ends
                result += ways;
                continue;
            }

            let ch = grid[nr][c];
            if ch == b'^' {
                // Split into left/right
                if c > 0 {
                    dp[nr][c - 1] += ways;
                }
                if c + 1 < w {
                    dp[nr][c + 1] += ways;
                }
            } else {
                // Continue straight down
                dp[nr][c] += ways;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::solve;

    const EXAMPLE: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn example_timelines() {
        let ans = solve(EXAMPLE);
        assert_eq!(ans, 40);
    }
}
