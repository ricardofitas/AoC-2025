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
    let mut total: i64 = 0;
    for line in input.lines() {
        let s = line.trim();
        if s.is_empty() {
            continue;
        }
        let (b, buttons) = parse_machine(s);
        total += min_presses(&b, &buttons);
    }
    total
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Frac {
    num: i64,
    den: i64, // always > 0
}

impl Frac {
    fn new(num: i64, den: i64) -> Self {
        assert!(den != 0);
        if num == 0 {
            return Frac { num: 0, den: 1 };
        }
        let mut n = num;
        let mut d = den;
        if d < 0 {
            n = -n;
            d = -d;
        }
        let g = gcd_i64(n.abs(), d);
        Frac { num: n / g, den: d / g }
    }
    fn zero() -> Self {
        Frac { num: 0, den: 1 }
    }
    fn one() -> Self {
        Frac { num: 1, den: 1 }
    }
    fn is_zero(self) -> bool {
        self.num == 0
    }
    fn add(self, other: Frac) -> Frac {
        // a/b + c/d = (ad+bc)/bd
        let n = (self.num as i128) * (other.den as i128) + (other.num as i128) * (self.den as i128);
        let d = (self.den as i128) * (other.den as i128);
        Frac::from_i128(n, d)
    }
    fn sub(self, other: Frac) -> Frac {
        let n = (self.num as i128) * (other.den as i128) - (other.num as i128) * (self.den as i128);
        let d = (self.den as i128) * (other.den as i128);
        Frac::from_i128(n, d)
    }
    fn mul(self, other: Frac) -> Frac {
        let n = (self.num as i128) * (other.num as i128);
        let d = (self.den as i128) * (other.den as i128);
        Frac::from_i128(n, d)
    }
    fn div(self, other: Frac) -> Frac {
        assert!(other.num != 0);
        let n = (self.num as i128) * (other.den as i128);
        let d = (self.den as i128) * (other.num as i128);
        Frac::from_i128(n, d)
    }
    fn from_i128(n: i128, d: i128) -> Frac {
        assert!(d != 0);
        if n == 0 {
            return Frac::zero();
        }
        let mut n = n;
        let mut d = d;
        if d < 0 {
            n = -n;
            d = -d;
        }
        let gn = (n.abs() as i128);
        let gd = d;
        let g = gcd_i128(gn, gd);
        let nn = (n / g) as i64;
        let dd = (d / g) as i64;
        Frac { num: nn, den: dd }
    }
    fn neg(self) -> Frac {
        Frac { num: -self.num, den: self.den }
    }
    fn is_int(self) -> bool {
        self.den == 1
    }
    fn as_int(self) -> i64 {
        assert!(self.den == 1);
        self.num
    }
}

fn gcd_i64(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.abs().max(1)
}
fn gcd_i128(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.abs().max(1)
}

/// Parse one machine line:
/// - buttons from (...) groups
/// - target jolts from {...}
/// We ignore the [...] diagram for part 2.
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
            while j < bytes.len() && bytes[j] != b')' {
                j += 1;
            }
            if j >= bytes.len() {
                panic!("unclosed (");
            }
            let inner = &line[i + 1..j].trim();
            let mut v: Vec<usize> = Vec::new();
            if !inner.is_empty() {
                for tok in inner.split(',') {
                    let t = tok.trim();
                    if !t.is_empty() {
                        v.push(t.parse::<usize>().expect("bad index"));
                    }
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

#[derive(Clone)]
struct Expr {
    constant: Frac,
    coeffs: Vec<(usize, Frac)>, // (free_var_idx, coeff)
}

fn min_presses(b: &[i64], buttons: &[Vec<usize>]) -> i64 {
    let k = b.len();
    let m = buttons.len();
    assert!(k > 0 && m > 0);

    // Build A as k x m with 0/1.
    let mut aug: Vec<Vec<Frac>> = vec![vec![Frac::zero(); m + 1]; k];
    for i in 0..k {
        aug[i][m] = Frac::new(b[i], 1);
    }
    for (j, btn) in buttons.iter().enumerate() {
        for &idx in btn {
            aug[idx][j] = Frac::one();
        }
    }

    // RREF on augmented matrix.
    let mut pivot_col_for_row: Vec<Option<usize>> = vec![None; k];
    let mut row = 0usize;

    for col in 0..m {
        // find pivot
        let mut piv = None;
        for r in row..k {
            if !aug[r][col].is_zero() {
                piv = Some(r);
                break;
            }
        }
        if piv.is_none() {
            continue;
        }
        let piv = piv.unwrap();
        aug.swap(row, piv);

        // normalize pivot row
        let pv = aug[row][col];
        let inv = Frac::one().div(pv);
        for c in col..=m {
            aug[row][c] = aug[row][c].mul(inv);
        }

        // eliminate other rows
        for r in 0..k {
            if r == row {
                continue;
            }
            let factor = aug[r][col];
            if factor.is_zero() {
                continue;
            }
            for c in col..=m {
                aug[r][c] = aug[r][c].sub(factor.mul(aug[row][c]));
            }
        }

        pivot_col_for_row[row] = Some(col);
        row += 1;
        if row == k {
            break;
        }
    }

    // Check inconsistency: 0 = nonzero
    for r in 0..k {
        let mut all0 = true;
        for c in 0..m {
            if !aug[r][c].is_zero() {
                all0 = false;
                break;
            }
        }
        if all0 && !aug[r][m].is_zero() {
            panic!("No solution for a machine (inconsistent constraints).");
        }
    }

    // Identify pivot cols and free cols.
    let mut pivot_cols = vec![false; m];
    for &pc in pivot_col_for_row.iter().flatten() {
        pivot_cols[pc] = true;
    }
    let mut free_cols: Vec<usize> = Vec::new();
    for c in 0..m {
        if !pivot_cols[c] {
            free_cols.push(c);
        }
    }

    // Upper bounds for each variable: x_j <= min(b_i over i in button j)
    // (safe bound; also makes enumeration tiny)
    let mut ub: Vec<i64> = vec![0; m];
    for (j, btn) in buttons.iter().enumerate() {
        let mut u = i64::MAX;
        for &idx in btn {
            u = u.min(b[idx]);
        }
        if u == i64::MAX {
            u = b.iter().copied().max().unwrap_or(0);
        }
        ub[j] = u.max(0);
    }

    // Build expressions for pivot variables in terms of free vars.
    // For each pivot row: x_p + sum(c_free * x_free) = rhs
    // => x_p = rhs - sum(c_free * x_free)
    let mut pivot_exprs: Vec<Option<Expr>> = vec![None; m];
    for r in 0..k {
        let Some(pcol) = pivot_col_for_row[r] else { continue; };
        let mut e = Expr { constant: aug[r][m], coeffs: Vec::new() };
        for (fi, &fcol) in free_cols.iter().enumerate() {
            let c = aug[r][fcol];
            if !c.is_zero() {
                e.coeffs.push((fi, c.neg()));
            }
        }
        pivot_exprs[pcol] = Some(e);
    }

    // Enumerate integer free vars (count ≤ 3 in this dataset) and find min sum solution.
    let f = free_cols.len();
    let mut best_sum: i64 = i64::MAX;

    // Evaluate pivot expressions given free values; return integer if integral.
    let eval_expr = |expr: &Expr, free_vals: &[i64]| -> Option<i64> {
        let mut v = expr.constant;
        for &(fi, coeff) in &expr.coeffs {
            let mul = coeff.mul(Frac::new(free_vals[fi], 1));
            v = v.add(mul);
        }
        if v.is_int() {
            Some(v.as_int())
        } else {
            None
        }
    };

    // helper to validate and score an assignment
    let mut check_assignment = |free_vals: &[i64]| {
        let mut x: Vec<i64> = vec![0; m];

        // assign free vars
        for (i, &col) in free_cols.iter().enumerate() {
            x[col] = free_vals[i];
        }

        // compute pivot vars
        for col in 0..m {
            if pivot_cols[col] {
                let expr = pivot_exprs[col].as_ref().expect("missing pivot expr");
                let val = match eval_expr(expr, free_vals) {
                    Some(v) => v,
                    None => return,
                };
                if val < 0 {
                    return;
                }
                if val > ub[col] {
                    // optional pruning; safe
                    return;
                }
                x[col] = val;
            }
        }

        // final sanity check: A x == b
        for i in 0..k {
            let mut s = 0i64;
            for j in 0..m {
                if buttons[j].iter().any(|&idx| idx == i) {
                    s += x[j];
                }
            }
            if s != b[i] {
                return;
            }
        }

        let sum: i64 = x.iter().sum();
        if sum < best_sum {
            best_sum = sum;
        }
    };

    if f == 0 {
        check_assignment(&[]);
    } else if f == 1 {
        let col0 = free_cols[0];
        for a in 0..=ub[col0] {
            check_assignment(&[a]);
        }
    } else if f == 2 {
        let col0 = free_cols[0];
        let col1 = free_cols[1];
        for a in 0..=ub[col0] {
            for b2 in 0..=ub[col1] {
                check_assignment(&[a, b2]);
            }
        }
    } else if f == 3 {
        let col0 = free_cols[0];
        let col1 = free_cols[1];
        let col2 = free_cols[2];
        for a in 0..=ub[col0] {
            for b2 in 0..=ub[col1] {
                for c in 0..=ub[col2] {
                    check_assignment(&[a, b2, c]);
                }
            }
        }
    } else {
        // Not expected in this dataset; still handle generically.
        // Could implement recursion if needed.
        panic!("Unexpected: too many free variables ({f})");
    }

    if best_sum == i64::MAX {
        panic!("No nonnegative integer solution found for a machine.");
    }
    best_sum
}
