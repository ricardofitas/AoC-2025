use std::collections::{HashMap, VecDeque};
use std::time::Instant;

use rayon::prelude::*;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, Debug)]
struct Pt {
    x: i32,
    y: i32,
}

#[inline(always)]
fn cell_id(w: usize, x: usize, y: usize) -> usize {
    y * w + x
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let stress = args.iter().any(|a| a == "--stress");

    let start = Instant::now();
    let ans = solve(INPUT);
    let elapsed = start.elapsed();

    println!("{ans}");
    println!("Time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);

    if stress {
        let tasks: usize = 10_000;
        let ans_bb = std::hint::black_box(ans);

        let t0 = Instant::now();
        let dummy_sum: u128 = (0..tasks)
            .into_par_iter()
            .map(|_| std::hint::black_box(ans_bb as u128))
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

fn solve(input: &str) -> u64 {
    let red = parse_points(input);
    let n = red.len();
    if n < 2 {
        return 0;
    }

    // Bounding box (for margins)
    let mut minx = red[0].x;
    let mut maxx = red[0].x;
    let mut miny = red[0].y;
    let mut maxy = red[0].y;
    for p in &red {
        minx = minx.min(p.x);
        maxx = maxx.max(p.x);
        miny = miny.min(p.y);
        maxy = maxy.max(p.y);
    }

    // Coordinate compression: include x, x±1, x+2 (and same for y)
    let mut xs: Vec<i32> = Vec::with_capacity(n * 4 + 8);
    let mut ys: Vec<i32> = Vec::with_capacity(n * 4 + 8);

    for p in &red {
        xs.push(p.x - 1);
        xs.push(p.x);
        xs.push(p.x + 1);
        xs.push(p.x + 2);

        ys.push(p.y - 1);
        ys.push(p.y);
        ys.push(p.y + 1);
        ys.push(p.y + 2);
    }

    // Outside margin so cell (0,0) is definitely outside
    xs.push(minx - 2);
    xs.push(maxx + 3);
    ys.push(miny - 2);
    ys.push(maxy + 3);

    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();

    let xlen = xs.len();
    let ylen = ys.len();
    if xlen < 2 || ylen < 2 {
        return 0;
    }

    let mut x_pos: HashMap<i32, usize> = HashMap::with_capacity(xlen * 2);
    for (i, &v) in xs.iter().enumerate() {
        x_pos.insert(v, i);
    }
    let mut y_pos: HashMap<i32, usize> = HashMap::with_capacity(ylen * 2);
    for (i, &v) in ys.iter().enumerate() {
        y_pos.insert(v, i);
    }

    // Cell grid between coordinate lines
    let cw = xlen - 1;
    let ch = ylen - 1;
    let cell_count = cw * ch;

    // Walls: vwall at each x-line, per cell-row; hwall at each y-line, per cell-col
    let mut vwall = vec![false; xlen * ch];
    let mut hwall = vec![false; cw * ylen];

    // Mark polygon edges as walls
    for i in 0..n {
        let a = red[i];
        let b = red[(i + 1) % n];

        if a.y == b.y {
            let yidx = *y_pos.get(&a.y).expect("y not in map");
            let xlo = a.x.min(b.x);
            let xhi = a.x.max(b.x);

            let xs0 = *x_pos.get(&xlo).expect("xlo not in map");
            let xs1 = *x_pos.get(&xhi).expect("xhi not in map");

            for cx in xs0..xs1 {
                hwall[cx + yidx * cw] = true;
            }
        } else {
            let xidx = *x_pos.get(&a.x).expect("x not in map");
            let ylo = a.y.min(b.y);
            let yhi = a.y.max(b.y);

            let ys0 = *y_pos.get(&ylo).expect("ylo not in map");
            let ys1 = *y_pos.get(&yhi).expect("yhi not in map");

            for cy in ys0..ys1 {
                vwall[xidx + cy * xlen] = true;
            }
        }
    }

    // Flood fill OUTSIDE in cell space
    let mut outside = vec![false; cell_count];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();

    outside[0] = true;
    q.push_back((0, 0));

    while let Some((cx, cy)) = q.pop_front() {
        if cx > 0 && !vwall[cx + cy * xlen] {
            let nx = cx - 1;
            let nid = cell_id(cw, nx, cy);
            if !outside[nid] {
                outside[nid] = true;
                q.push_back((nx, cy));
            }
        }
        if cx + 1 < cw && !vwall[(cx + 1) + cy * xlen] {
            let nx = cx + 1;
            let nid = cell_id(cw, nx, cy);
            if !outside[nid] {
                outside[nid] = true;
                q.push_back((nx, cy));
            }
        }
        if cy > 0 && !hwall[cx + cy * cw] {
            let ny = cy - 1;
            let nid = cell_id(cw, cx, ny);
            if !outside[nid] {
                outside[nid] = true;
                q.push_back((cx, ny));
            }
        }
        if cy + 1 < ch && !hwall[cx + (cy + 1) * cw] {
            let ny = cy + 1;
            let nid = cell_id(cw, cx, ny);
            if !outside[nid] {
                outside[nid] = true;
                q.push_back((cx, ny));
            }
        }
    }

    // Weighted prefix sum over INSIDE cells
    let pw = cw + 1; // = xlen
    let mut pref = vec![0u64; (cw + 1) * (ch + 1)];

    for cy in 0..ch {
        let mut row_sum: u64 = 0;
        let dy = (ys[cy + 1] - ys[cy]) as u64;
        for cx in 0..cw {
            let inside = !outside[cell_id(cw, cx, cy)];
            let dx = (xs[cx + 1] - xs[cx]) as u64;
            let w = if inside { dx * dy } else { 0 };

            row_sum += w;
            let above = pref[cy * pw + (cx + 1)];
            pref[(cy + 1) * pw + (cx + 1)] = above + row_sum;
        }
    }

    #[inline(always)]
    fn rect_sum(pref: &[u64], pw: usize, x0: usize, y0: usize, x1: usize, y1: usize) -> u64 {
        let a = pref[y1 * pw + x1] as i128;
        let b = pref[y1 * pw + x0] as i128;
        let c = pref[y0 * pw + x1] as i128;
        let d = pref[y0 * pw + x0] as i128;
        (a - b - c + d) as u64
    }

    let mut best: u64 = 0;

    for i in 0..n {
        let a = red[i];
        for j in (i + 1)..n {
            let b = red[j];

            let xmin = a.x.min(b.x);
            let xmax = a.x.max(b.x);
            let ymin = a.y.min(b.y);
            let ymax = a.y.max(b.y);

            let area = (xmax as i64 - xmin as i64 + 1) as u64
                * (ymax as i64 - ymin as i64 + 1) as u64;

            if area <= best {
                continue;
            }

            let x0 = *x_pos.get(&xmin).expect("xmin not in map");
            let x1 = *x_pos.get(&(xmax + 1)).expect("xmax+1 not in map");
            let y0 = *y_pos.get(&ymin).expect("ymin not in map");
            let y1 = *y_pos.get(&(ymax + 1)).expect("ymax+1 not in map");

            let inside_tiles = rect_sum(&pref, pw, x0, y0, x1, y1);
            if inside_tiles == area {
                best = area;
            }
        }
    }

    best
}

fn parse_points(input: &str) -> Vec<Pt> {
    let mut out = Vec::new();
    for line in input.lines() {
        let s = line.trim();
        if s.is_empty() {
            continue;
        }
        let (a, b) = s.split_once(',').expect("bad line");
        let x: i32 = a.parse().expect("bad x");
        let y: i32 = b.parse().expect("bad y");
        out.push(Pt { x, y });
    }
    out
}
