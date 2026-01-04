use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy)]
struct Pt {
    x: i32,
    y: i32,
}

#[inline(always)]
fn abs_diff_i32(a: i32, b: i32) -> u64 {
    if a >= b {
        (a as i64 - b as i64) as u64
    } else {
        (b as i64 - a as i64) as u64
    }
}

fn main() {
    let start = Instant::now();
    let ans = solve(INPUT);
    let elapsed = start.elapsed();

    println!("{ans}");
    println!("Time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);
}

fn solve(input: &str) -> u64 {
    let pts = parse_points(input);
    let n = pts.len();
    if n < 2 {
        return 0;
    }

    let mut best: u64 = 0;
    for i in 0..n {
        let pi = pts[i];
        for j in (i + 1)..n {
            let pj = pts[j];
            let dx = abs_diff_i32(pi.x, pj.x);
            let dy = abs_diff_i32(pi.y, pj.y);
            let area = dx * dy;
            if area > best {
                best = area;
            }
        }
    }
    best
}

fn parse_points(input: &str) -> Vec<Pt> {
    let b = input.as_bytes();
    let mut i = 0usize;
    let mut pts: Vec<Pt> = Vec::new();

    #[inline(always)]
    fn parse_i32(b: &[u8], mut i: usize) -> (i32, usize) {
        let mut sign: i32 = 1;
        if i < b.len() && b[i] == b'-' {
            sign = -1;
            i += 1;
        }
        let mut val: i32 = 0;
        while i < b.len() {
            let c = b[i];
            if !c.is_ascii_digit() {
                break;
            }
            val = val * 10 + (c - b'0') as i32;
            i += 1;
        }
        (sign * val, i)
    }

    while i < b.len() {
        while i < b.len() && (b[i] == b'\n' || b[i] == b'\r' || b[i] == b' ' || b[i] == b'\t') {
            i += 1;
        }
        if i >= b.len() {
            break;
        }

        let (x, mut j) = parse_i32(b, i);
        if j >= b.len() || b[j] != b',' {
            break;
        }
        j += 1;

        let (y, mut k) = parse_i32(b, j);

        while k < b.len() && b[k] != b'\n' {
            k += 1;
        }
        i = k;

        pts.push(Pt { x, y });
    }

    pts
}
