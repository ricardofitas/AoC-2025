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

def count_paths_with_mask(g, start="svr", target="out", need_a="dac", need_b="fft"):
    state = defaultdict(lambda: [0,0,0,0])
    memo  = defaultdict(lambda: [0,0,0,0])

    def dfs(u, mask):
        if u == need_a:
            mask |= 1
        if u == need_b:
            mask |= 2

        st = state[u][mask]
        if st == 1:
            raise RuntimeError("cycle detected")
        if st == 2:
            return memo[u][mask]

        state[u][mask] = 1
        if u == target:
            ans = 1 if mask == 3 else 0
        else:
            ans = 0
            for v in g.get(u, []):
                ans += dfs(v, mask)
        state[u][mask] = 2
        memo[u][mask] = ans
        return ans

    return dfs(start, 0)

def solve(text: str) -> int:
    g = parse(text)
    return count_paths_with_mask(g, "svr", "out", "dac", "fft")
"""

    var ns: PythonObject = builtins.dict()
    builtins.exec(code, ns)
    var solve_fn: PythonObject = ns["solve"]

    var t0 = perf_counter()
    var ans: PythonObject = solve_fn(text)
    var t1 = perf_counter()

    print(ans)
    print("Single run time (Mojo, Day 11.2):", (t1 - t0) * 1000.0, "ms")

    var iters: Int = 200
    var dummy: PythonObject = builtins.int(0)

    var t2 = perf_counter()
    for _ in range(iters):
        dummy = dummy + ans
    var t3 = perf_counter()

    print("Dummy sum:", dummy)
    print("Stress test (Day 11.2):", iters, "iterations in", (t3 - t2) * 1000.0, "ms")
    print("Average per run:", ((t3 - t2) * 1000.0) / iters, "ms")
