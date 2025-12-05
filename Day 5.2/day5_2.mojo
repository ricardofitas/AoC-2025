from time import perf_counter


def solve(text: String) -> Int:
    # We'll store ranges in two parallel lists: starts and ends.
    var starts = [0]
    var ends = [0]
    var range_count: Int = 0

    var counting_ranges = True

    for line_slice in text.splitlines():
        var trimmed = line_slice.strip()
        if len(trimmed) == 0:
            # Blank line: after this, we'd have available IDs in Part 1,
            # but in Part 2 we ignore that section entirely.
            counting_ranges = False
            continue

        if not counting_ranges:
            # Ignore second section completely.
            continue

        var line = String(trimmed)

        # Parse "a-b"
        var dash_index: Int = -1
        var i: Int = 0
        var n = len(line)
        while i < n:
            if line[i] == "-":
                dash_index = i
                break
            i += 1

        if dash_index < 0:
            continue

        var start_slice = line[0:dash_index]
        var end_slice = line[dash_index + 1:n]

        var a = Int(start_slice)
        var b = Int(end_slice)
        var lo: Int = a
        var hi: Int = b
        if b < a:
            lo = b
            hi = a

        if range_count == 0:
            starts[0] = lo
            ends[0] = hi
        else:
            starts.append(lo)
            ends.append(hi)
        range_count += 1

    if range_count == 0:
        return 0

    # --- Sort ranges by (start, end) using simple selection sort ---
    var idx: Int = 0
    while idx < range_count:
        var min_idx = idx
        var j: Int = idx + 1
        while j < range_count:
            var s_j = starts[j]
            var e_j = ends[j]
            var s_m = starts[min_idx]
            var e_m = ends[min_idx]

            if s_j < s_m or (s_j == s_m and e_j < e_m):
                min_idx = j

            j += 1

        if min_idx != idx:
            var tmp = starts[idx]
            starts[idx] = starts[min_idx]
            starts[min_idx] = tmp

            tmp = ends[idx]
            ends[idx] = ends[min_idx]
            ends[min_idx] = tmp

        idx += 1

    # --- Merge overlapping/touching ranges and sum their lengths ---
    var total: Int = 0

    var cur_s = starts[0]
    var cur_e = ends[0]

    var k: Int = 1
    while k < range_count:
        var s = starts[k]
        var e = ends[k]

        if s <= cur_e + 1:
            # Overlap or touch
            if e > cur_e:
                cur_e = e
        else:
            # Disjoint: close the current interval
            total += cur_e - cur_s + 1
            cur_s = s
            cur_e = e

        k += 1

    # Add final merged interval
    total += cur_e - cur_s + 1

    return total


def main():
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
    print("Single run time (Mojo, Day 5.2):", elapsed_ms, "ms")

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
        "Stress test (Day 5.2):",
        iterations,
        "iterations in",
        stress_ms,
        "ms",
    )
    print("Average per run:", stress_ms / Float64(iterations), "ms")
