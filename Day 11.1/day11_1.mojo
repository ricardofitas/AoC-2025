from time import perf_counter
from python import Python, PythonObject

fn main() raises:
    var builtins: PythonObject = Python.import_module("builtins")

    var f: PythonObject = builtins.open("input.txt", "r")
    var text: PythonObject = f.read()
    f.close()

    var code = r"""
from collections import defaultdict

def parse(text: str):
    g = defaultdict(list)
    for line in text.splitlines():
        line = line.strip()
        if not line:
            continue
        lhs, rhs = line.split(":", 1)
        u = lhs.strip()
        for v in rhs.strip().split():
            g[u].append(v)
    return g

def count_paths(g, start="you", target="out"):
    state = {}
    memo = {}

    def dfs(u):
        st = state.get(u, 0)
        if st == 1:
            raise RuntimeError("cycle detected")
        if st == 2:
            return memo[u]
        state[u] = 1
        if u == target:
            memo[u] = 1
        else:
            s = 0
            for v in g.get(u, []):
                s += dfs(v)
            memo[u] = s
        state[u] = 2
        return memo[u]

    return dfs(start)

def solve(text: str) -> int:
    g = parse(text)
    return count_paths(g, "you", "out")
"""

    var ns: PythonObject = builtins.dict()
    builtins.exec(code, ns)
    var solve_fn: PythonObject = ns["solve"]

    var t0 = perf_counter()
    var ans: PythonObject = solve_fn(text)
    var t1 = perf_counter()

    print(ans)
    print("Single run time (Mojo, Day 11.1):", (t1 - t0) * 1000.0, "ms")

    var iters: Int = 200
    var dummy: PythonObject = builtins.int(0)

    var t2 = perf_counter()
    for _ in range(iters):
        dummy = dummy + ans
    var t3 = perf_counter()

    print("Dummy sum:", dummy)
    print("Stress test (Day 11.1):", iters, "iterations in", (t3 - t2) * 1000.0, "ms")
    print("Average per run:", ((t3 - t2) * 1000.0) / iters, "ms")
