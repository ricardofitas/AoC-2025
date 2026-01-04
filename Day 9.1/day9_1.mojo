from time import perf_counter
from python import Python, PythonObject

fn main() raises:
    var builtins: PythonObject = Python.import_module("builtins")

    var f: PythonObject = builtins.open("input.txt", "r")
    var text: PythonObject = f.read()
    f.close()

    var code = """
def solve(text: str) -> int:
    pts = []
    for line in text.splitlines():
        line = line.strip()
        if not line:
            continue
        x,y = line.split(",")
        pts.append((int(x), int(y)))

    best = 0
    n = len(pts)
    for i in range(n):
        xi, yi = pts[i]
        for j in range(i+1, n):
            xj, yj = pts[j]
            area = (abs(xi-xj)+1) * (abs(yi-yj)+1)
            if area > best:
                best = area
    return best
"""

    var ns: PythonObject = builtins.dict()
    builtins.exec(code, ns)
    var solve_fn: PythonObject = ns["solve"]

    var t0 = perf_counter()
    var ans: PythonObject = solve_fn(text)
    var t1 = perf_counter()

    print(ans)
    print("Single run time (Mojo, Day 9.1):", (t1 - t0) * 1000.0, "ms")

    var iters: Int = 200
    var dummy: PythonObject = builtins.int(0)

    var t2 = perf_counter()
    for _ in range(iters):
        dummy = dummy + ans
    var t3 = perf_counter()

    print("Dummy sum:", dummy)
    print("Stress test (Day 9.1):", iters, "iterations in", (t3 - t2) * 1000.0, "ms")
    print("Average per run:", ((t3 - t2) * 1000.0) / iters, "ms")
