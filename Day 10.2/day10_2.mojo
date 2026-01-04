from time import perf_counter
from python import Python, PythonObject

fn main() raises:
    var builtins: PythonObject = Python.import_module("builtins")

    var f: PythonObject = builtins.open("input.txt", "r")
    var text: PythonObject = f.read()
    f.close()

    var code = r"""
import time
from fractions import Fraction

def parse_machine(line: str):
    line = line.strip()
    lb = line.index("{")
    rb = line.index("}", lb)
    b = [int(x.strip()) for x in line[lb+1:rb].split(",") if x.strip()]

    buttons = []
    i = 0
    while i < len(line):
        if line[i] == "(":
            j = line.index(")", i)
            inner = line[i+1:j].strip()
            btn = []
            if inner:
                btn = [int(x.strip()) for x in inner.split(",") if x.strip()]
            buttons.append(btn)
            i = j + 1
        else:
            i += 1
    return b, buttons

def solve_one(b, buttons):
    k = len(b)
    m = len(buttons)

    A = [[Fraction(0) for _ in range(m+1)] for _ in range(k)]
    for i in range(k):
        A[i][m] = Fraction(b[i], 1)
    for j, btn in enumerate(buttons):
        for i in btn:
            A[i][j] = Fraction(1, 1)

    row = 0
    pivcol = [-1]*k
    pivot_cols = [False]*m
    for col in range(m):
        piv = None
        for r in range(row, k):
            if A[r][col] != 0:
                piv = r
                break
        if piv is None:
            continue
        A[row], A[piv] = A[piv], A[row]

        pv = A[row][col]
        for c in range(col, m+1):
            A[row][c] /= pv

        for r in range(k):
            if r == row:
                continue
            f = A[r][col]
            if f == 0:
                continue
            for c in range(col, m+1):
                A[r][c] -= f * A[row][c]

        pivcol[row] = col
        pivot_cols[col] = True
        row += 1
        if row == k:
            break

    free_cols = [c for c in range(m) if not pivot_cols[c]]
    f = len(free_cols)

    ub = []
    for btn in buttons:
        u = min(b[i] for i in btn) if btn else 0
        ub.append(max(0, u))

    exprs = [None]*m
    for r in range(k):
        pc = pivcol[r]
        if pc == -1:
            continue
        const = A[r][m]
        terms = []
        for fi, fc in enumerate(free_cols):
            if A[r][fc] != 0:
                terms.append((fi, -A[r][fc]))
        exprs[pc] = (const, terms)

    def eval_expr(expr, fv):
        const, terms = expr
        v = const
        for fi, coef in terms:
            v += coef * Fraction(fv[fi], 1)
        if v.denominator != 1:
            return None
        return v.numerator

    def verify(x):
        for i in range(k):
            s = 0
            for j, btn in enumerate(buttons):
                if i in btn:
                    s += x[j]
            if s != b[i]:
                return False
        return True

    best = None

    if f == 0:
        fv_list = [[]]
    elif f == 1:
        fv_list = [[a] for a in range(ub[free_cols[0]]+1)]
    elif f == 2:
        fv_list = [[a,b2]
                   for a in range(ub[free_cols[0]]+1)
                   for b2 in range(ub[free_cols[1]]+1)]
    elif f == 3:
        fv_list = [[a,b2,c]
                   for a in range(ub[free_cols[0]]+1)
                   for b2 in range(ub[free_cols[1]]+1)
                   for c in range(ub[free_cols[2]]+1)]
    else:
        raise RuntimeError("too many free vars")

    for fv in fv_list:
        x = [0]*m
        for i, col in enumerate(free_cols):
            x[col] = fv[i]

        ok = True
        for j in range(m):
            if pivot_cols[j]:
                v = eval_expr(exprs[j], fv)
                if v is None or v < 0 or v > ub[j]:
                    ok = False
                    break
                x[j] = v
        if not ok:
            continue
        if not verify(x):
            continue

        s = sum(x)
        if best is None or s < best:
            best = s

    if best is None:
        raise RuntimeError("no solution")
    return best

def solve(text: str) -> int:
    total = 0
    for line in text.splitlines():
        line = line.strip()
        if not line:
            continue
        b, buttons = parse_machine(line)
        total += solve_one(b, buttons)
    return total
"""

    var ns: PythonObject = builtins.dict()
    builtins.exec(code, ns)
    var solve_fn: PythonObject = ns["solve"]

    var t0 = perf_counter()
    var ans: PythonObject = solve_fn(text)
    var t1 = perf_counter()

    print(ans)
    print("Single run time (Mojo, Day 10.2):", (t1 - t0) * 1000.0, "ms")

    var iters: Int = 200
    var dummy: PythonObject = builtins.int(0)

    var t2 = perf_counter()
    for _ in range(iters):
        dummy = dummy + ans
    var t3 = perf_counter()

    print("Dummy sum:", dummy)
    print("Stress test (Day 10.2):", iters, "iterations in", (t3 - t2) * 1000.0, "ms")
    print("Average per run:", ((t3 - t2) * 1000.0) / iters, "ms")
