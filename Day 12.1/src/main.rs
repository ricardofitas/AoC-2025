use std::time::Instant;
use rayon::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let stress = args.iter().any(|a| a == "--stress");

    let start = Instant::now();
    let ans = solve(INPUT);
    let elapsed = start.elapsed();

    println!("{ans}");
    eprintln!("Time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);

    if stress {
        let tasks: usize = 10_000;
        let ans_bb = std::hint::black_box(ans);

        let t0 = Instant::now();
        let dummy_sum: u64 = (0..tasks)
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

fn solve(input: &str) -> u64 {
    let mut lines = input.lines().map(str::trim);

    // Parse 6 shapes (0..5), each is 3 lines of 3 chars.
    // We only need the number of '#' per shape.
    let mut shape_area = [0u64; 6];

    // Read until first region line, buffering the shape section + first region if found.
    let mut buffered: Vec<String> = Vec::new();
    for l in lines.by_ref() {
        if l.is_empty() {
            continue;
        }
        if is_region_line(l) {
            buffered.push(l.to_string());
            break;
        }
        buffered.push(l.to_string());
    }

    // Parse shapes from buffered prefix.
    let mut i = 0usize;
    while i < buffered.len() {
        let s = buffered[i].as_str();
        if let Some(id_str) = s.strip_suffix(':') {
            let id: usize = id_str.parse().expect("bad shape id");
            let r1 = buffered.get(i + 1).expect("missing shape row 1");
            let r2 = buffered.get(i + 2).expect("missing shape row 2");
            let r3 = buffered.get(i + 3).expect("missing shape row 3");

            shape_area[id] =
                count_hashes(r1) as u64 + count_hashes(r2) as u64 + count_hashes(r3) as u64;

            i += 4;
        } else {
            i += 1;
        }
    }

    // Count how many region lines fit by area
    let mut ok: u64 = 0;

    for l in buffered.iter().filter(|s| is_region_line(s)) {
        if region_fits(l, &shape_area) {
            ok += 1;
        }
    }

    for l in lines {
        if l.is_empty() {
            continue;
        }
        if !is_region_line(l) {
            continue;
        }
        if region_fits(l, &shape_area) {
            ok += 1;
        }
    }

    ok
}

#[inline(always)]
fn count_hashes(s: &str) -> usize {
    s.as_bytes().iter().filter(|&&b| b == b'#').count()
}

#[inline(always)]
fn is_region_line(s: &str) -> bool {
    // "<digits>x<digits>:"
    let bytes = s.as_bytes();
    let mut j = 0usize;

    if bytes.is_empty() || !bytes[0].is_ascii_digit() {
        return false;
    }
    while j < bytes.len() && bytes[j].is_ascii_digit() {
        j += 1;
    }
    if j >= bytes.len() || bytes[j] != b'x' {
        return false;
    }
    j += 1;
    if j >= bytes.len() || !bytes[j].is_ascii_digit() {
        return false;
    }
    while j < bytes.len() && bytes[j].is_ascii_digit() {
        j += 1;
    }
    j < bytes.len() && bytes[j] == b':'
}

fn region_fits(line: &str, shape_area: &[u64; 6]) -> bool {
    let (wh, rest) = line.split_once(':').expect("bad region line");
    let (w_s, h_s) = wh.split_once('x').expect("bad WxH");
    let w: u64 = w_s.parse().expect("bad W");
    let h: u64 = h_s.parse().expect("bad H");
    let cap = w * h;

    let mut need: u64 = 0;
    let mut idx = 0usize;
    for tok in rest.split_whitespace() {
        if idx >= 6 {
            break;
        }
        let c: u64 = tok.parse().expect("bad count");
        need += c * shape_area[idx];
        idx += 1;
    }
    if idx != 6 {
        panic!("expected 6 counts per region line");
    }

    need <= cap
}
