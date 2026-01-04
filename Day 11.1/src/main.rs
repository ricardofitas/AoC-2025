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
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (lhs, rhs) = line
            .split_once(':')
            .unwrap_or_else(|| panic!("bad line (missing ':'): {line}"));
        let from = get_id(lhs.trim(), &mut id, &mut adj);

        let rhs = rhs.trim();
        if rhs.is_empty() {
            continue;
        }
        for t in rhs.split_whitespace() {
            let to = get_id(t.trim(), &mut id, &mut adj);
            adj[from].push(to);
        }
    }

    let start = *id.get("you").expect("missing node 'you'");
    let out = *id.get("out").expect("missing node 'out'");

    let n = adj.len();
    let mut state = vec![0u8; n]; // 0=unvisited, 1=visiting, 2=done
    let mut ways = vec![0u128; n];

    let mut stack: Vec<(usize, u8)> = Vec::new(); // (node, phase) 0=enter,1=exit
    stack.push((start, 0));

    while let Some((v, phase)) = stack.pop() {
        if phase == 0 {
            match state[v] {
                2 => continue,
                1 => panic!("cycle detected reachable from 'you' (infinite paths?)"),
                _ => {}
            }
            state[v] = 1;
            stack.push((v, 1));
            for &nx in &adj[v] {
                if state[nx] == 1 {
                    panic!("cycle detected reachable from 'you' (infinite paths?)");
                }
                if state[nx] != 2 {
                    stack.push((nx, 0));
                }
            }
        } else {
            if v == out {
                ways[v] = 1;
            } else {
                let mut sum = 0u128;
                for &nx in &adj[v] {
                    sum = sum.saturating_add(ways[nx]);
                }
                ways[v] = sum;
            }
            state[v] = 2;
        }
    }

    ways[start]
}
