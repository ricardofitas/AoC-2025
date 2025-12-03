from time import perf_counter

# AoC Day 1: count how many times the dial hits 0.
def solve(text: String) -> Int:
    var pos: Int = 50
    var hits: Int = 0

    # Split the input into lines (StringSlice list)
    for line_slice in text.splitlines():
        # Trim whitespace
        var trimmed = line_slice.strip()
        if len(trimmed) == 0:
            continue

        # Convert slice to a full String so we can slice/index more easily
        var line = String(trimmed)

        # Direction is the first character as a 1-char slice
        var dir_slice = line[0:1]
        var direction = String(dir_slice)

        # Distance is everything after the first character
        var dist_slice = line[1:]
        var dist = Int(dist_slice)

        if direction == "L":
            pos = ((pos - dist) % 100 + 100) % 100
        elif direction == "R":
            pos = ((pos + dist) % 100 + 100) % 100
        else:
            # Ignore malformed lines
            continue

        if pos == 0:
            hits += 1

    return hits


def main():
    # --- Read input.txt using Mojo's builtin open() ---
    var f = open("input.txt", "r")
    var text = f.read()
    f.close()

    # --- Single run timing ---
    var t0 = perf_counter()
    var answer = solve(text)
    var t1 = perf_counter()

    var elapsed_ms = (t1 - t0) * 1000.0

    print(answer)
    print("Single run time (Mojo): ", elapsed_ms, " ms")

    # --- CPU stress: run solve() many times ---
    var iterations: Int = 10_000
    var t2 = perf_counter()
    var sum: Int = 0
    for _ in range(iterations):
        sum += solve(text)
    var t3 = perf_counter()

    var stress_ms = (t3 - t2) * 1000.0
    print("Dummy sum: ", sum)
    print(
        "Stress test: ",
        iterations,
        " iterations in ",
        stress_ms,
        " ms (avg ",
        stress_ms / Float64(iterations),
        " ms per run)"
    )
