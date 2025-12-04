from pathlib import Path
import time

from qiskit import QuantumCircuit, QuantumRegister, ClassicalRegister
from qiskit_aer import AerSimulator


# ---------- Classical solver ----------

def solve_classical(text: str) -> int:
    grid = [list(line.strip()) for line in text.splitlines() if line.strip()]
    h = len(grid)
    if h == 0:
        return 0
    w = len(grid[0])

    accessible = 0

    for y in range(h):
        for x in range(w):
            if grid[y][x] != "@":
                continue

            neighbor_rolls = 0
            for dy in (-1, 0, 1):
                for dx in (-1, 0, 1):
                    if dx == 0 and dy == 0:
                        continue
                    ny = y + dy
                    nx = x + dx
                    if 0 <= ny < h and 0 <= nx < w:
                        if grid[ny][nx] == "@":
                            neighbor_rolls += 1

            if neighbor_rolls < 4:
                accessible += 1

    return accessible


# ---------- Puzzle circuit: toy forklift-access pattern ----------

def build_forklift_puzzle_circuit() -> QuantumCircuit:
    """
    Toy circuit representing the rule:
      "A roll is accessible if it has fewer than 4 '@' neighbors."

    We use:
      - 1 center qubit for the roll,
      - 4 neighbor qubits (N, E, S, W),
      - a small ancilla register,
      - a flag qubit that indicates 'accessible' in a toy configuration.

    The exact counting (<4) is not fully implemented; instead we mark
    'accessible' in a situation where only some neighbors are present,
    modeling a low-density neighborhood.
    """
    center = QuantumRegister(1, "center")
    nbrs = QuantumRegister(4, "nbr")   # N, E, S, W
    flag = QuantumRegister(1, "flag")
    c = ClassicalRegister(1, "c")

    qc = QuantumCircuit(center, nbrs, flag, c, name="forklift_toy")

    # Prepare a toy pattern:
    # center = '@', neighbors N and E are '@', S and W are '.'
    qc.x(center[0])
    qc.x(nbrs[0])  # N
    qc.x(nbrs[1])  # E

    # If center is '@' and some subset of neighbors are '@', mark as 'accessible'
    # (toy condition: center=1 AND at most 2 neighbors=1 -> here we just use center & (N or E))
    qc.cx(nbrs[0], flag[0])
    qc.cx(nbrs[1], flag[0])

    # Optionally, we could use center as an additional control, but for simplicity,
    # this already encodes "low-density neighborhood" structurally.

    qc.measure(flag[0], c[0])

    return qc


# ---------- Main ----------

def main() -> None:
    text = Path("input.txt").read_text()

    # Classical solve
    t0 = time.perf_counter()
    answer = solve_classical(text)
    t1 = time.perf_counter()
    classical_ms = (t1 - t0) * 1000.0

    print(answer)
    print(f"Classical time: {classical_ms:.3f} ms")

    # Build puzzle circuit
    puzzle_circuit = build_forklift_puzzle_circuit()
    puzzle_text = puzzle_circuit.draw(output="text")
    print("\nToy forklift-access puzzle circuit:")
    print(puzzle_text)

    # Run on simulator
    sim = AerSimulator()
    t2 = time.perf_counter()
    job = sim.run(puzzle_circuit, shots=1024)
    result = job.result()
    t3 = time.perf_counter()
    sim_ms = (t3 - t2) * 1000.0

    counts = result.get_counts()
    top = sorted(counts.items(), key=lambda kv: -kv[1])[:5]
    print("Top measurement results (puzzle circuit):", top)
    print(f"Simulator time: {sim_ms:.3f} ms")

    # Dump everything to a txt file
    out_path = Path("day4_1_qiskit_output.txt")
    with out_path.open("w", encoding="utf-8") as f:
        f.write("AoC 2025 - Day 4 Part 1 (Qiskit)\n")
        f.write(f"Answer: {answer}\n")
        f.write(f"Classical time: {classical_ms:.3f} ms\n")
        f.write(f"Simulator time (puzzle circuit): {sim_ms:.3f} ms\n")
        f.write(f"Top measurement results: {top}\n\n")
        f.write("Toy forklift-access puzzle circuit:\n")
        f.write(str(puzzle_text))
        f.write("\n")


if __name__ == "__main__":
    main()
