use std::collections::HashMap;
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

fn solve(input: &str) -> u128 {
    let mut id: HashMap<String, usize> = HashMap::new();
    let mut adj: Vec<Vec<usize>> = Vec::new();

    fn get_id(name: &str, id: &mut HashMap<String, usize>, adj: &mut Vec<Vec<usize>>) -> usize {
        if let Some(&v) = id.get(name) {
            return v;
        }
        let v = adj.len();
        id.insert(name.to_string(), v);
        adj.push(Vec::new());
        v
    }

    for line in input.lines() {
        let s = line.trim();
        if s.is_empty() {
            continue;
        }
        let (lhs, rhs) = s
            .split_once(':')
            .unwrap_or_else(|| panic!("bad line (missing ':'): {s}"));
        let u = get_id(lhs.trim(), &mut id, &mut adj);
        for vname in rhs.trim().split_whitespace() {
            let v = get_id(vname.trim(), &mut id, &mut adj);
            adj[u].push(v);
        }
    }

    let svr = *id.get("svr").expect("missing 'svr'");
    let out = *id.get("out").expect("missing 'out'");
    let dac = *id.get("dac").expect("missing 'dac'");
    let fft = *id.get("fft").expect("missing 'fft'");

    let n = adj.len();
    let mut state = vec![[0u8; 4]; n]; // 0=unvisited,1=visiting,2=done per mask
    let mut memo = vec![[0u128; 4]; n];

    fn dfs(
        u: usize,
        mask: usize,
        out: usize,
        dac: usize,
        fft: usize,
        adj: &[Vec<usize>],
        state: &mut [[u8; 4]],
        memo: &mut [[u128; 4]],
    ) -> u128 {
        let mut mask2 = mask;
        if u == dac {
            mask2 |= 1;
        }
        if u == fft {
            mask2 |= 2;
        }

        match state[u][mask2] {
            2 => return memo[u][mask2],
            1 => panic!("cycle detected reachable from 'svr' (infinite paths?)"),
            _ => {}
        }

        state[u][mask2] = 1;

        let ans = if u == out {
            if mask2 == 3 { 1 } else { 0 }
        } else {
            let mut sum = 0u128;
            for &v in &adj[u] {
                sum = sum.saturating_add(dfs(v, mask2, out, dac, fft, adj, state, memo));
            }
            sum
        };

        state[u][mask2] = 2;
        memo[u][mask2] = ans;
        ans
    }

    dfs(svr, 0, out, dac, fft, &adj, &mut state, &mut memo)
}
