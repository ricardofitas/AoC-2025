from time import perf_counter


def is_invalid_id(n: Int) -> Bool:
    """
    Invalid if the decimal digits of n are of even length and of the form AA,
    i.e., first half == second half.
    Implemented purely numerically (no strings).
    """
    if n <= 0:
        return False

    # Count digits
    var tmp = n
    var digits: Int = 0
    while tmp > 0:
        digits += 1
        tmp = tmp // 10

    # Must have even number of digits
    if digits % 2 == 1:
        return False

    var half = digits // 2

    # Compute 10^half
    var pow10_half: Int = 1
    for _ in range(half):
        pow10_half *= 10

    var first = n // pow10_half
    var second = n % pow10_half

    return first == second


def solve(text: String) -> Int:
    # Get first non-empty line (the whole input is a single long line)
    var line = ""
    for line_slice in text.splitlines():
        var trimmed = line_slice.strip()
        if len(trimmed) != 0:
            line = String(trimmed)
            break

    if line == "":
        return 0

    var total: Int = 0

    # Split by commas -> ranges "start-end"
    for token_slice in line.split(","):
        var token_trim = token_slice.strip()
        if len(token_trim) == 0:
            continue

        var token = String(token_trim)

        # Split "a-b" into parts
        var parts = token.split("-")
        if len(parts) != 2:
            continue

        var start_s = String(parts[0])
        var end_s = String(parts[1])

        var start: Int
        var end: Int

        try:
            start = Int(start_s)
            end = Int(end_s)
        except:
            continue

        var id_val = start
        while id_val <= end:
            if is_invalid_id(id_val):
                total += id_val
            id_val += 1

    return total


def main():
    # Read input.txt
    var text = ""
    try:
        var f = open("input.txt", "r")
        text = f.read()
        f.close()
    except:
        print("Error: could not open input.txt")
        return

    # Single run timing
    var t0 = perf_counter()
    var answer = solve(text)
    var t1 = perf_counter()
    var elapsed_ms = (t1 - t0) * 1000.0

    print(answer)
    print("Single run time (Mojo, Day 2.1):", elapsed_ms, "ms")

    # CPU stress test (fewer iterations; Day 2 is heavier)
    var iterations: Int = 100
    var t2 = perf_counter()
    var s: Int = 0
    for _ in range(iterations):
        s += solve(text)
    var t3 = perf_counter()

    var stress_ms = (t3 - t2) * 1000.0
    print("Dummy sum:", s)
    print(
        "Stress test (Day 2.1):",
        iterations,
        "iterations in",
        stress_ms,
        "ms",
    )
    print("Average per run:", stress_ms / Float64(iterations), "ms")
