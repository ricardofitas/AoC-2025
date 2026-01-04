import time
import heapq

from qiskit import QuantumCircuit
from qiskit_aer import AerSimulator


def parse_points(path="input.txt"):
    pts = []
    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            x, y, z = line.split(",")
            pts.append((int(x), int(y), int(z)))
    return pts


def dist2(a, b):
    dx = a[0] - b[0]
    dy = a[1] - b[1]
    dz = a[2] - b[2]
    return dx * dx + dy * dy + dz * dz


class DSU:
    def __init__(self, n):
        self.p = list(range(n))
        self.sz = [1] * n

    def find(self, x):
        while self.p[x] != x:
            self.p[x] = self.p[self.p[x]]
            x = self.p[x]
        return x

    def union(self, a, b):
        ra = self.find(a)
        rb = self.find(b)
        if ra == rb:
            return
        if self.sz[ra] < self.sz[rb]:
            ra, rb = rb, ra
        self.p[rb] = ra
        self.sz[ra] += self.sz[rb]


def solve_k(pts, k=1000):
    n = len(pts)
    total_pairs = n * (n - 1) // 2
    k = min(k, total_pairs)
    if k <= 0:
        return 1

    # Keep K smallest edges using a max-heap behavior via negative keys:
    # store (-d2, -a, -b, a, b). The "worst kept" is heap[0] (most negative).
    heap = []

    for a in range(n):
        pa = pts[a]
        for b in range(a + 1, n):
            d2 = dist2(pa, pts[b])
            item = (-d2, -a, -b, a, b)

            if len(heap) < k:
                heapq.heappush(heap, item)
            else:
                worst = heap[0]
                if item > worst:
                    heapq.heapreplace(heap, item)

    dsu = DSU(n)
    for _, _, _, a, b in heap:
        dsu.union(a, b)

    comp = {}
    for i in range(n):
        r = dsu.find(i)
        comp[r] = comp.get(r, 0) + 1

    sizes = sorted(comp.values(), reverse=True)
    a = sizes[0] if len(sizes) > 0 else 0
    b = sizes[1] if len(sizes) > 1 else 0
    c = sizes[2] if len(sizes) > 2 else 0
    return a * b * c


def toy_circuit():
    qc = QuantumCircuit(5, 1)
    qc.x(0)
    qc.x(2)
    qc.cx(0, 4)
    qc.ccx(2, 0, 4)
    qc.cx(1, 4)
    qc.measure(4, 0)
    return qc


def main():
    pts = parse_points("input.txt")

    t0 = time.perf_counter()
    ans = solve_k(pts, 1000)
    t1 = time.perf_counter()

    print(ans)
    print(f"Classical time: {(t1 - t0)*1000:.3f} ms\n")

    qc = toy_circuit()
    print("Toy closest-pair comparison circuit:")
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
