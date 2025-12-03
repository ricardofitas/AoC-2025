from pathlib import Path
import time
import numpy as np

from qiskit import QuantumCircuit
from qiskit.quantum_info import Operator
from qiskit_aer import AerSimulator


NUM_STATES = 100
NUM_QUBITS = 7  # 2^7 = 128 >= 100


def zeros_on_move(pos: int, direction: str, dist: int) -> tuple[int, int]:
    if dist <= 0:
        return pos, 0

    s = pos % 100

    if direction == "R":
        delta = (100 - s) % 100
        if delta == 0:
            delta = 100
        hits = 0
        if dist >= delta:
            hits = 1 + (dist - delta) // 100
        new_pos = (pos + dist) % 100
    elif direction == "L":
        delta = s % 100
        if delta == 0:
            delta = 100
        hits = 0
        if dist >= delta:
            hits = 1 + (dist - delta) // 100
        new_pos = (pos - dist) % 100
    else:
        raise ValueError(f"Invalid direction: {direction!r}")

    return new_pos, hits


def solve_classical(text: str) -> int:
    pos = 50
    hits = 0

    for raw_line in text.splitlines():
        line = raw_line.strip()
        if not line:
            continue

        direction = line[0]
        dist = int(line[1:])
        pos, h = zeros_on_move(pos, direction, dist)
        hits += h

    return hits


def parse_rotations(text: str):
    rots = []
    for raw_line in text.splitlines():
        line = raw_line.strip()
        if not line:
            continue
        direction = line[0]
        dist = int(line[1:]) % NUM_STATES
        rots.append((direction, dist))
    return rots


def make_rotation_unitary(direction: str, dist: int) -> Operator:
    dim = 2**NUM_QUBITS
    mat = np.zeros((dim, dim), dtype=complex)

    for basis in range(dim):
        if basis < NUM_STATES:
            if direction == "R":
                new_basis = (basis + dist) % NUM_STATES
            elif direction == "L":
                new_basis = (basis - dist) % NUM_STATES
            else:
                raise ValueError(f"Invalid direction {direction!r}")
        else:
            new_basis = basis

        mat[new_basis, basis] = 1.0

    return Operator(mat)


def build_dial_circuit(rotations):
    qc = QuantumCircuit(NUM_QUBITS, name="dial")
    cache: dict[tuple[str, int], Operator] = {}

    for direction, dist in rotations:
        key = (direction, dist)
        if key not in cache:
            cache[key] = make_rotation_unitary(direction, dist)
        op = cache[key]
        qc.unitary(op, qc.qubits, label=f"{direction}{dist}")

    return qc


def build_answer_circuit(value: int) -> QuantumCircuit:
    num_qubits = max(1, value.bit_length())
    qc = QuantumCircuit(num_qubits, num_qubits, name="answer")

    for i in range(num_qubits):
        if (value >> i) & 1:
            qc.x(i)

    qc.measure_all()
    return qc


def main() -> None:
    text = Path("input.txt").read_text()

    # Classical solve
    t0 = time.perf_counter()
    answer = solve_classical(text)
    t1 = time.perf_counter()
    classical_ms = (t1 - t0) * 1000.0

    print(answer)
    print(f"Classical time: {classical_ms:.3f} ms")

    # Build circuits
    rotations = parse_rotations(text)
    dial_circuit = build_dial_circuit(rotations)
    answer_circuit = build_answer_circuit(answer)

    dial_text = dial_circuit.draw(output="text")
    answer_text = answer_circuit.draw(output="text")

    print("\nDial circuit (process):")
    print(dial_text)
    print("\nAnswer circuit (binary encoding):")
    print(answer_text)

    # Simulator just on the answer circuit (like before)
    sim = AerSimulator()
    t2 = time.perf_counter()
    job = sim.run(answer_circuit, shots=1024)
    result = job.result()
    t3 = time.perf_counter()
    sim_ms = (t3 - t2) * 1000.0

    counts = result.get_counts()
    top = sorted(counts.items(), key=lambda kv: -kv[1])[:5]
    print("Top measurement results:", top)
    print(f"Simulator time: {sim_ms:.3f} ms")

    # Dump everything to txt
    out_path = Path("day1_2_qiskit_output.txt")
    with out_path.open("w", encoding="utf-8") as f:
        f.write("AoC 2025 - Day 1 Part 2 (Qiskit)\n")
        f.write(f"Answer: {answer}\n")
        f.write(f"Classical time: {classical_ms:.3f} ms\n")
        f.write(f"Simulator time: {sim_ms:.3f} ms\n")
        f.write(f"Top measurement results: {top}\n\n")
        f.write("Dial circuit (process):\n")
        f.write(str(dial_text))
        f.write("\n\nAnswer circuit (binary encoding):\n")
        f.write(str(answer_text))
        f.write("\n")


if __name__ == "__main__":
    main()

