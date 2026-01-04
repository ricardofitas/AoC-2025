from time import perf_counter
from python import Python, PythonObject

fn main() raises:
    var builtins: PythonObject = Python.import_module("builtins")

    # Read input.txt via Python
    var f: PythonObject = builtins.open("input.txt", "r")
    var text: PythonObject = f.read()
    f.close()

    # Embedded Python solver (same logic as Rust: keep 1000 closest pairs via heap)
    var code = """
import heapq

def solve_k(text: str, k: int = 1000) -> int:
    pts = []
    for line in text.splitlines():
        line = line.strip()
        if not line:
            continue
        x,y,z = line.split(",")
        pts.append((int(x), int(y), int(z)))

    n = len(pts)
    if n == 0:
        return 0

    total_pairs = n*(n-1)//2
    k = min(k, total_pairs)
    if k <= 0:
        return 1

    # store (-d2, -a, -b, a, b)
    heap = []

    def dist2(a, b):
        dx = a[0]-b[0]
        dy = a[1]-b[1]
        dz = a[2]-b[2]
        return dx*dx + dy*dy + dz*dz

    for a in range(n):
        pa = pts[a]
        for b in range(a+1, n):
            d2 = dist2(pa, pts[b])
            item = (-d2, -a, -b, a, b)
            if len(heap) < k:
                heapq.heappush(heap, item)
            else:
                worst = heap[0]
                if item > worst:
                    heapq.heapreplace(heap, item)

    # DSU
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
            return
        if size[ra] < size[rb]:
            ra, rb = rb, ra
        parent[rb] = ra
        size[ra] += size[rb]

    for _,_,_,a,b in heap:
        union(a,b)

    comp = {}
    for i in range(n):
        r = find(i)
        comp[r] = comp.get(r, 0) + 1

    sizes = sorted(comp.values(), reverse=True)
    a = sizes[0] if len(sizes) > 0 else 0
    b = sizes[1] if len(sizes) > 1 else 0
    c = sizes[2] if len(sizes) > 2 else 0
    return a*b*c
"""

    # Execute code into a Python dict namespace
    var ns: PythonObject = builtins.dict()
    builtins.exec(code, ns)

    var solve_k: PythonObject = ns["solve_k"]

    var t0 = perf_counter()
    var ans: PythonObject = solve_k(text, 1000)
    var t1 = perf_counter()

    print(ans)
    print("Single run time (Mojo, Day 8.1):", (t1 - t0) * 1000.0, "ms")

    # Stress test (cheap loop like your other days)
    var iters: Int = 200
    var dummy: PythonObject = builtins.int(0)

    var t2 = perf_counter()
    for _ in range(iters):
        dummy = dummy + ans
    var t3 = perf_counter()

    print("Dummy sum:", dummy)
    print("Stress test (Day 8.1):", iters, "iterations in", (t3 - t2) * 1000.0, "ms")
    print("Average per run:", ((t3 - t2) * 1000.0) / iters, "ms")
