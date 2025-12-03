from pathlib import Path
from typing import Dict, Tuple

import numpy as np

from qiskit import QuantumCircuit
from qiskit.quantum_info import Statevector, Operator


NUM_STATES = 100      # dial values 0..99
NUM_QUBITS = 7        # 2^7 = 128 >= 100


def parse_rotations(text: str):
    rotations = []
    for raw_line in text.splitlines():
        line = raw_line.strip()
        if not line:
            continue
        direction = line[0]
        dist = int(line[1:])
        rotations.append((direction, dist % NUM_STATES))
    return rotations


def make_rotation_unitary(direction: str, dist: int) -> Operator:
    """
    Build a 2^7 x 2^7 permutation unitary representing a single rotation:
      - For states 0..99: add/subtract dist modulo 100
      - For states 100..127: leave unchanged
    """
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
            # unused states: act as identity
            new_basis = basis

        mat[new_basis, basis] = 1.0

    return Operator(mat)


def build_quantum_dial_circuit(rotations) -> Tuple[QuantumCircuit, Dict[Tuple[str, int], Operator]]:
    """
    Build a QuantumCircuit with 7 qubits where each rotation is a big unitary gate.
    Also return a cache of Operators so we can reuse them with Statevector.
    """
    qc = QuantumCircuit(NUM_QUBITS, name="dial")
    cache: Dict[Tuple[str, int], Operator] = {}

    for direction, dist in rotations:
        key = (direction, dist)
        if key not in cache:
            cache[key] = make_rotation_unitary(direction, dist)
        op = cache[key]
        qc.unitary(op, qc.qubits, label=f"{direction}{dist}")

    return qc, cache


def solve_quantum(text: str) -> int:
    """
    Use quantum state evolution to count how many times the dial is at 0.
    """
    rotations = parse_rotations(text)
    qc, op_cache = build_quantum_dial_circuit(rotations)

    # Start in |50> (dial initially at 50)
    state = Statevector.from_int(50, 2**NUM_QUBITS)
    hits = 0

    # Evolve step by step
    for direction, dist in rotations:
        op = op_cache[(direction, dist)]
        state = state.evolve(op)

        # Because our unitaries are permutations, the state is always a basis state.
        # Find which basis vector has probability 1:
        probs = np.abs(state.data) ** 2
        idx = int(np.argmax(probs))

        if idx == 0:
            hits += 1

    return hits, qc


def main():
    text = Path("input.txt").read_text()

    hits, qc = solve_quantum(text)

    print(f"Quantum-simulated hits at 0: {hits}")

    # Show the circuit (lots of big unitary boxes)
    print("\nQuantum dial circuit:")
    print(qc.draw(output="text"))

    # Optional: save circuit to text file
    out_path = Path("day1_qiskit_quantum_solver_output.txt")
    with out_path.open("w", encoding="utf-8") as f:
        f.write(f"Quantum hits at 0: {hits}\n\n")
        f.write("Circuit:\n")
        f.write(str(qc.draw(output='text')))
        f.write("\n")


if __name__ == "__main__":
    main()
