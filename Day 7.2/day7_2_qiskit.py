from pathlib import Path
import time

from qiskit import QuantumCircuit, QuantumRegister, ClassicalRegister
from qiskit_aer import AerSimulator


# ----- Classical quantum-timeline solver -----


def solve_quantum(text: str) -> int:
    lines = text.splitlines()
    if not lines:
        return 0

    grid = [list(line) for line in lines]
    h = len(grid)
    w = max(len(row) for row in grid)

    # Pad rows to same width for safety
    for r in range(h):
        if len(grid[r]) < w:
            grid[r] += [" "] * (w - len(grid[r]))

    # Find S
    start = None
    for r in range(h):
        for c in range(w):
            if grid[r][c] == "S":
                start = (r, c)
                break
        if start is not None:
            break

    if start is None:
        return 0

    sr, sc = start

    # dp[r][c]: number of timelines at cell (r, c)
    dp = [[0 for _ in range(w)] for _ in range(h)]
    dp[sr][sc] = 1

    total_timelines = 0

    for r in range(sr, h):
        for c in range(w):
            ways = dp[r][c]
            if ways == 0:
                continue

            nr = r + 1
            if nr >= h:
                total_timelines += ways
                continue

            ch = grid[nr][c]
            if ch == "^":
                # Split
                if c > 0:
                    dp[nr][c - 1] += ways
                if c + 1 < w:
                    dp[nr][c + 1] += ways
            else:
                # Straight down
                dp[nr][c] += ways

    return total_timelines


# ----- Toy "many-worlds splitter chain" circuit -----


def build_splitter_chain_circuit(num_splitters: int = 3) -> QuantumCircuit:
    """
    Toy circuit to illustrate exponential branching:

    - A single qubit starts in |0>.
    - Each splitter is modeled as a Hadamard gate H, putting it into a
      superposition of "left" and "right".
    - After N splitters, the qubit has 2^N basis paths in superposition.

    This does NOT simulate the real grid, it's just a visual metaphor
    for many-worlds branching.
    """
    path = QuantumRegister(1, "path")
    c = ClassicalRegister(1, "c")
    qc = QuantumCircuit(path, c, name="splitter_chain")

    # Start in |0>, then apply H for each "splitter"
    for _ in range(num_splitters):
        qc.h(path[0])

    qc.measure(path[0], c[0])

    return qc


def main() -> None:
    text = Path("input.txt").read_text()

    # Classical quantum-timeline solver
    t0 = time.perf_counter()
    answer = solve_quantum(text)
    t1 = time.perf_counter()
    classical_ms = (t1 - t0) * 1000.0

    print(answer)
    print(f"Classical time: {classical_ms:.3f} ms")

    # Toy circuit
    qc = build_splitter_chain_circuit(num_splitters=3)
    circuit_text = qc.draw(output="text")
    print("\nToy quantum splitter-chain puzzle circuit:")
    print(circuit_text)

    sim = AerSimulator()
    t2 = time.perf_counter()
    job = sim.run(qc, shots=1024)
    result = job.result()
    t3 = time.perf_counter()
    sim_ms = (t3 - t2) * 1000.0

    counts = result.get_counts()
    top = sorted(counts.items(), key=lambda kv: -kv[1])[:5]
    print("Top measurement results (puzzle circuit):", top)
    print(f"Simulator time: {sim_ms:.3f} ms")

    # Save summary to txt
    out_path = Path("day7_2_qiskit_output.txt")
    with out_path.open("w", encoding="utf-8") as f:
        f.write("AoC 2025 - Day 7 Part 2 (Qiskit)\n")
        f.write(f"Answer (timelines): {answer}\n")
        f.write(f"Classical time: {classical_ms:.3f} ms\n")
        f.write(f"Simulator time (puzzle circuit): {sim_ms:.3f} ms\n")
        f.write(f"Top measurement results: {top}\n\n")
        f.write("Toy splitter-chain puzzle circuit:\n")
        f.write(str(circuit_text))
        f.write("\n")


if __name__ == "__main__":
    main()
