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

    // Original incidence for fast verification / bounds.
    let mut affects: Vec<Vec<usize>> = Vec::with_capacity(m);
    for btn in buttons {
        affects.push(btn.clone());
    }

    // Build A as k x m (0/1)
    let mut a = vec![vec![0i64; m]; k];
    for (j, btn) in buttons.iter().enumerate() {
        for &i in btn {
            a[i][j] = 1;
        }
    }
    let mut rhs = b.to_vec();

    // Integer Gauss with full-factor elimination when pivot is ±1; otherwise fall back.
    let mut row = 0usize;
    let mut pivot_col_for_row = vec![None; k];
    let mut is_pivot_col = vec![false; m];

    for col in 0..m {
        let mut piv = None;
        for r in row..k {
            if a[r][col].abs() == 1 { piv = Some(r); break; }
        }
        if piv.is_none() {
            for r in row..k {
                if a[r][col] != 0 { piv = Some(r); break; }
            }
        }
        let Some(piv_row) = piv else { continue; };

        a.swap(row, piv_row);
        rhs.swap(row, piv_row);

        if a[row][col] == -1 {
            for c in col..m { a[row][c] = -a[row][c]; }
            rhs[row] = -rhs[row];
        }

        if a[row][col].abs() != 1 {
            return solve_machine_frac(b, buttons);
        }

        for r in 0..k {
            if r == row { continue; }
            let factor = a[r][col];
            if factor == 0 { continue; }
            for c in col..m {
                a[r][c] -= factor * a[row][c];
            }
            rhs[r] -= factor * rhs[row];
        }

        pivot_col_for_row[row] = Some(col);
        is_pivot_col[col] = true;
        row += 1;
        if row == k { break; }
    }

    for r in 0..k {
        let mut all0 = true;
        for c in 0..m {
            if a[r][c] != 0 { all0 = false; break; }
        }
        if all0 && rhs[r] != 0 {
            return solve_machine_frac(b, buttons);
        }
    }

    let mut free_cols = Vec::new();
    for c in 0..m {
        if !is_pivot_col[c] { free_cols.push(c); }
    }
    let f = free_cols.len();

    let mut ub = vec![0i64; m];
    for (j, btn) in affects.iter().enumerate() {
        let mut u = i64::MAX;
        for &i in btn { u = u.min(b[i]); }
        if u == i64::MAX { u = 0; }
        ub[j] = u.max(0);
    }

    let mut best = i64::MAX;

    let mut try_assign = |free_vals: &[i64]| {
        let mut x = vec![0i64; m];

        for (i, &c) in free_cols.iter().enumerate() {
            x[c] = free_vals[i];
        }

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
            if v < 0 { return; }
            if v > ub[pcol] { return; }
            x[pcol] = v;
        }

        for i in 0..k {
            let mut s = 0i64;
            for j in 0..m {
                if affects[j].iter().any(|&ii| ii == i) {
                    s += x[j];
                }
            }
            if s != b[i] { return; }
        }

        let sum: i64 = x.iter().sum();
        if sum < best { best = sum; }
    };

    match f {
        0 => try_assign(&[]),
        1 => { let c0 = free_cols[0]; for a0 in 0..=ub[c0] { try_assign(&[a0]); } }
        2 => {
            let c0 = free_cols[0]; let c1 = free_cols[1];
            for a0 in 0..=ub[c0] { for a1 in 0..=ub[c1] { try_assign(&[a0,a1]); } }
        }
        3 => {
            let c0 = free_cols[0]; let c1 = free_cols[1]; let c2 = free_cols[2];
            for a0 in 0..=ub[c0] { for a1 in 0..=ub[c1] { for a2 in 0..=ub[c2] { try_assign(&[a0,a1,a2]); } } }
        }
        _ => return solve_machine_frac(b, buttons),
    }

    if best == i64::MAX {
        return solve_machine_frac(b, buttons);
    }
    best
}

/* ---------- SAFE FALLBACK (fraction-based) ---------- */

#[derive(Clone, Copy)]
struct Frac { n: i128, d: i128 } // d>0

fn gcd_i128(mut a: i128, mut b: i128) -> i128 {
    while b != 0 { let r = a % b; a = b; b = r; }
    a.abs().max(1)
}
fn frac(n: i128, d: i128) -> Frac {
    assert!(d != 0);
    if n == 0 { return Frac { n: 0, d: 1 }; }
    let mut n = n;
    let mut d = d;
    if d < 0 { n = -n; d = -d; }
    let g = gcd_i128(n.abs(), d);
    Frac { n: n / g, d: d / g }
}
impl Frac {
    fn zero() -> Self { Frac { n: 0, d: 1 } }
    fn one() -> Self { Frac { n: 1, d: 1 } }
    fn is_zero(self) -> bool { self.n == 0 }
    fn add(self, o: Frac) -> Frac { frac(self.n*o.d + o.n*self.d, self.d*o.d) }
    fn sub(self, o: Frac) -> Frac { frac(self.n*o.d - o.n*self.d, self.d*o.d) }
    fn mul(self, o: Frac) -> Frac { frac(self.n*o.n, self.d*o.d) }
    fn div(self, o: Frac) -> Frac { frac(self.n*o.d, self.d*o.n) }
    fn neg(self) -> Frac { Frac { n: -self.n, d: self.d } }
    fn is_int(self) -> bool { self.d == 1 }
    fn as_i64(self) -> i64 { self.n as i64 }
}

fn solve_machine_frac(b: &[i64], buttons: &[Vec<usize>]) -> i64 {
    let k = b.len();
    let m = buttons.len();

    let mut aug = vec![vec![Frac::zero(); m+1]; k];
    for i in 0..k { aug[i][m] = frac(b[i] as i128, 1); }
    for (j, btn) in buttons.iter().enumerate() {
        for &i in btn { aug[i][j] = Frac::one(); }
    }

    let mut row = 0usize;
    let mut pivot_col_for_row = vec![None; k];
    let mut pivot_cols = vec![false; m];

    for col in 0..m {
        let mut piv = None;
        for r in row..k {
            if !aug[r][col].is_zero() { piv = Some(r); break; }
        }
        let Some(p) = piv else { continue; };
        aug.swap(row, p);

        let pv = aug[row][col];
        let inv = Frac::one().div(pv);
        for c in col..=m { aug[row][c] = aug[row][c].mul(inv); }

        for r in 0..k {
            if r == row { continue; }
            let f = aug[r][col];
            if f.is_zero() { continue; }
            for c in col..=m {
                aug[r][c] = aug[r][c].sub(f.mul(aug[row][c]));
            }
        }

        pivot_col_for_row[row] = Some(col);
        pivot_cols[col] = true;
        row += 1;
        if row == k { break; }
    }

    for r in 0..k {
        let mut all0 = true;
        for c in 0..m {
            if !aug[r][c].is_zero() { all0 = false; break; }
        }
        if all0 && !aug[r][m].is_zero() {
            panic!("inconsistent machine (fallback)");
        }
    }

    let mut free_cols = Vec::new();
    for c in 0..m {
        if !pivot_cols[c] { free_cols.push(c); }
    }
    let f = free_cols.len();

    let mut ub = vec![0i64; m];
    for (j, btn) in buttons.iter().enumerate() {
        let mut u = i64::MAX;
        for &i in btn { u = u.min(b[i]); }
        if u == i64::MAX { u = 0; }
        ub[j] = u.max(0);
    }

    #[derive(Clone)]
    struct Expr { c: Frac, terms: Vec<(usize, Frac)> }

    let mut exprs: Vec<Option<Expr>> = vec![None; m];
    for r in 0..k {
        let Some(pcol) = pivot_col_for_row[r] else { continue; };
        let mut e = Expr { c: aug[r][m], terms: Vec::new() };
        for (fi, &fcol) in free_cols.iter().enumerate() {
            let coef = aug[r][fcol];
            if !coef.is_zero() {
                e.terms.push((fi, coef.neg()));
            }
        }
        exprs[pcol] = Some(e);
    }

    let eval = |e: &Expr, fv: &[i64]| -> Option<i64> {
        let mut v = e.c;
        for &(fi, coef) in &e.terms {
            v = v.add(coef.mul(frac(fv[fi] as i128, 1)));
        }
        if v.is_int() { Some(v.as_i64()) } else { None }
    };

    let mut best = i64::MAX;
    let mut try_assign = |fv: &[i64]| {
        let mut x = vec![0i64; m];
        for (i, &c) in free_cols.iter().enumerate() { x[c] = fv[i]; }
        for c in 0..m {
            if pivot_cols[c] {
                let e = exprs[c].as_ref().unwrap();
                let v = match eval(e, fv) { Some(v) => v, None => return };
                if v < 0 || v > ub[c] { return; }
                x[c] = v;
            }
        }
        for i in 0..k {
            let mut s = 0i64;
            for j in 0..m {
                if buttons[j].iter().any(|&ii| ii == i) {
                    s += x[j];
                }
            }
            if s != b[i] { return; }
        }
        let sum: i64 = x.iter().sum();
        if sum < best { best = sum; }
    };

    match f {
        0 => try_assign(&[]),
        1 => { let c0 = free_cols[0]; for a0 in 0..=ub[c0] { try_assign(&[a0]); } }
        2 => {
            let c0 = free_cols[0]; let c1 = free_cols[1];
            for a0 in 0..=ub[c0] { for a1 in 0..=ub[c1] { try_assign(&[a0,a1]); } }
        }
        3 => {
            let c0 = free_cols[0]; let c1 = free_cols[1]; let c2 = free_cols[2];
            for a0 in 0..=ub[c0] { for a1 in 0..=ub[c1] { for a2 in 0..=ub[c2] { try_assign(&[a0,a1,a2]); } } }
        }
        _ => panic!("too many free vars in fallback: {f}"),
    }

    if best == i64::MAX {
        panic!("no solution found");
    }
    best
}
