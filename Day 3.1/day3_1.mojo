from time import perf_counter


def max_joltage_for_bank(line: String) -> Int:
    """
    For one bank (line of digits), find the maximum two-digit number you can form
    by choosing positions i < j and using digits[i] as tens, digits[j] as ones.
    Implemented by reading digits from the string directly.
    """
    # Collect digits from the string into a list.
    # Use a non-empty list so Mojo knows the element type.
    var digits = [0]
    var count: Int = 0

    for ch in line:
        # ch is a 1-char slice; try to parse it as an integer digit
        var d: Int
        try:
            d = Int(String(ch))
        except:
            continue  # skip non-digit (shouldn't happen in valid input)

        if count == 0:
            digits[0] = d
        else:
            digits.append(d)
        count += 1

    if count < 2:
        return 0

    var length = count

    # suffix_max[i] = max digit from i..end
    var suffix_max = [0] * length
    suffix_max[length - 1] = digits[length - 1]

    var j = length - 2
    while j >= 0:
        if digits[j] > suffix_max[j + 1]:
            suffix_max[j] = digits[j]
        else:
            suffix_max[j] = suffix_max[j + 1]
        j -= 1

    # Scan all positions i < j and compute best tens*10 + ones
    var best: Int = 0
    var k = 0
    while k < length - 1:
        var tens = digits[k]
        var ones = suffix_max[k + 1]
        var val = tens * 10 + ones
        if val > best:
            best = val
        k += 1

    return best


def solve(text: String) -> Int:
    var total: Int = 0
    for line_slice in text.splitlines():
        var trimmed = line_slice.strip()
        if len(trimmed) == 0:
            continue
        total += max_joltage_for_bank(String(trimmed))
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
    print("Single run time (Mojo, Day 3.1):", elapsed_ms, "ms")

    # CPU stress test
    var iterations: Int = 1000
    var t2 = perf_counter()
    var s: Int = 0
    for _ in range(iterations):
        s += solve(text)
    var t3 = perf_counter()

    var stress_ms = (t3 - t2) * 1000.0
    print("Dummy sum:", s)
    print(
        "Stress test (Day 3.1):",
        iterations,
        "iterations in",
        stress_ms,
        "ms",
    )
    print("Average per run:", stress_ms / Float64(iterations), "ms")
