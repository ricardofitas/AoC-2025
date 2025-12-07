use std::collections::VecDeque;
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let start = Instant::now();
    let answer = solve(INPUT);
    let elapsed = start.elapsed();

    println!("{answer}");
    eprintln!("Time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);
}

/// Simulate the tachyon beams and count how many times the beam is split.
///
/// Rules:
/// - Beam starts at 'S', moving downward.
/// - Empty space '.' -> beam just continues downward.
/// - Splitter '^' -> the incoming beam stops; two new beams appear to the
///   immediate left and right of the splitter (same row), and then move downward.
/// - Beams exit when they move past the last row.
/// - We count a "split" each time a beam *enters* a '^' cell from above.
/// - To avoid double-counting and redundant work, each cell is only processed
///   once as a beam position using a `visited` grid.
fn solve(input: &str) -> u64 {
    // Build grid as bytes and locate 'S'
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

    // visited[r][c] == true means we already processed a beam occupying (r, c)
    let mut visited = vec![vec![false; w]; h];
    let mut queue = VecDeque::new();

    // Beam "head" starts at S; on each step we move it down one row.
    queue.push_back((sr, sc));

    let mut splits: u64 = 0;

    while let Some((r, c)) = queue.pop_front() {
        let nr = r + 1;
        if nr >= h {
            // Beam exits the manifold
            continue;
        }
        let nc = c;

        if visited[nr][nc] {
            // Already processed a beam in this cell; future beams here
            // would behave identically, so skip.
            continue;
        }
        visited[nr][nc] = true;

        match grid[nr][nc] {
            b'^' => {
                // Beam hits a splitter: count a split, stop this beam,
                // and spawn two new beams to the left and right on the
                // same row, which will move down on their next step.
                splits += 1;

                if nc > 0 {
                    queue.push_back((nr, nc - 1));
                }
                if nc + 1 < w {
                    queue.push_back((nr, nc + 1));
                }
            }
            _ => {
                // Empty space (or S, theoretically) – beam continues downward.
                queue.push_back((nr, nc));
            }
        }
    }

    splits
}
