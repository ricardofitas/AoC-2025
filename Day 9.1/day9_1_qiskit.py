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
            x, y = s.split(",")
            pts.append((int(x), int(y)))
    return pts


def solve(pts):
    best = 0
    n = len(pts)
    for i in range(n):
        xi, yi = pts[i]
        for j in range(i + 1, n):
            xj, yj = pts[j]
            area = (abs(xi - xj)+1) * (abs(yi - yj)+1)
            if area > best:
                best = area
    return best


def toy_circuit():
    qc = QuantumCircuit(3, 1)
    qc.x(0)
    qc.cx(0, 2)
    qc.measure(2, 0)
    return qc


def main():
    pts = parse_points("input.txt")

    t0 = time.perf_counter()
    ans = solve(pts)
    t1 = time.perf_counter()

    print(ans)
    print(f"Classical time: {(t1 - t0)*1000:.3f} ms\n")

    qc = toy_circuit()
    print("Toy rectangle-flag circuit:")
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
