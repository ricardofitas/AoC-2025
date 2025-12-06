from pathlib import Path
import time

from qiskit import QuantumCircuit, QuantumRegister, ClassicalRegister
from qiskit_aer import AerSimulator


# ---------- Classical solver (same logic as Rust) ----------

def build_grid(text: str):
    lines = text.splitlines()
    if not lines:
        return [], 0, 0
    width = max(len(line) for line in lines)
    height = len(lines)
    grid = []
    for line in lines:
        row = list(line)
        if len(row) < width:
            row += [" "] * (width - len(row))
        grid.append(row)
    return grid, width, height


def solve_classical(text: str) -> int:
    grid, width, height = build_grid(text)
    if height == 0:
        return 0

    # Operator row
    op_row = None
    for r in range(height - 1, -1, -1):
        if any(ch in "+*" for ch in grid[r]):
            op_row = r
            break
    if op_row is None:
        return 0

    # Column blank flags
    col_blank = [True] * width
    for c in range(width):
        for r in range(height):
            if grid[r][c] != " ":
                col_blank[c] = False
                break

    # Problem column ranges
    problems = []
    in_group = False
    start = 0
    for c in range(width):
        if not col_blank[c]:
            if not in_group:
                in_group = True
                start = c
        else:
            if in_group:
                problems.append((start, c - 1))
                in_group = False
    if in_group:
        problems.append((start, width - 1))

    total = 0

    for start_col, end_col in problems:
        # Operator in this chunk
        op = None
        for c in range(start_col, end_col + 1):
            ch = grid[op_row][c]
            if ch in "+*":
                op = ch
                break
        if op is None:
            continue

        # Numbers: one per column, top->bottom
        nums = []
        for c in range(start_col, end_col + 1):
            s = ""
            for r in range(op_row):
                ch = grid[r][c]
                if ch.isdigit():
                    s += ch
            if s:
                nums.append(int(s))

        if not nums:
            continue

        if op == "+":
            val = sum(nums)
        else:
            val = 1
            for x in nums:
                val *= x

        total += val

    return total


# ---------- Toy cephalopod vertical puzzle circuit ----------

def build_cephalopod_puzzle_circuit() -> QuantumCircuit:
    """
    Toy circuit for cephalopod-style vertical columns:

    - col_0, col_1, col_2 represent three "digit columns" as single qubits.
    - op qubit indicates which operation is chosen.
    - flag is set if "at least one column is active AND op is active".

    This is just a structural visualization, not an arithmetic circuit.
    """
    col = QuantumRegister(3, "col")
    op = QuantumRegister(1, "op")
    flag = QuantumRegister(1, "flag")
    c = ClassicalRegister(1, "c")

    qc = QuantumCircuit(col, op, flag, c, name="cephalopod_toy")

    # Prepare a toy state: all columns active, op = 1
    qc.x(col[0])
    qc.x(col[1])
    qc.x(col[2])
    qc.x(op[0])

    # Flip flag if any column AND op is active
    # Implement as: for each col_i, use a Toffoli-like pattern with op
    qc.ccx(col[0], op[0], flag[0])
    qc.ccx(col[1], op[0], flag[0])
    qc.ccx(col[2], op[0], flag[0])

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
    puzzle_circuit = build_cephalopod_puzzle_circuit()
    puzzle_text = puzzle_circuit.draw(output="text")
    print("\nToy cephalopod vertical puzzle circuit:")
    print(puzzle_text)

    # Simulate
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

    # Save to txt
    out_path = Path("day6_2_qiskit_output.txt")
    with out_path.open("w", encoding="utf-8") as f:
        f.write("AoC 2025 - Day 6 Part 2 (Qiskit)\n")
        f.write(f"Answer: {answer}\n")
        f.write(f"Classical time: {classical_ms:.3f} ms\n")
        f.write(f"Simulator time (puzzle circuit): {sim_ms:.3f} ms\n")
        f.write(f"Top measurement results: {top}\n\n")
        f.write("Toy cephalopod vertical puzzle circuit:\n")
        f.write(str(puzzle_text))
        f.write("\n")


if __name__ == "__main__":
    main()
