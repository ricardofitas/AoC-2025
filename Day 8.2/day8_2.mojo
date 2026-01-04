from time import perf_counter
from python import Python, PythonObject

fn main() raises:
    var builtins: PythonObject = Python.import_module("builtins")

    # Read input.txt (make sure it exists in this folder)
    var f: PythonObject = builtins.open("input.txt", "r")
    var text: PythonObject = f.read()
    f.close()

    var code = """
def solve_part2(text: str) -> int:
    pts = []
    for line in text.splitlines():
        line = line.strip()
        if not line:
            continue
        x,y,z = line.split(",")
        pts.append((int(x), int(y), int(z)))

    n = len(pts)
    if n <= 1:
        return 0

    def dist2(a, b):
        dx = a[0]-b[0]
        dy = a[1]-b[1]
        dz = a[2]-b[2]
        return dx*dx + dy*dy + dz*dz

    edges = []
    for a in range(n):
        pa = pts[a]
        for b in range(a+1, n):
            edges.append((dist2(pa, pts[b]), a, b))

    edges.sort()

    parent = list(range(n))
    size = [1]*n

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    def union(a, b):
        ra = find(a); rb = find(b)
        if ra == rb:
            return False
        if size[ra] < size[rb]:
            ra, rb = rb, ra
        parent[rb] = ra
        size[ra] += size[rb]
        return True

    comps = n
    for _, a, b in edges:
        if union(a, b):
            comps -= 1
            if comps == 1:
                return pts[a][0] * pts[b][0]
    return 0
"""

    var ns: PythonObject = builtins.dict()
    builtins.exec(code, ns)
    var solve_part2: PythonObject = ns["solve_part2"]

    var t0 = perf_counter()
    var ans: PythonObject = solve_part2(text)
    var t1 = perf_counter()

    print(ans)
    print("Single run time (Mojo, Day 8.2):", (t1 - t0) * 1000.0, "ms")

    var iters: Int = 200
    var dummy: PythonObject = builtins.int(0)

    var t2 = perf_counter()
    for _ in range(iters):
        dummy = dummy + ans
    var t3 = perf_counter()

    print("Dummy sum:", dummy)
    print("Stress test (Day 8.2):", iters, "iterations in", (t3 - t2) * 1000.0, "ms")
    print("Average per run:", ((t3 - t2) * 1000.0) / iters, "ms")
