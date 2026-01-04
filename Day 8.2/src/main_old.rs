use std::time::Instant;

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

    // returns true if merged
    #[inline(always)]
    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        true
    }
}

fn main() {
    let start = Instant::now();
    let ans = solve(INPUT);
    let elapsed = start.elapsed();

    println!("{ans}");
    println!("Time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);
}

// Part 2:
// Keep connecting the globally closest not-yet-directly-connected pair (each unordered pair once).
// The moment the whole graph becomes one circuit is exactly the moment DSU reaches 1 component
// while scanning pairs in increasing distance (Kruskal-style). We output x[a] * x[b] for that edge.
fn solve(input: &str) -> i128 {
    let pts = parse_points(input);
    let n = pts.len();
    if n <= 1 {
        return 0;
    }

    // Build all pairs once, sort by (dist2, a, b)
    let m = n * (n - 1) / 2;
    let mut edges: Vec<(u64, u32, u32)> = Vec::with_capacity(m);

    for a in 0..n {
        let pa = pts[a];
        for b in (a + 1)..n {
            edges.push((dist2(pa, pts[b]), a as u32, b as u32));
        }
    }

    edges.sort_unstable_by(|e1, e2| {
        e1.0.cmp(&e2.0)
            .then_with(|| e1.1.cmp(&e2.1))
            .then_with(|| e1.2.cmp(&e2.2))
    });

    let mut dsu = Dsu::new(n);
    let mut comps = n;

    for &(_, a, b) in &edges {
        if dsu.union(a as usize, b as usize) {
            comps -= 1;
            if comps == 1 {
                let xa = pts[a as usize].x as i128;
                let xb = pts[b as usize].x as i128;
                return xa * xb;
            }
        }
    }

    // Should never happen if input has >=2 points
    0
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
