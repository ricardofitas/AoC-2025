from pathlib import Path
import time

from qiskit import QuantumCircuit, QuantumRegister, ClassicalRegister
from qiskit_aer import AerSimulator


# ---------- Classical solver ----------

def parse_sections(text: str):
    parts = text.split("\n\n", 1)
    ranges_section = parts[0] if parts else ""
    ids_section = parts[1] if len(parts) > 1 else ""
    return ranges_section, ids_section


def parse_ranges(ranges_section: str):
    ranges = []
    for line in ranges_section.splitlines():
        line = line.strip()
        if not line:
            continue
        if "-" not in line:
            continue
        a_str, b_str = line.split("-", 1)
        a = int(a_str.strip())
        b = int(b_str.strip())
        lo, hi = (a, b) if a <= b else (b, a)
        ranges.append((lo, hi))
    return ranges


def solve_classical(text: str) -> int:
    ranges_section, ids_section = parse_sections(text)
    ranges = parse_ranges(ranges_section)

    fresh_count = 0
    for line in ids_section.splitlines():
        line = line.strip()
        if not line:
            continue
        val = int(line)
        fresh = any(lo <= val <= hi for (lo, hi) in ranges)
        if fresh:
            fresh_count += 1
    return fresh_count


# ---------- Toy "fresh range" puzzle circuit ----------

def build_fresh_range_puzzle_circuit() -> QuantumCircuit:
    """
    Toy circuit representing:
      - An 'ID' qubit
      - Two 'range' qubits
      - A flag qubit that lights up if ID is considered 'fresh'
        (ID ∈ range1 OR ID ∈ range2, in a toy encoding).

    This is not encoding real numbers; it's just structural.
    """
    id_q = QuantumRegister(1, "id")
    range_q = QuantumRegister(2, "rng")   # pretend two ranges
    flag = QuantumRegister(1, "flag")
    c = ClassicalRegister(1, "c")

    qc = QuantumCircuit(id_q, range_q, flag, c, name="fresh_range_toy")

    # Toy preparation: say ID is active and both ranges are "valid"
    qc.x(id_q[0])
    qc.x(range_q[0])
    qc.x(range_q[1])

    # Toy rule:
    # If id == 1 and (range0 == 1 or range1 == 1), set flag
    qc.ccx(id_q[0], range_q[0], flag[0])
    qc.ccx(id_q[0], range_q[1], flag[0])

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

    # Build toy circuit
    puzzle_circuit = build_fresh_range_puzzle_circuit()
    puzzle_text = puzzle_circuit.draw(output="text")
    print("\nToy fresh-range puzzle circuit:")
    print(puzzle_text)

    # Run simulator
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

    # Dump to txt
    out_path = Path("day5_1_qiskit_output.txt")
    with out_path.open("w", encoding="utf-8") as f:
        f.write("AoC 2025 - Day 5 Part 1 (Qiskit)\n")
        f.write(f"Answer: {answer}\n")
        f.write(f"Classical time: {classical_ms:.3f} ms\n")
        f.write(f"Simulator time (puzzle circuit): {sim_ms:.3f} ms\n")
        f.write(f"Top measurement results: {top}\n\n")
        f.write("Toy fresh-range puzzle circuit:\n")
        f.write(str(puzzle_text))
        f.write("\n")


if __name__ == "__main__":
    main()
