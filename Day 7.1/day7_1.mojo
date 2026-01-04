from time import perf_counter

fn solve(text: String) -> Int:
    # Parse lines and locate S
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

        # find 'S' in this line
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

    # Flattened visited: 0 = not visited, 1 = visited
    var visited = [Int(0)]
    var total_cells = h * w
    var i: Int = 1
    while i < total_cells:
        visited.append(0)
        i += 1

    # Queue of positions (row, col)
    var q_rows = [Int(0)]
    var q_cols = [Int(0)]
    var head: Int = 0
    var tail: Int = 0

    # push (sr, sc)
    q_rows[0] = sr
    q_cols[0] = sc
    tail = 1

    fn idx(r: Int, c: Int, w: Int) -> Int:
        return r * w + c

    var splits: Int = 0

    while head < tail:
        var r = q_rows[head]
        var c = q_cols[head]
        head += 1

        var nr = r + 1
        if nr >= h:
            continue

        var nc = c
        if nc < 0 or nc >= w:
            continue

        var v_idx = idx(nr, nc, w)
        if visited[v_idx] != 0:
            continue
        visited[v_idx] = 1

        # Get character at (nr, nc); treat out-of-range as space
        var row_str = lines[nr]
        var row_len = len(row_str)
        var ch = " "
        if nc < row_len:
            ch = String(row_str[nc])

        if ch == "^":
            # Splitter hit
            splits += 1

            # Left beam
            var left_c = nc - 1
            if left_c >= 0:
                if tail == len(q_rows):
                    q_rows.append(nr)
                    q_cols.append(left_c)
                else:
                    q_rows[tail] = nr
                    q_cols[tail] = left_c
                tail += 1

            # Right beam
            var right_c = nc + 1
            if right_c < w:
                if tail == len(q_rows):
                    q_rows.append(nr)
                    q_cols.append(right_c)
                else:
                    q_rows[tail] = nr
                    q_cols[tail] = right_c
                tail += 1
        else:
            # Empty (or S, space, etc.): continue downward
            if tail == len(q_rows):
                q_rows.append(nr)
                q_cols.append(nc)
            else:
                q_rows[tail] = nr
                q_cols[tail] = nc
            tail += 1

    return splits


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
    print("Single run time (Mojo, Day 7.1):", elapsed_ms, "ms")

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
        "Stress test (Day 7.1):",
        iterations,
        "iterations in",
        stress_ms,
        "ms",
    )
    print("Average per run:", stress_ms / Float64(iterations), "ms")
