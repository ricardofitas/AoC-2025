from pathlib import Path
import time

from qiskit import QuantumCircuit, QuantumRegister, ClassicalRegister
from qiskit_aer import AerSimulator


# ----- Classical solver (same logic as Rust) -----


def solve_classical(text: str) -> int:
    lines = text.splitlines()
    if not lines:
        return 0

    grid = [list(line) for line in lines]
    h = len(grid)
    w = max(len(row) for row in grid)

    # pad rows to same width
    for r in range(h):
        if len(grid[r]) < w:
            grid[r] += [" "] * (w - len(grid[r]))

    # locate S
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
    from collections import deque

    visited = [[False] * w for _ in range(h)]
    q = deque()
    q.append((sr, sc))

    splits = 0

    while q:
        r, c = q.popleft()
        nr = r + 1
        if nr >= h:
            continue
        nc = c

        if visited[nr][nc]:
            continue
        visited[nr][nc] = True

        ch = grid[nr][nc]
        if ch == "^":
            splits += 1
            if nc > 0:
                q.append((nr, nc - 1))
            if nc + 1 < w:
                q.append((nr, nc + 1))
        else:
            q.append((nr, nc))

    return splits


# ----- Toy manifold-splitter puzzle circuit -----


def build_splitter_puzzle_circuit() -> QuantumCircuit:
    """
    Toy circuit to illustrate a single splitter:

    - beam_in: indicates a tachyon beam arriving from above.
    - split: marks the presence of a splitter cell.
    - left, right: represent the two outgoing beams.
    - flag: set if a valid split occurs (beam_in AND split).

    This is purely structural, not a full manifold simulation.
    """
    beam_in = QuantumRegister(1, "beam_in")
    split = QuantumRegister(1, "split")
    left = QuantumRegister(1, "left")
    right = QuantumRegister(1, "right")
    flag = QuantumRegister(1, "flag")
    c = ClassicalRegister(3, "c")

    qc = QuantumCircuit(beam_in, split, left, right, flag, c, name="splitter_toy")

    # Prepare beam_in = 1 (beam present), split = 1 (splitter cell)
    qc.x(beam_in[0])
    qc.x(split[0])

    # If beam_in AND split, create outgoing beams on left and right
    qc.ccx(beam_in[0], split[0], left[0])
    qc.ccx(beam_in[0], split[0], right[0])

    # Flag is 1 if at least one outgoing beam is created
    qc.cx(left[0], flag[0])
    qc.cx(right[0], flag[0])

    qc.measure(left[0], c[0])
    qc.measure(right[0], c[1])
    qc.measure(flag[0], c[2])

    return qc


def main() -> None:
    text = Path("input.txt").read_text()

    t0 = time.perf_counter()
    answer = solve_classical(text)
    t1 = time.perf_counter()
    classical_ms = (t1 - t0) * 1000.0

    print(answer)
    print(f"Classical time: {classical_ms:.3f} ms")

    # Build toy circuit
    qc = build_splitter_puzzle_circuit()
    circuit_text = qc.draw(output="text")
    print("\nToy splitter manifold puzzle circuit:")
    print(circuit_text)

    # Simulate toy circuit
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
    out_path = Path("day7_1_qiskit_output.txt")
    with out_path.open("w", encoding="utf-8") as f:
        f.write("AoC 2025 - Day 7 Part 1 (Qiskit)\n")
        f.write(f"Answer: {answer}\n")
        f.write(f"Classical time: {classical_ms:.3f} ms\n")
        f.write(f"Simulator time (puzzle circuit): {sim_ms:.3f} ms\n")
        f.write(f"Top measurement results: {top}\n\n")
        f.write("Toy splitter manifold puzzle circuit:\n")
        f.write(str(circuit_text))
        f.write("\n")


if __name__ == "__main__":
    main()
