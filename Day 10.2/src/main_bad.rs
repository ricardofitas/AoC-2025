use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let start = Instant::now();
    let ans = solve(INPUT);
    let elapsed = start.elapsed();
    println!("{ans}");
    eprintln!("Time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);
}

fn solve(input: &str) -> i64 {
    let mut total = 0i64;
    for line in input.lines() {
        let s = line.trim();
        if s.is_empty() { continue; }
        let (b, buttons) = parse_machine(s);
        total += solve_machine(&b, &buttons);
    }
    total
}

/// Parse: buttons (...) and target jolts {...}. Ignore [...].
fn parse_machine(line: &str) -> (Vec<i64>, Vec<Vec<usize>>) {
    let lb = line.find('{').expect("missing {");
    let rb = line[lb..].find('}').map(|i| lb + i).expect("missing }");
    let inside = &line[lb + 1..rb];

    let b: Vec<i64> = inside
        .split(',')
        .filter_map(|x| {
            let t = x.trim();
            if t.is_empty() { None } else { Some(t.parse::<i64>().expect("bad joltage")) }
        })
        .collect();

    let mut buttons: Vec<Vec<usize>> = Vec::new();
    let mut i = 0usize;
    let bytes = line.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'(' {
            let mut j = i + 1;
            while j < bytes.len() && bytes[j] != b')' { j += 1; }
            if j >= bytes.len() { panic!("unclosed ("); }
            let inner = line[i + 1..j].trim();
            let mut v = Vec::new();
            if !inner.is_empty() {
                for tok in inner.split(',') {
                    let t = tok.trim();
                    if !t.is_empty() { v.push(t.parse::<usize>().expect("bad index")); }
                }
            }
            buttons.push(v);
            i = j + 1;
        } else {
            i += 1;
        }
    }
    (b, buttons)
}

fn solve_machine(b: &[i64], buttons: &[Vec<usize>]) -> i64 {
    let k = b.len();
    let m = buttons.len();
    if k == 0 { return 0; }
    if m == 0 { panic!("no buttons"); }

    // Build A as k x m (0/1)
    let mut a = vec![vec![0i64; m]; k];
    for (j, btn) in buttons.iter().enumerate() {
        for &i in btn {
            a[i][j] = 1;
        }
    }

    // We'll do elimination over integers in a "solve for pivot" style:
    // choose pivot col, pivot row, and do row ops in i64 without fractions.
    // Since coefficients are 0/1, we can use:
    //   row_r = row_r - row_p  (when eliminating a 1 under pivot)
    // while keeping RHS consistent.
    // This is equivalent to elimination over Z, and works because pivots are 1.
    // (Dataset always lets us pick a pivot 1.)
    let mut rhs = b.to_vec();

    let mut row = 0usize;
    let mut pivot_col_for_row = vec![None; k];
    let mut is_pivot_col = vec![false; m];

    for col in 0..m {
        // find row with a[row][col] == 1 at/under current row
        let mut piv = None;
        for r in row..k {
            if a[r][col] == 1 {
                piv = Some(r);
                break;
            }
        }
        let Some(piv_row) = piv else { continue; };

        a.swap(row, piv_row);
        rhs.swap(row, piv_row);

        // eliminate this col in all other rows (since pivot is 1)
        for r in 0..k {
            if r == row { continue; }
            if a[r][col] == 1 {
                // row_r -= row_p
                for c in col..m {
                    a[r][c] -= a[row][c];
                }
                rhs[r] -= rhs[row];
            }
        }

        pivot_col_for_row[row] = Some(col);
        is_pivot_col[col] = true;
        row += 1;
        if row == k { break; }
    }

    // inconsistency check: 0 == nonzero
    for r in 0..k {
        let all0 = (0..m).all(|c| a[r][c] == 0);
        if all0 && rhs[r] != 0 {
            panic!("inconsistent machine");
        }
    }

    // free columns
    let mut free_cols = Vec::new();
    for c in 0..m {
        if !is_pivot_col[c] {
            free_cols.push(c);
        }
    }
    let f = free_cols.len();

    // bounds for free vars: min rhs_i among rows where col appears positively (a[i][col] > 0)
    // but since rows have been transformed, we use original b bound: x_j <= min(b[i] for i in button j)
    let mut ub = vec![0i64; m];
    for (j, btn) in buttons.iter().enumerate() {
        let mut u = i64::MAX;
        for &i in btn {
            u = u.min(b[i]);
        }
        ub[j] = u.max(0);
    }

    // Express pivot vars in terms of free vars from the current eliminated system:
    // For each pivot row r with pivot col p:
    //   a[r][p] = 1, others may be -1/0/1 etc, equation:
    //   x_p + sum_{c!=p} a[r][c]*x_c = rhs[r]
    // => x_p = rhs[r] - sum a[r][c]*x_c
    //
    // We'll enumerate free vars and compute pivots directly.

    let mut best = i64::MAX;

    // helper: compute and validate assignment
    let mut try_assign = |free_vals: &[i64]| {
        let mut x = vec![0i64; m];
        for (i, &c) in free_cols.iter().enumerate() {
            x[c] = free_vals[i];
        }

        // compute pivots
        for r in 0..k {
            let Some(pcol) = pivot_col_for_row[r] else { continue; };
            let mut v = rhs[r];
            for c in 0..m {
                if c == pcol { continue; }
                let coef = a[r][c];
                if coef != 0 {
                    v -= coef * x[c];
                }
            }
            // pivot var must be integer, nonnegative
            if v < 0 { return; }
            if v > ub[pcol] { return; }
            x[pcol] = v;
        }

        // verify original constraints A x = b (fast)
        for i in 0..k {
            let mut s = 0i64;
            for j in 0..m {
                if a0_has(buttons[j].as_slice(), i) {
                    s += x[j];
                }
            }
            if s != b[i] { return; }
        }

        let sum: i64 = x.iter().sum();
        if sum < best { best = sum; }
    };

    // enumerate (dataset f is tiny; if not, panic)
    match f {
        0 => try_assign(&[]),
        1 => {
            let c0 = free_cols[0];
            for a0 in 0..=ub[c0] { try_assign(&[a0]); }
        }
        2 => {
            let c0 = free_cols[0];
            let c1 = free_cols[1];
            for a0 in 0..=ub[c0] {
                for a1 in 0..=ub[c1] {
                    try_assign(&[a0, a1]);
                }
            }
        }
        3 => {
            let c0 = free_cols[0];
            let c1 = free_cols[1];
            let c2 = free_cols[2];
            for a0 in 0..=ub[c0] {
                for a1 in 0..=ub[c1] {
                    for a2 in 0..=ub[c2] {
                        try_assign(&[a0, a1, a2]);
                    }
                }
            }
        }
        _ => panic!("too many free vars in this machine: {f}"),
    }

    if best == i64::MAX {
        panic!("no nonnegative solution found");
    }
    best
}

#[inline(always)]
fn a0_has(btn: &[usize], idx: usize) -> bool {
    // buttons are tiny; linear scan is fastest
    for &x in btn {
        if x == idx { return true; }
    }
    false
}
