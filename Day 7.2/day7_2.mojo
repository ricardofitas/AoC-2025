from time import perf_counter

fn solve(text: String) -> Int:
    # Read lines and find S
    var lines = [String("")]
    var height: Int = 0
    var width: Int = 0

    var sr: Int = -1
    var sc: Int = -1

    for slice_line in text.splitlines():
        var line = String(slice_line)
        if height == 0:
            lines[0] = line
        else:
            lines.append(line)

        var len_line = len(line)
        if len_line > width:
            width = len_line

        var c: Int = 0
        while c < len_line:
            var ch = String(line[c])
            if ch == "S":
                sr = height
                sc = c
                break
            c += 1

        height += 1

    if height == 0 or sr < 0 or sc < 0:
        return 0

    var h = height
    var w = width

    # Flattened DP array: dp[r][c] => dp[r * w + c]
    var total_cells = h * w
    var dp = [Int(0)]
    var i: Int = 1
    while i < total_cells:
        dp.append(0)
        i += 1

    fn idx(r: Int, c: Int, w: Int) -> Int:
        return r * w + c

    # Initialize: one timeline at S
    dp[idx(sr, sc, w)] = 1

    var result: Int = 0

    # Process rows from sr downwards
    var r: Int = sr
    while r < h:
        var c: Int = 0
        while c < w:
            var ways = dp[idx(r, c, w)]
            if ways != 0:
                var nr = r + 1
                if nr >= h:
                    # Particle leaves manifold
                    result += ways
                else:
                    # Character at (nr, c); treat out-of-range as space
                    var row_str = lines[nr]
                    var row_len = len(row_str)
                    var ch = " "
                    if c < row_len:
                        ch = String(row_str[c])

                    if ch == "^":
                        # Split to left/right
                        var left_c = c - 1
                        if left_c >= 0:
                            dp[idx(nr, left_c, w)] += ways
                        var right_c = c + 1
                        if right_c < w:
                            dp[idx(nr, right_c, w)] += ways
                    else:
                        # Continue downward
                        dp[idx(nr, c, w)] += ways
            c += 1
        r += 1

    return result


def main():
    var text = ""
    try:
        var f = open("input.txt", "r")
        text = f.read()
        f.close()
    except:
        print("Error: could not open input.txt")
        return

    var t0 = perf_counter()
    var answer = solve(text)
    var t1 = perf_counter()
    var elapsed_ms = (t1 - t0) * 1000.0

    print(answer)
    print("Single run time (Mojo, Day 7.2):", elapsed_ms, "ms")

    # Stress test
    var iterations: Int = 1000
    var t2 = perf_counter()
    var s: Int = 0
    for _ in range(iterations):
        s += solve(text)
    var t3 = perf_counter()

    var stress_ms = (t3 - t2) * 1000.0
    print("Dummy sum:", s)
    print(
        "Stress test (Day 7.2):",
        iterations,
        "iterations in",
        stress_ms,
        "ms",
    )
    print("Average per run:", stress_ms / Float64(iterations), "ms")
