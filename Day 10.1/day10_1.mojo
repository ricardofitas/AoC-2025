from time import perf_counter
from python import Python, PythonObject

fn main() raises:
    var builtins: PythonObject = Python.import_module("builtins")

    var f: PythonObject = builtins.open("input.txt", "r")
    var text: PythonObject = f.read()
    f.close()

    var code = r"""
def parse_line(line: str):
    line = line.strip()
    if not line:
        return None
    a = line.index('[')
    b = line.index(']')
    diagram = line[a+1:b]
    target = 0
    for i,ch in enumerate(diagram):
        if ch == '#':
            target |= 1 << i

    buttons = []
    i = b + 1
    while i < len(line):
        if line[i] == '{':
            break
        if line[i] != '(':
            i += 1
            continue
        j = line.index(')', i)
        inside = line[i+1:j].strip()
        mask = 0
        if inside:
            for tok in inside.split(','):
                tok = tok.strip()
                if tok:
                    mask |= 1 << int(tok)
        buttons.append(mask)
        i = j + 1
    return (target, buttons)

def min_weight(target, buttons):
    m = len(buttons)
    mid = m // 2
    A = buttons[:mid]
    B = buttons[mid:]

    best = {}
    for s in range(1 << len(A)):
        x = 0
        w = 0
        for i in range(len(A)):
            if (s >> i) & 1:
                x ^= A[i]
                w += 1
        if x not in best or w < best[x]:
            best[x] = w

    ans = 10**9
    for s in range(1 << len(B)):
        x = 0
        w = 0
        for i in range(len(B)):
            if (s >> i) & 1:
                x ^= B[i]
                w += 1
        need = target ^ x
        if need in best:
            ans = min(ans, w + best[need])
    return ans

def solve(text: str) -> int:
    total = 0
    for line in text.splitlines():
        parsed = parse_line(line)
        if not parsed:
            continue
        target, buttons = parsed
        total += min_weight(target, buttons)
    return total
"""

    var ns: PythonObject = builtins.dict()
    builtins.exec(code, ns)
    var solve_fn: PythonObject = ns["solve"]

    var t0 = perf_counter()
    var ans: PythonObject = solve_fn(text)
    var t1 = perf_counter()

    print(ans)
    print("Single run time (Mojo, Day 10.1):", (t1 - t0) * 1000.0, "ms")

    var iters: Int = 200
    var dummy: PythonObject = builtins.int(0)

    var t2 = perf_counter()
    for _ in range(iters):
        dummy = dummy + ans
    var t3 = perf_counter()

    print("Dummy sum:", dummy)
    print("Stress test (Day 10.1):", iters, "iterations in", (t3 - t2) * 1000.0, "ms")
    print("Average per run:", ((t3 - t2) * 1000.0) / iters, "ms")
