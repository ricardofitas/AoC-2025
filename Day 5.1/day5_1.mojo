from time import perf_counter


def solve(text: String) -> Int:
    # We'll store ranges in two parallel lists: starts and ends
    var starts = [0]
    var ends = [0]
    var range_count: Int = 0

    var counting_ranges = True
    var fresh_count: Int = 0

    for line_slice in text.splitlines():
        var trimmed = line_slice.strip()
        if len(trimmed) == 0:
            # Blank line: after this, lines are IDs
            counting_ranges = False
            continue

        var line = String(trimmed)

        if counting_ranges:
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
        else:
            # IDs section: each line is a single integer ID
            var id_val = Int(line)

            var is_fresh = False
            var r: Int = 0
            while r < range_count:
                var s = starts[r]
                var e = ends[r]
                if id_val >= s and id_val <= e:
                    is_fresh = True
                    break
                r += 1

            if is_fresh:
                fresh_count += 1

    return fresh_count


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
    print("Single run time (Mojo, Day 5.1):", elapsed_ms, "ms")

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
        "Stress test (Day 5.1):",
        iterations,
        "iterations in",
        stress_ms,
        "ms",
    )
    print("Average per run:", stress_ms / Float64(iterations), "ms")
