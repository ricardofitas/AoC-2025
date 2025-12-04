from pathlib import Path
import time

from qiskit import QuantumCircuit, QuantumRegister, ClassicalRegister
from qiskit_aer import AerSimulator


# ---------- Classical solver (same idea as Rust/Mojo) ----------

def max_joltage_12(line: str) -> int:
    K = 12
    digits = [int(ch) for ch in line.strip() if ch.isdigit()]
    n = len(digits)
    if n < K:
        return 0

    result = []
    start_idx = 0
    for pos in range(K):
        remaining_slots = K - pos
        end_idx = n - remaining_slots
        best_digit = -1
        best_pos = start_idx
        for i in range(start_idx, end_idx + 1):
            d = digits[i]
            if d > best_digit:
                best_digit = d
                best_pos = i
                if d == 9:
                    break
        result.append(best_digit)
        start_idx = best_pos + 1

    val = 0
    for d in result:
        val = val * 10 + d
    return val


def solve_classical(text: str) -> int:
    total = 0
    for line in text.splitlines():
        line = line.strip()
        if not line:
            continue
        total += max_joltage_12(line)
    return total


# ---------- Puzzle circuit: toy subsequence chooser ----------

def build_subsequence_puzzle_circuit() -> QuantumCircuit:
    """
    A toy circuit that encodes the idea of "choosing the best subsequence".

    We use 4 positions with fixed 'digit strengths' and a 2-digit subsequence (K=2):
      positions 0..3
    We mark a flag qubit if the subsequence (0,1) is selected as 'best'
    under some toy condition (e.g., pos_0 is strong enough).
    This is NOT encoding your full inputs; it's a structural model.
    """
    pos = QuantumRegister(4, "pos")
    flag = QuantumRegister(1, "flag")
    c = ClassicalRegister(1, "c")

    qc = QuantumCircuit(pos, flag, c, name="subseq_toy")

    # Prepare a toy state where position 0 is 'stronger'
    qc.x(pos[0])

    # If pos[0] is on, we treat subsequence (0,1) as 'best':
    qc.cx(pos[0], flag[0])

    # Measure the flag (which subsequence was selected)
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

    # Build toy subsequence puzzle circuit
    puzzle_circuit = build_subsequence_puzzle_circuit()
    puzzle_text = puzzle_circuit.draw(output="text")
    print("\nToy subsequence puzzle circuit (structure for choosing best subsequence):")
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
    out_path = Path("day3_2_qiskit_output.txt")
    with out_path.open("w", encoding="utf-8") as f:
        f.write("AoC 2025 - Day 3 Part 2 (Qiskit)\n")
        f.write(f"Answer: {answer}\n")
        f.write(f"Classical time: {classical_ms:.3f} ms\n")
        f.write(f"Simulator time (puzzle circuit): {sim_ms:.3f} ms\n")
        f.write(f"Top measurement results: {top}\n\n")
        f.write("Toy subsequence puzzle circuit:\n")
        f.write(str(puzzle_text))
        f.write("\n")


if __name__ == "__main__":
    main()
