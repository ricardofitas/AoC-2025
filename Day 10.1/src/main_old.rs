use std::collections::HashMap;
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let start = Instant::now();
    let ans = solve(INPUT);
    let elapsed = start.elapsed();

    println!("{ans}");
    eprintln!("Time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);
}

/// Sum, across all machines, of the minimum number of button presses to reach the target pattern.
/// Each button press toggles bits (GF(2)); since cost is linear in presses and toggles are mod 2,
/// optimal solutions only need each button pressed 0 or 1 times.
fn solve(input: &str) -> u64 {
    let mut total: u64 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (n_lights, target_mask, buttons) = parse_machine(line);

        // We store masks in u64, so require n_lights <= 64.
        if n_lights > 64 {
            panic!("Too many lights ({n_lights}); this solver supports up to 64.");
        }

        let min_presses = min_weight_solution(target_mask, &buttons);
        total += min_presses as u64;
    }

    total
}

/// Meet-in-the-middle minimum Hamming weight solution to:
/// XOR of chosen button masks == target.
fn min_weight_solution(target: u64, buttons: &[u64]) -> u32 {
    let m = buttons.len();
    if m == 0 {
        return if target == 0 { 0 } else { u32::MAX / 2 };
    }

    let mid = m / 2;
    let (a, b) = buttons.split_at(mid);

    // Enumerate all subsets of A and record best (minimum weight) for each XOR value.
    let size_a = 1usize << a.len();
    let mut best: HashMap<u64, u16> = HashMap::with_capacity(size_a * 2);

    // DP subset enumeration: xor[s] = xor[s without lsb] ^ mask[lsb_index]
    let mut xor_a = vec![0u64; size_a];
    let mut w_a = vec![0u16; size_a];
    for s in 1..size_a {
        let lsb = s & (!s + 1);
        let i = lsb.trailing_zeros() as usize;
        let prev = s ^ lsb;
        xor_a[s] = xor_a[prev] ^ a[i];
        w_a[s] = w_a[prev] + 1;
    }

    for s in 0..size_a {
        let x = xor_a[s];
        let w = w_a[s];
        match best.get_mut(&x) {
            None => {
                best.insert(x, w);
            }
            Some(cur) => {
                if w < *cur {
                    *cur = w;
                }
            }
        }
    }

    // Enumerate all subsets of B and check complement needed from A.
    let size_b = 1usize << b.len();
    let mut xor_b = vec![0u64; size_b];
    let mut w_b = vec![0u16; size_b];
    for s in 1..size_b {
        let lsb = s & (!s + 1);
        let i = lsb.trailing_zeros() as usize;
        let prev = s ^ lsb;
        xor_b[s] = xor_b[prev] ^ b[i];
        w_b[s] = w_b[prev] + 1;
    }

    let mut ans: u32 = u32::MAX / 2;
    for s in 0..size_b {
        let xb = xor_b[s];
        let need = target ^ xb;
        if let Some(&wa) = best.get(&need) {
            let w = wa as u32 + w_b[s] as u32;
            if w < ans {
                ans = w;
            }
        }
    }

    ans
}

/// Parse a single machine line:
///   [diagram] (button) (button) ... {ignored}
/// Returns (n_lights, target_mask, button_masks).
fn parse_machine(line: &str) -> (usize, u64, Vec<u64>) {
    let bytes = line.as_bytes();
    let mut i = 0usize;

    // Find '['
    while i < bytes.len() && bytes[i] != b'[' {
        i += 1;
    }
    if i >= bytes.len() {
        panic!("No diagram '[' found in line: {line}");
    }
    i += 1;
    let start = i;

    // Find ']'
    while i < bytes.len() && bytes[i] != b']' {
        i += 1;
    }
    if i >= bytes.len() {
        panic!("No diagram ']' found in line: {line}");
    }
    let diagram = &line[start..i];
    i += 1;

    let n = diagram.len();
    let mut target: u64 = 0;
    for (idx, ch) in diagram.bytes().enumerate() {
        match ch {
            b'#' => target |= 1u64 << idx,
            b'.' => {}
            _ => panic!("Invalid diagram char: {}", ch as char),
        }
    }

    // Parse zero or more (...) groups until '{' or end.
    let mut buttons: Vec<u64> = Vec::new();

    while i < bytes.len() {
        // stop at '{' (joltage requirements ignored)
        if bytes[i] == b'{' {
            break;
        }

        // skip whitespace
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= bytes.len() || bytes[i] == b'{' {
            break;
        }

        if bytes[i] != b'(' {
            // If there's stray chars, skip them (robustness).
            i += 1;
            continue;
        }

        // read '( ... )'
        i += 1;
        let mut mask: u64 = 0;
        loop {
            // skip whitespace
            while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            if i >= bytes.len() {
                panic!("Unclosed '(' in line: {line}");
            }
            if bytes[i] == b')' {
                i += 1;
                break;
            }

            // parse integer
            let mut val: usize = 0;
            let mut saw_digit = false;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                saw_digit = true;
                val = val * 10 + (bytes[i] - b'0') as usize;
                i += 1;
            }
            if !saw_digit {
                panic!("Expected digit inside (...) in line: {line}");
            }
            if val >= 64 {
                panic!("Light index {val} too large for u64 in line: {line}");
            }
            mask |= 1u64 << val;

            // skip whitespace
            while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            if i >= bytes.len() {
                panic!("Unclosed '(' in line: {line}");
            }
            if bytes[i] == b',' {
                i += 1;
                continue;
            }
            if bytes[i] == b')' {
                i += 1;
                break;
            }

            // Any other char is invalid inside parentheses
            panic!("Unexpected char '{}' in (...) in line: {line}", bytes[i] as char);
        }

        buttons.push(mask);
    }

    (n, target, buttons)
}
