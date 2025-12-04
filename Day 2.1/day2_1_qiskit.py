from pathlib import Path
import time
import numpy as np

from qiskit import QuantumCircuit, QuantumRegister, ClassicalRegister
from qiskit.quantum_info import Operator
from qiskit_aer import AerSimulator


# ---------- Classical solver ----------

def is_invalid_id(n: int) -> bool:
    s = str(n)
    if len(s) % 2 == 1:
        return False
    half = len(s) // 2
    return s[:half] == s[half:]


def solve_classical(text: str) -> int:
    line = text.strip()
    total = 0

    for token in line.split(","):
        token = token.strip()
        if not token:
            continue

        start_s, end_s = token.split("-")
        start = int(start_s)
        end = int(end_s)

        for n in range(start, end + 1):
            if is_invalid_id(n):
                total += n

    return total


# ---------- Puzzle circuit: "AA" pattern check ----------

def build_equal_halves_circuit(num_pairs: int) -> QuantumCircuit:
    """
    Build a circuit that:
    - Has 2*num_pairs data qubits (representing bits of AA).
    - Has num_pairs ancilla qubits to store pairwise mismatches.
    - Has 1 final 'flag' qubit that is flipped if halves are equal.
    This is a generic circuit that encodes the equality-check structure.
    """
    data = QuantumRegister(2 * num_pairs, "x")      # first and second half
    anc = QuantumRegister(num_pairs, "cmp")         # compare bits
    flag = QuantumRegister(1, "flag")               # equals flag
    c = ClassicalRegister(2 * num_pairs + 1, "c")   # for measurement
    qc = QuantumCircuit(data, anc, flag, c, name="equal_halves")

    # --- 1. Prepare an example invalid ID of the form AA ---
    #
    # We'll just choose some random bitstring for A; here we pick 101... pattern.
    for i in range(num_pairs):
        if i % 2 == 0:
            qc.x(data[i])         # set bit in first half
            qc.x(data[num_pairs + i])  # mirror into second half

    # --- 2. Compare first half and second half into ancillas ---
    #
    # For each pair (x_i, x_{i+n}), we compute XOR into anc[i]:
    # anc[i] = x_i XOR x_{i+n}.
    for i in range(num_pairs):
        qc.cx(data[i], anc[i])
        qc.cx(data[num_pairs + i], anc[i])


    # --- 3. Use multi-controlled phase (π) to set 'flag' if all anc[i] == 0 ---
    #
    # Your Qiskit version has `mcp` instead of `mct`. We can use a multi-controlled
    # phase gate with angle π, wrapped with H gates on the target, to emulate MCX.
    for i in range(num_pairs):
        qc.x(anc[i])

    qc.h(flag[0])
    qc.mcp(np.pi, anc, flag[0])  # multi-controlled phase with angle π
    qc.h(flag[0])

    for i in range(num_pairs):
        qc.x(anc[i])


    # --- 4. Measure all data bits and the flag ---
    for i in range(2 * num_pairs):
        qc.measure(data[i], c[i])

    qc.measure(flag[0], c[2 * num_pairs])

    return qc


# ---------- Small unitary example (optional visualization) ----------

def build_pair_swap_unitary(num_pairs: int) -> Operator:
    """
    For completeness: build a unitary that swaps first and second halves of the
    2*num_pairs data bits. This is another way of encoding the 'AA' structure,
    but we only use it if we want to attach a big unitary block to the circuit.
    """
    dim = 2 ** (2 * num_pairs)
    mat = np.zeros((dim, dim), dtype=complex)

    for basis in range(dim):
        # decode bits
        bits = [(basis >> i) & 1 for i in range(2 * num_pairs)]
        # swap halves
        new_bits = bits[num_pairs:] + bits[:num_pairs]
        new_index = 0
        for i, b in enumerate(new_bits):
            new_index |= (b << i)
        mat[new_index, basis] = 1.0

    return Operator(mat)


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

    # Build a puzzle circuit for, say, 3 bit-pairs (6 bits total => length-6 ID)
    num_pairs = 3
    eq_circuit = build_equal_halves_circuit(num_pairs)
    eq_circuit_text = eq_circuit.draw(output="text")
    print("\nEqual-halves puzzle circuit (structure for AA pattern):")
    print(eq_circuit_text)

    # Optional: run on simulator just to show it's valid
    sim = AerSimulator()
    t2 = time.perf_counter()
    job = sim.run(eq_circuit, shots=1024)
    result = job.result()
    t3 = time.perf_counter()
    sim_ms = (t3 - t2) * 1000.0

    counts = result.get_counts()
    top = sorted(counts.items(), key=lambda kv: -kv[1])[:5]
    print("Top measurement results (puzzle circuit):", top)
    print(f"Simulator time: {sim_ms:.3f} ms")

    # Dump everything to a txt file
    out_path = Path("day2_1_qiskit_output.txt")
    with out_path.open("w", encoding="utf-8") as f:
        f.write("AoC 2025 - Day 2 Part 1 (Qiskit)\n")
        f.write(f"Answer: {answer}\n")
        f.write(f"Classical time: {classical_ms:.3f} ms\n")
        f.write(f"Simulator time (puzzle circuit): {sim_ms:.3f} ms\n")
        f.write(f"Top measurement results: {top}\n\n")
        f.write("Equal-halves puzzle circuit:\n")
        f.write(str(eq_circuit_text))
        f.write("\n")


if __name__ == "__main__":
    main()
