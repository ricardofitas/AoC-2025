from pathlib import Path
import time

from qiskit import QuantumCircuit, QuantumRegister, ClassicalRegister
from qiskit_aer import AerSimulator


# ---------- Classical solver ----------

def max_joltage_for_bank(line: str) -> int:
    digits = [int(ch) for ch in line.strip() if ch.isdigit()]
    n = len(digits)
    if n < 2:
        return 0

    # suffix_max[i] = max digit from i..end
    suffix_max = [0] * n
    suffix_max[-1] = digits[-1]
    for i in range(n - 2, -1, -1):
        suffix_max[i] = max(digits[i], suffix_max[i + 1])

    best = 0
    for i in range(n - 1):
        val = 10 * digits[i] + suffix_max[i + 1]
        if val > best:
            best = val
    return best


def solve_classical(text: str) -> int:
    total = 0
    for line in text.splitlines():
        line = line.strip()
        if not line:
            continue
        total += max_joltage_for_bank(line)
    return total


# ---------- Puzzle circuit: toy "max pair" logic ----------

def build_max_pair_puzzle_circuit() -> QuantumCircuit:
    """
    Conceptual puzzle circuit:
    - We imagine a tiny 'bank' with 3 positions and their 'digit strengths':
        position 0: strong
        position 1: medium
        position 2: weak
    - We encode position strengths as qubits and mark which pair is the best.
    This is not a full 0-9 digit encoding, but a structural model of
    "choosing the pair with the largest (tens, ones)" ordering.
    """
    pos = QuantumRegister(3, "pos")   # represent 3 positions (0,1,2)
    flag = QuantumRegister(3, "flag") # flag[ij] indicates pair (i,j) is "best"
    c = ClassicalRegister(3, "c")

    qc = QuantumCircuit(pos, flag, c, name="max_pair_toy")

    # Prepare a toy state: assume pos[0] >= pos[1] >= pos[2]
    # (for illustration, just set pos[0] = 1, others 0)
    qc.x(pos[0])

    # Encode: (0,1) is the best pair among (0,1), (0,2), (1,2)
    # by flipping flag[0] if pos[0] is 'on' and others not better.
    # Here it's purely illustrative with simple controls.
    qc.cx(pos[0], flag[0])  # say (0,1) is best if 0 is on

    # We can mark (0,2) and (1,2) as not-best by leaving their flags at 0,
    # or we could put some conditionals here for more structure.

    # Measure flags (which pair was 'selected')
    for i in range(3):
        qc.measure(flag[i], c[i])

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

    # Build the toy "max pair" circuit
    puzzle_circuit = build_max_pair_puzzle_circuit()
    puzzle_text = puzzle_circuit.draw(output="text")
    print("\nToy max-pair puzzle circuit (structure for choosing best pair):")
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
    out_path = Path("day3_1_qiskit_output.txt")
    with out_path.open("w", encoding="utf-8") as f:
        f.write("AoC 2025 - Day 3 Part 1 (Qiskit)\n")
        f.write(f"Answer: {answer}\n")
        f.write(f"Classical time: {classical_ms:.3f} ms\n")
        f.write(f"Simulator time (puzzle circuit): {sim_ms:.3f} ms\n")
        f.write(f"Top measurement results: {top}\n\n")
        f.write("Toy max-pair puzzle circuit:\n")
        f.write(str(puzzle_text))
        f.write("\n")


if __name__ == "__main__":
    main()
