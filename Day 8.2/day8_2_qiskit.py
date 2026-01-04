import time

from qiskit import QuantumCircuit
from qiskit_aer import AerSimulator


def parse_points(path="input.txt"):
    pts = []
    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            s = line.strip()
            if not s:
                continue
            x, y, z = s.split(",")
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
            return False
        if self.sz[ra] < self.sz[rb]:
            ra, rb = rb, ra
        self.p[rb] = ra
        self.sz[ra] += self.sz[rb]
        return True


def solve_part2(pts):
    n = len(pts)
    if n <= 1:
        return 0

    edges = []
    for a in range(n):
        pa = pts[a]
        for b in range(a + 1, n):
            edges.append((dist2(pa, pts[b]), a, b))

    edges.sort()  # by (d2, a, b)

    dsu = DSU(n)
    comps = n
    for _, a, b in edges:
        if dsu.union(a, b):
            comps -= 1
            if comps == 1:
                return pts[a][0] * pts[b][0]
    return 0


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
    ans = solve_part2(pts)
    t1 = time.perf_counter()

    print(ans)
    print(f"Classical time: {(t1 - t0)*1000:.3f} ms\n")

    qc = toy_circuit()
    print("Toy last-connection demo circuit:")
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
