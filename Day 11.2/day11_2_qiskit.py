import time
from collections import defaultdict

from qiskit import QuantumCircuit
from qiskit_aer import AerSimulator


def parse(text: str):
    g = defaultdict(list)
    nodes = set()
    for line in text.splitlines():
        line = line.strip()
        if not line:
            continue
        lhs, rhs = line.split(":", 1)
        u = lhs.strip()
        nodes.add(u)
        for v in rhs.strip().split():
            nodes.add(v)
            g[u].append(v)
    return g, nodes


def count_paths_with_mask(g, start="svr", target="out", need_a="dac", need_b="fft"):
    # mask bits: 1=need_a seen, 2=need_b seen
    state = defaultdict(lambda: [0, 0, 0, 0])
    memo = defaultdict(lambda: [0, 0, 0, 0])

    def dfs(u, mask):
        if u == need_a:
            mask |= 1
        if u == need_b:
            mask |= 2

        st = state[u][mask]
        if st == 1:
            raise RuntimeError("cycle detected (infinite paths?)")
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

    g, _ = parse(text)

    t0 = time.perf_counter()
    ans = count_paths_with_mask(g, "svr", "out", "dac", "fft")
    t1 = time.perf_counter()

    print(ans)
    print(f"Classical time: {(t1 - t0)*1000:.3f} ms\n")

    qc = toy_circuit()
    print("Toy masked-path demo circuit:")
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
