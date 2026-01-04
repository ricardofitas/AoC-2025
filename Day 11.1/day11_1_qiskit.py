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


def count_paths(g, start="you", target="out"):
    # DFS + memo (assumes DAG on reachable subgraph; if cycle -> raise)
    state = {}  # 0/1/2
    memo = {}

    def dfs(u):
        st = state.get(u, 0)
        if st == 1:
            raise RuntimeError("cycle detected (infinite paths?)")
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


def toy_circuit():
    qc = QuantumCircuit(3, 1)
    qc.x(0)
    qc.cx(0, 1)
    qc.cx(1, 2)
    qc.measure(2, 0)
    return qc


def main():
    with open("input.txt", "r", encoding="utf-8") as f:
        text = f.read()

    g, _ = parse(text)

    t0 = time.perf_counter()
    ans = count_paths(g, "you", "out")
    t1 = time.perf_counter()

    print(ans)
    print(f"Classical time: {(t1 - t0)*1000:.3f} ms\n")

    qc = toy_circuit()
    print("Toy path-propagation demo circuit:")
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
