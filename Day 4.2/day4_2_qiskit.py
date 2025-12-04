from pathlib import Path
import time

from qiskit import QuantumCircuit, QuantumRegister, ClassicalRegister
from qiskit_aer import AerSimulator


# ---------- Classical iterative-removal solver ----------

def parse_grid(text: str):
    return [list(line.strip()) for line in text.splitlines() if line.strip()]


def count_neighbors(grid, y, x):
    h = len(grid)
    w = len(grid[0])
    count = 0
    for dy in (-1, 0, 1):
        for dx in (-1, 0, 1):
            if dx == 0 and dy == 0:
                continue
            ny = y + dy
            nx = x + dx
            if 0 <= ny < h and 0 <= nx < w and grid[ny][nx] == "@":
                count += 1
    return count


def solve_classical(text: str) -> int:
    grid = parse_grid(text)
    if not grid:
        return 0

    h = len(grid)
    w = len(grid[0])
    total_removed = 0

    while True:
        to_remove = []

        for y in range(h):
            for x in range(w):
                if grid[y][x] != "@":
                    continue
                neigh = count_neighbors(grid, y, x)
                if neigh < 4:
                    to_remove.append((y, x))

        if not to_remove:
            break

        for y, x in to_remove:
            if grid[y][x] == "@":
                grid[y][x] = "."
                total_removed += 1

    return total_removed


# ---------- Toy iterative-removal puzzle circuit ----------

def build_iterative_removal_puzzle_circuit() -> QuantumCircuit:
    """
    Toy circuit representing one 'round' of removal:

    - center qubit: a roll '@'
    - 4 neighbor qubits
    - flag qubit that is set if the roll is considered 'removable'
      (toy condition: center=1 and at most 2 neighbors are 1, modeled simply).

    This is not a full simulation, just a structural model of the rule.
    """
    center = QuantumRegister(1, "center")
    nbrs = QuantumRegister(4, "nbr")
    flag = QuantumRegister(1, "flag")
    c = ClassicalRegister(1, "c")

    qc = QuantumCircuit(center, nbrs, flag, c, name="iter_removal_toy")

    # Prepare toy configuration:
    # center roll '@', and 2 neighbors '@', others empty.
    qc.x(center[0])
    qc.x(nbrs[0])
    qc.x(nbrs[1])

    # Toy rule:
    # If center is 1 and (nbr0 or nbr1) is 1, mark as 'removable'
    # (low-density neighborhood in a simplified way).
    qc.ccx(center[0], nbrs[0], flag[0])
    qc.ccx(center[0], nbrs[1], flag[0])

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

    # Build toy iterative-removal circuit
    puzzle_circuit = build_iterative_removal_puzzle_circuit()
    puzzle_text = puzzle_circuit.draw(output="text")
    print("\nToy iterative-removal puzzle circuit:")
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

    # Dump log to txt
    out_path = Path("day4_2_qiskit_output.txt")
    with out_path.open("w", encoding="utf-8") as f:
        f.write("AoC 2025 - Day 4 Part 2 (Qiskit)\n")
        f.write(f"Answer: {answer}\n")
        f.write(f"Classical time: {classical_ms:.3f} ms\n")
        f.write(f"Simulator time (puzzle circuit): {sim_ms:.3f} ms\n")
        f.write(f"Top measurement results: {top}\n\n")
        f.write("Toy iterative-removal puzzle circuit:\n")
        f.write(str(puzzle_text))
        f.write("\n")


if __name__ == "__main__":
    main()
