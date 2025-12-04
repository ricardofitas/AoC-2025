from pathlib import Path
import time
import numpy as np

from qiskit import QuantumCircuit, QuantumRegister, ClassicalRegister
from qiskit_aer import AerSimulator


# ---------- Classical solver ----------

def is_invalid_id(n: int) -> bool:
    s = str(n)
    length = len(s)
    if length < 2:
        return False

    # Try all repeat counts k >= 2 such that length = k * block_len
    for k in range(2, length + 1):
        if length % k != 0:
            continue
        block_len = length // k
        block = s[:block_len]
        ok = True
        i = block_len
        while i < length:
            if s[i:i + block_len] != block:
                ok = False
                break
            i += block_len
        if ok:
            return True
    return False


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


# ---------- Puzzle circuit: repeated pattern check ----------

def build_repeat_pattern_circuit() -> QuantumCircuit:
    """
    Build a small circuit that encodes the idea:
    "some sequence of bits repeated at least twice".

    We use 3 copies of a 2-bit pattern A:
      A A A  => 6 data qubits

    - data[0:2]  = first copy of A
    - data[2:4]  = second copy of A
    - data[4:6]  = third copy of A

    We compare:
      - block0 vs block1
      - block0 vs block2

    If all comparisons match, we flip a flag qubit.
    """
    num_blocks = 3
    block_len = 2
    num_data = num_blocks * block_len

    data = QuantumRegister(num_data, "x")
    cmp = QuantumRegister(4, "cmp")   # 2 comparisons per pair
    flag = QuantumRegister(1, "flag")
    c = ClassicalRegister(num_data + 1, "c")

    qc = QuantumCircuit(data, cmp, flag, c, name="repeat_pattern")

    # 1. Prepare an example repeated pattern A A A, with A = '10'
    #    So bits: [1,0, 1,0, 1,0]
    for block in range(num_blocks):
        base = block * block_len
        qc.x(data[base])  # set first bit of each block to 1
        # second bit remains 0

    # 2. Compare block 0 vs block 1 into cmp[0], cmp[1]
    qc.cx(data[0], cmp[0])
    qc.cx(data[2], cmp[0])
    qc.cx(data[1], cmp[1])
    qc.cx(data[3], cmp[1])

    #    Compare block 0 vs block 2 into cmp[2], cmp[3]
    qc.cx(data[0], cmp[2])
    qc.cx(data[4], cmp[2])
    qc.cx(data[1], cmp[3])
    qc.cx(data[5], cmp[3])

    # 3. Flip 'flag' if all cmp[*] == 0 (all comparisons match)
    for i in range(4):
        qc.x(cmp[i])

    # emulate multi-controlled X using mcp(pi) + H
    qc.h(flag[0])
    qc.mcp(np.pi, cmp, flag[0])
    qc.h(flag[0])

    for i in range(4):
        qc.x(cmp[i])

    # 4. Measure data and flag
    for i in range(num_data):
        qc.measure(data[i], c[i])

    qc.measure(flag[0], c[num_data])

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

    # Build the repeat-pattern puzzle circuit
    repeat_circuit = build_repeat_pattern_circuit()
    repeat_text = repeat_circuit.draw(output="text")
    print("\nRepeat-pattern puzzle circuit (structure for A repeated ≥ 2 times):")
    print(repeat_text)

    # Run on simulator (just to show it's valid)
    sim = AerSimulator()
    t2 = time.perf_counter()
    job = sim.run(repeat_circuit, shots=1024)
    result = job.result()
    t3 = time.perf_counter()
    sim_ms = (t3 - t2) * 1000.0

    counts = result.get_counts()
    top = sorted(counts.items(), key=lambda kv: -kv[1])[:5]
    print("Top measurement results (puzzle circuit):", top)
    print(f"Simulator time: {sim_ms:.3f} ms")

    # Dump everything to a txt file
    out_path = Path("day2_2_qiskit_output.txt")
    with out_path.open("w", encoding="utf-8") as f:
        f.write("AoC 2025 - Day 2 Part 2 (Qiskit)\n")
        f.write(f"Answer: {answer}\n")
        f.write(f"Classical time: {classical_ms:.3f} ms\n")
        f.write(f"Simulator time (puzzle circuit): {sim_ms:.3f} ms\n")
        f.write(f"Top measurement results: {top}\n\n")
        f.write("Repeat-pattern puzzle circuit:\n")
        f.write(str(repeat_text))
        f.write("\n")


if __name__ == "__main__":
    main()
