import time
from fractions import Fraction

from qiskit import QuantumCircuit
from qiskit_aer import AerSimulator


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

    # Build augmented matrix with Fractions
    A = [[Fraction(0) for _ in range(m+1)] for _ in range(k)]
    for i in range(k):
        A[i][m] = Fraction(b[i], 1)
    for j, btn in enumerate(buttons):
        for i in btn:
            A[i][j] = Fraction(1, 1)

    # RREF
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

    # Free cols
    free_cols = [c for c in range(m) if not pivot_cols[c]]
    f = len(free_cols)

    # upper bounds
    ub = []
    for btn in buttons:
        u = min(b[i] for i in btn) if btn else 0
        ub.append(max(0, u))

    # expressions for pivot vars
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

    best = None

    def verify(x):
        for i in range(k):
            s = 0
            for j, btn in enumerate(buttons):
                if i in btn:
                    s += x[j]
            if s != b[i]:
                return False
        return True

    # enumerate free vars (dataset typically <=3)
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
        raise RuntimeError(f"Too many free vars for toy solver: {f}")

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
        raise RuntimeError("no solution found")
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


def toy_circuit():
    qc = QuantumCircuit(4, 1)
    qc.x(0)
    qc.cx(0, 1)
    qc.cx(1, 2)
    qc.cx(2, 3)
    qc.measure(3, 0)
    return qc


def main():
    with open("input.txt", "r", encoding="utf-8") as f:
        text = f.read()

    t0 = time.perf_counter()
    ans = solve(text)
    t1 = time.perf_counter()

    print(ans)
    print(f"Classical time: {(t1 - t0)*1000:.3f} ms\n")

    qc = toy_circuit()
    print("Toy counter-increment demo circuit:")
    print(qc.draw(output="text"))

    sim = AerSimulator()
    t2 = time.perf_counter()
    res = sim.run(qc, shots=1024).result()
    t3 = time.perf_counter()

    counts = res.get_counts()
    top = sorted(counts.items(), key=lambda kv: kv[1], reverse=True)[:5]
    print(f"Top measurement results (puzzle circuit): {top}")
    print(f"Simulator time: {(t3 - t2)*1000:.3f} ms")


if __name__ == "__main__":
    main()
