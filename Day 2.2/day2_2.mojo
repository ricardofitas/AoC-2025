from time import perf_counter


def pow10(exp: Int) -> Int:
    var result: Int = 1
    var i: Int = 0
    while i < exp:
        result *= 10
        i += 1
    return result


def get_block(n: Int, total_digits: Int, block_len: Int, index: Int) -> Int:
    """
    Extract the decimal block number `index` (0-based from left) of length
    `block_len` from n, assuming n has exactly total_digits digits.

    Example:
      n = 12341234, total_digits = 8, block_len = 4
      index 0 -> 1234
      index 1 -> 1234
    """
    var shift = total_digits - (index + 1) * block_len
    if shift < 0:
        return 0

    var div = pow10(shift)
    var mod_base = pow10(block_len)
    var tmp = n // div
    return tmp % mod_base


def is_invalid_id(n: Int) -> Bool:
    """
    Part 2 invalid ID:
    A decimal number whose digits are some sequence repeated at least twice.
    Examples:
      12341234      -> "1234" x 2
      123123123     -> "123"  x 3
      1212121212    -> "12"   x 5
      1111111       -> "1"    x 7
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

    if digits < 2:
        return False

    # Try all repeat counts k >= 2 such that digits = k * block_len
    var k: Int = 2
    while k <= digits:
        if digits % k != 0:
            k += 1
            continue

        var block_len = digits // k

        # Extract first block
        var first_block = get_block(n, digits, block_len, 0)

        # Compare remaining blocks
        var ok = True
        var i: Int = 1
        while i < k:
            var b = get_block(n, digits, block_len, i)
            if b != first_block:
                ok = False
                break
            i += 1

        if ok:
            return True

        k += 1

    return False


def solve(text: String) -> Int:
    # First non-empty line is the long list of ranges
    var line = ""
    for line_slice in text.splitlines():
        var trimmed = line_slice.strip()
        if len(trimmed) != 0:
            line = String(trimmed)
            break

    if line == "":
        return 0

    var total: Int = 0

    # Split by commas -> "start-end"
    for token_slice in line.split(","):
        var token_trim = token_slice.strip()
        if len(token_trim) == 0:
            continue

        var token = String(token_trim)
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
    print("Single run time (Mojo, Day 2.2):", elapsed_ms, "ms")

    # CPU stress test (heavier puzzle, keep iterations modest)
    var iterations: Int = 50
    var t2 = perf_counter()
    var s: Int = 0
    for _ in range(iterations):
        s += solve(text)
    var t3 = perf_counter()

    var stress_ms = (t3 - t2) * 1000.0
    print("Dummy sum:", s)
    print(
        "Stress test (Day 2.2):",
        iterations,
        "iterations in",
        stress_ms,
        "ms",
    )
    print("Average per run:", stress_ms / Float64(iterations), "ms")
