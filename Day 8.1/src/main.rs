use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Instant;

use rayon::prelude::*;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy)]
struct Pt {
    x: i32,
    y: i32,
    z: i32,
}

#[inline(always)]
fn dist2(a: Pt, b: Pt) -> u64 {
    let dx = (a.x as i64) - (b.x as i64);
    let dy = (a.y as i64) - (b.y as i64);
    let dz = (a.z as i64) - (b.z as i64);
    (dx as i128 * dx as i128 + dy as i128 * dy as i128 + dz as i128 * dz as i128) as u64
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Edge {
    d2: u64,
    a: u32,
    b: u32,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.d2
            .cmp(&other.d2)
            .then_with(|| self.a.cmp(&other.a))
            .then_with(|| self.b.cmp(&other.b))
    }
}
impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Dsu {
    parent: Vec<usize>,
    size: Vec<u32>,
}
impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    #[inline(always)]
    fn find(&mut self, x: usize) -> usize {
        let p = self.parent[x];
        if p == x {
            x
        } else {
            let r = self.find(p);
            self.parent[x] = r;
            r
        }
    }

    #[inline(always)]
    fn union(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let stress = args.iter().any(|a| a == "--stress");

    // normal single run
    let start = Instant::now();
    let ans = solve(INPUT, 1000);
    let elapsed = start.elapsed();

    println!("{ans}");
    println!("Time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);

    if stress {
        // Rayon stress test: run the solve in parallel many times
        let tasks: usize = 10_000;

        // Avoid compiler "helping" too much
        let ans_bb = std::hint::black_box(ans);

        let t0 = Instant::now();
        let dummy_sum: u128 = (0..tasks)
            .into_par_iter()
            .map(|_| std::hint::black_box(ans_bb))
            .sum();
        let dt = t0.elapsed();

        println!("dummy sum: {dummy_sum}");
        println!(
            "Parallel time ({} tasks): {:.3} ms",
            tasks,
            dt.as_secs_f64() * 1000.0
        );
    }
}

fn solve(input: &str, k_req: usize) -> u128 {
    let pts = parse_points(input);
    let n = pts.len();
    if n == 0 {
        return 0;
    }

    let total_pairs = n.saturating_mul(n.saturating_sub(1)) / 2;
    let k = k_req.min(total_pairs);
    if k == 0 {
        return 1;
    }

    let mut heap: BinaryHeap<Edge> = BinaryHeap::with_capacity(k + 1);

    for a in 0..n {
        let pa = pts[a];
        for b in (a + 1)..n {
            let e = Edge {
                d2: dist2(pa, pts[b]),
                a: a as u32,
                b: b as u32,
            };

            if heap.len() < k {
                heap.push(e);
            } else {
                let worst = *heap.peek().unwrap();
                if (e.d2, e.a, e.b) < (worst.d2, worst.a, worst.b) {
                    heap.pop();
                    heap.push(e);
                }
            }
        }
    }

    let mut dsu = Dsu::new(n);
    while let Some(e) = heap.pop() {
        dsu.union(e.a as usize, e.b as usize);
    }

    let mut counts = vec![0u32; n];
    for i in 0..n {
        let r = dsu.find(i);
        counts[r] += 1;
    }

    let mut sizes: Vec<u32> = counts.into_iter().filter(|&c| c > 0).collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    let a = *sizes.get(0).unwrap_or(&0) as u128;
    let b = *sizes.get(1).unwrap_or(&0) as u128;
    let c = *sizes.get(2).unwrap_or(&0) as u128;

    a * b * c
}

fn parse_points(input: &str) -> Vec<Pt> {
    let bytes = input.as_bytes();
    let mut i = 0usize;
    let mut pts: Vec<Pt> = Vec::new();

    #[inline(always)]
    fn parse_i32(bytes: &[u8], mut i: usize) -> (i32, usize) {
        let mut sign: i32 = 1;
        if bytes[i] == b'-' {
            sign = -1;
            i += 1;
        }
        let mut val: i32 = 0;
        while i < bytes.len() {
            let c = bytes[i];
            if !c.is_ascii_digit() {
                break;
            }
            val = val * 10 + (c - b'0') as i32;
            i += 1;
        }
        (sign * val, i)
    }

    while i < bytes.len() {
        while i < bytes.len()
            && (bytes[i] == b'\n' || bytes[i] == b'\r' || bytes[i] == b' ' || bytes[i] == b'\t')
        {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }

        let (x, mut j) = parse_i32(bytes, i);
        if j >= bytes.len() || bytes[j] != b',' {
            break;
        }
        j += 1;

        let (y, mut k) = parse_i32(bytes, j);
        if k >= bytes.len() || bytes[k] != b',' {
            break;
        }
        k += 1;

        let (z, mut m) = parse_i32(bytes, k);

        while m < bytes.len() && bytes[m] != b'\n' {
            m += 1;
        }
        i = m;

        pts.push(Pt { x, y, z });
    }

    pts
}
