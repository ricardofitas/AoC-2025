from time import perf_counter


def max_joltage_12(line: String) -> Int:
    """
    Choose exactly 12 digits (as a subsequence) so that the resulting 12-digit
    number is as large as possible.
    """
    # Collect digits from the string into a list.
    var digits = [0]
    var count: Int = 0

    for ch in line:
        var d: Int
        try:
            d = Int(String(ch))
        except:
            continue  # skip non-digits

        if count == 0:
            digits[0] = d
        else:
            digits.append(d)
        count += 1

    if count < 12:
        return 0

    var length = count
    var K: Int = 12

    # Greedy subsequence selection
    var result = [0] * K
    var start_idx: Int = 0
    var pos: Int = 0
    while pos < K:
        var remaining_slots = K - pos
        var end_idx = length - remaining_slots  # inclusive

        var best_digit: Int = 0
        var best_pos: Int = start_idx

        var i = start_idx
        while i <= end_idx:
            var d = digits[i]
            if d > best_digit:
                best_digit = d
                best_pos = i
                if best_digit == 9:
                    break
            i += 1

        result[pos] = best_digit
        start_idx = best_pos + 1
        pos += 1

    # Convert 12 digits in result[] to Int
    var value: Int = 0
    var idx: Int = 0
    while idx < K:
        value = value * 10 + result[idx]
        idx += 1

    return value


def solve(text: String) -> Int:
    var total: Int = 0
    for line_slice in text.splitlines():
        var trimmed = line_slice.strip()
        if len(trimmed) == 0:
            continue
        total += max_joltage_12(String(trimmed))
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
    print("Single run time (Mojo, Day 3.2):", elapsed_ms, "ms")

    # CPU stress test (heavy, so keep iterations low)
    var iterations: Int = 200
    var t2 = perf_counter()
    var s: Int = 0
    for _ in range(iterations):
        s += solve(text)
    var t3 = perf_counter()

    var stress_ms = (t3 - t2) * 1000.0
    print("Dummy sum:", s)
    print(
        "Stress test (Day 3.2):",
        iterations,
        "iterations in",
        stress_ms,
        "ms",
    )
    print("Average per run:", stress_ms / Float64(iterations), "ms")
