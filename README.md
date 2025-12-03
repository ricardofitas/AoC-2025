# Advent of Code 2025 – Multi-language Solutions (Rust, Qiskit, Mojo)

This repository contains my personal solutions for **Advent of Code 2025**, with a strong focus on:

- **Rust** for fast, systems-level solutions
- **Python + Qiskit** for quantum-computing flavored variants
- **Mojo** (via WSL) as a modern high-performance language experiment

> ⚠️ **Important:**  
> All `input.txt` files (the actual puzzle inputs from Advent of Code) are **not** committed to this repository, in order to respect Advent of Code’s rules.  
> If you want to run the solutions, you’ll need to download your own inputs from the Advent of Code website.

---

## How Advent of Code works (short version)

- Advent of Code is an annual coding event with **25 days** of puzzles.
- Each day has:
  - **Part 1** (first star ★)
  - **Part 2** (second star ★)
- You get a **personal puzzle input** when you log in; it’s usually a text file.
- The puzzle statement gives you the rules; you write code that:
  1. Reads your `input.txt`,
  2. Computes the correct answer,
  3. Prints it (usually a single number or string).

Here I’m focusing on **days 1–12** and experimenting with multiple languages per day.

---

## Repository structure

To avoid repeating the same pattern for each day, I’ll use **`X`** to represent the day number:

- `Day X.1/` – solutions for **Day X – Part 1**
- `Day X.2/` – solutions for **Day X – Part 2**

Where:

- `X` ranges from **1 to 12** (e.g., `Day 1.1`, `Day 1.2`, `Day 2.1`, …)
- Each `Day X.Y` folder may contain:
  - A Rust Cargo project (`Cargo.toml`, `src/main.rs`)
  - A Qiskit script (`dayX_Y_qiskit.py`)
  - Mojo scripts (`dayX_Y.mojo` in the WSL clone of this repo)
  - Timing summaries (e.g., `aoc_dayX_partY_times.txt`)

Current example:

- `Day 1.1/`
  - Rust solution for Day 1 – Part 1
  - Qiskit script that encodes the answer in a small quantum circuit
  - Mojo solution using the Python-compatible subset
  - Timing summary
- `Day 1.2/`
  - Rust solution for Day 1 – Part 2 (including a CPU stress test using Rayon)
  - Qiskit script that:
    - Solves the puzzle classically
    - Builds a “dial circuit” representing the rotations
    - Encodes the final answer in a quantum circuit
  - Mojo solution for Part 2
  - Timing summary

As I add more days, the same pattern will hold; only `X` changes (2, 3, …, 12).

---

## How to use this repository

### 1. Clone the repo

```bash
git clone https://github.com/ricardofitas/AoC-2025.git
cd AoC-2025
```

On Windows, you can also clone via GitHub Desktop and open the folder in VS Code.

---

### 2. Get your Advent of Code inputs

1. Go to [https://adventofcode.com](https://adventofcode.com).
2. Log in with your account.
3. Navigate to the puzzle for **Day X**.
4. Click on **“Get your puzzle input”**, copy the content, and save it as:

   - `Day X.1/input.txt` for Part 1
   - `Day X.2/input.txt` for Part 2

> These input files are **ignored** by Git and are never pushed to the repository.

---

### 3. Running the Rust solutions

Each `Day X.Y/` is a standalone Cargo project.

**Example for Day 1 – Part 1 (Day 1.1):**

```powershell
cd "Day 1.1"
cargo run
```

This will:

- Compile the Rust program
- Read `input.txt`
- Print the puzzle answer to stdout
- Also print timing information (in milliseconds) to stderr

For optimized timings with CPU-specific optimizations:

```powershell
$env:RUSTFLAGS="-C target-cpu=native"
cargo run --release
```

**Example for Day 1 – Part 2 (Day 1.2):**

```powershell
cd "Day 1.2"
cargo run --release
```

You’ll see:

- The puzzle answer
- Single-run timing
- A parallel CPU stress test using Rayon (e.g., 10,000 tasks in parallel)

---

### 4. Running the Qiskit solutions (Python)

Each part has a corresponding Qiskit script, typically named:

- `dayX_Y_qiskit.py`

These scripts:

- Solve the puzzle **classically** (normal Python logic)
- Build a **quantum circuit** that either:
  - Encodes the final answer in qubits, or
  - Represents the dial/rotation process as a sequence of unitary operations
- Run the circuit on a **simulator** (Qiskit Aer)
- Print:
  - Classical solve time
  - Simulator time
  - Top measurement results
- Save details (including ASCII circuit diagrams) to a `.txt` file

**Requirements:**

```bash
python -m venv .venv
.\.venv\Scripts ctivate  # Windows
# or source .venv/bin/activate on Linux/WSL

pip install qiskit qiskit-aer
```

**Example usage (Day 1 – Part 2):**

```powershell
cd "Day 1.2"
python day1_2_qiskit.py
```

You should see the answer, timings, and ASCII circuit. A file like
`day1_2_qiskit_output.txt` will be created with the full log and circuits.

---

### 5. Running the Mojo solutions (WSL)

Mojo currently doesn’t run natively on Windows, so I use **WSL (Ubuntu)**:

1. Install WSL and Ubuntu:

   ```powershell
   wsl --install -d Ubuntu-22.04
   ```

2. Inside Ubuntu, install Mojo following the official docs:

   - [https://docs.modular.com/mojo/](https://docs.modular.com/mojo/)

3. Clone this repo inside WSL (or access the Windows path via `/mnt/c/...`).

4. In the WSL copy, there will be subfolders like:

   - `~/aoc-2025/day1/`
   - `~/aoc-2025/day1_2/`

   containing `day1.mojo`, `day1_2.mojo`, etc.

**Example (Day 1 – Part 2 in Mojo, from WSL):**

```bash
cd ~/aoc-2025/day1_2
source ../day1/.venv/bin/activate  # if you reuse the same venv
mojo run day1_2.mojo
```

You will get:

- The AoC answer
- Single-run timing
- A CPU stress test (many iterations of the solver)

The Mojo files currently use the “Python-compatible” subset of Mojo syntax to
stay stable against language changes.

---

## About the languages used

### Rust

- **What it is:** A modern, memory-safe systems programming language.
- **Why I use it:** 
  - Blazing fast for AoC puzzles.
  - Great tooling (`cargo`), strong type system, fearless concurrency.
- **Official site:**  
  <https://www.rust-lang.org/>

---

### Qiskit (Python)

- **What it is:** An open-source SDK for working with **quantum computers** at
  the circuit and algorithm level.
- **Why I use it:**
  - To build **quantum circuit versions** of some AoC problems.
  - To show quantum-state modeling, unitaries, and measurements, even if
    there is no speedup for these puzzles.
- **Official site:**  
  <https://qiskit.org/>

---

### Mojo

- **What it is:** A new programming language by Modular, designed to combine:
  - Python-like syntax,
  - C/C++-level performance,
  - Strong focus on AI and numerical workloads.
- **Why I use it:**
  - As an experimental high-performance language for AoC problems.
  - To explore how Mojo’s evolving toolchain feels on real puzzle workloads.
- **Official docs:**  
  <https://docs.modular.com/mojo/>

> Note: Mojo is still evolving rapidly. Many of the Mojo solutions in this repo
> use the Python-compatible subset for stability, and are run via WSL on Ubuntu.

---

## Contact

If you’re curious, spot a bug, or want to discuss research/AI/quantum topics:

- **Name:** Ricardo Fitas
- **GitHub:** [@ricardofitas](https://github.com/ricardofitas)
- **Email:** rfitas99@gmail.com

Feel free to open issues or discussions on this repo – especially if you’re
doing AoC 2025 with “unusual” languages too (Rust+Mojo+Qiskit, Roc, etc.).
