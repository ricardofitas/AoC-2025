from pathlib import Path
import time

from qiskit import QuantumCircuit, QuantumRegister, ClassicalRegister
from qiskit_aer import AerSimulator


# ---------- Classical total-fresh solver (union of ranges) ----------

def parse_ranges_section(text: str):
    ranges_section = text.split("\n\n", 1)[0]
    return ranges_section


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


def total_fresh_count(text: str) -> int:
    ranges_section = parse_ranges_section(text)
    ranges = parse_ranges(ranges_section)

    if not ranges:
        return 0

    # Sort and merge like in Rust
    ranges.sort(key=lambda x: (x[0], x[1]))

    merged = []
    cur_s, cur_e = ranges[0]

    for s, e in ranges[1:]:
        if s <= cur_e + 1:
            if e > cur_e:
                cur_e = e
        else:
            merged.append((cur_s, cur_e))
            cur_s, cur_e = s, e
    merged.append((cur_s, cur_e))

    total = 0
    for s, e in merged:
        total += e - s + 1

    return total


# ---------- Toy "ranges union" puzzle circuit ----------

def build_ranges_union_puzzle_circuit() -> QuantumCircuit:
    """
    Toy circuit representing that an ID is fresh if it lies in any of several ranges.

    - One 'id' qubit
    - Three 'range' qubits (pretend: range0, range1, range2)
    - One flag qubit that marks 'fresh' if id == 1 AND (any range == 1).

    This is just structural, not encoding actual numbers.
    """
    id_q = QuantumRegister(1, "id")
    rng = QuantumRegister(3, "rng")
    flag = QuantumRegister(1, "flag")
    c = ClassicalRegister(1, "c")

    qc = QuantumCircuit(id_q, rng, flag, c, name="ranges_union_toy")

    # Toy preparation:
    # - ID is "active"
    # - Three ranges are "covering" this ID
    qc.x(id_q[0])
    qc.x(rng[0])
    qc.x(rng[1])
    qc.x(rng[2])

    # Toy rule:
    # If id == 1 and any of the ranges is 1, mark flag
    qc.ccx(id_q[0], rng[0], flag[0])
    qc.ccx(id_q[0], rng[1], flag[0])
    qc.ccx(id_q[0], rng[2], flag[0])

    qc.measure(flag[0], c[0])

    return qc


# ---------- Main ----------

def main() -> None:
    text = Path("input.txt").read_text()

    # Classical solve
    t0 = time.perf_counter()
    answer = total_fresh_count(text)
    t1 = time.perf_counter()
    classical_ms = (t1 - t0) * 1000.0

    print(answer)
    print(f"Classical time: {classical_ms:.3f} ms")

    # Build toy circuit
    puzzle_circuit = build_ranges_union_puzzle_circuit()
    puzzle_text = puzzle_circuit.draw(output="text")
    print("\nToy ranges-union puzzle circuit:")
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
    out_path = Path("day5_2_qiskit_output.txt")
    with out_path.open("w", encoding="utf-8") as f:
        f.write("AoC 2025 - Day 5 Part 2 (Qiskit)\n")
        f.write(f"Answer: {answer}\n")
        f.write(f"Classical time: {classical_ms:.3f} ms\n")
        f.write(f"Simulator time (puzzle circuit): {sim_ms:.3f} ms\n")
        f.write(f"Top measurement results: {top}\n\n")
        f.write("Toy ranges-union puzzle circuit:\n")
        f.write(str(puzzle_text))
        f.write("\n")


if __name__ == "__main__":
    main()
