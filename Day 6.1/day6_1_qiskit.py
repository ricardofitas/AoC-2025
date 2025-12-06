from pathlib import Path
import time

from qiskit import QuantumCircuit, QuantumRegister, ClassicalRegister
from qiskit_aer import AerSimulator


# ---------- Classical solver ----------

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

    # Find operator row (bottom-most with + or *)
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
        # Operator
        op = None
        for c in range(start_col, end_col + 1):
            ch = grid[op_row][c]
            if ch in "+*":
                op = ch
                break
        if op is None:
            continue

        # Numbers above
        nums = []
        for r in range(op_row):
            s = "".join(
                grid[r][c]
                for c in range(start_col, end_col + 1)
                if grid[r][c].isdigit()
            )
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


# ---------- Toy vertical-problem circuit ----------

def build_vertical_problem_puzzle_circuit() -> QuantumCircuit:
    """
    Toy circuit for: 3 numbers combined by + or *.

    It does NOT compute real arithmetic; instead:
      - Three 'num' qubits representing presence of numbers
      - One 'op' qubit (0 = +, 1 = *)
      - One 'flag' that lights up if "any numbers present"
    """
    num = QuantumRegister(3, "num")
    op = QuantumRegister(1, "op")
    flag = QuantumRegister(1, "flag")
    c = ClassicalRegister(1, "c")

    qc = QuantumCircuit(num, op, flag, c, name="vertical_problem_toy")

    # Toy prep: all three numbers "present", op qubit set (say '*')
    qc.x(num[0])
    qc.x(num[1])
    qc.x(num[2])
    qc.x(op[0])

    # Mark flag if any number qubit is 1 (OR-like structure)
    qc.cx(num[0], flag[0])
    qc.cx(num[1], flag[0])
    qc.cx(num[2], flag[0])

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

    # Build puzzle circuit
    puzzle_circuit = build_vertical_problem_puzzle_circuit()
    puzzle_text = puzzle_circuit.draw(output="text")
    print("\nToy vertical-problem puzzle circuit:")
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
    out_path = Path("day6_1_qiskit_output.txt")
    with out_path.open("w", encoding="utf-8") as f:
        f.write("AoC 2025 - Day 6 Part 1 (Qiskit)\n")
        f.write(f"Answer: {answer}\n")
        f.write(f"Classical time: {classical_ms:.3f} ms\n")
        f.write(f"Simulator time (puzzle circuit): {sim_ms:.3f} ms\n")
        f.write(f"Top measurement results: {top}\n\n")
        f.write("Toy vertical-problem puzzle circuit:\n")
        f.write(str(puzzle_text))
        f.write("\n")


if __name__ == "__main__":
    main()
