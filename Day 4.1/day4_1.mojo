from time import perf_counter


def solve(text: String) -> Int:
    # Build grid as list of rows, each row is list[Int] with 1 for '@' and 0 for '.'
    var grid = [[0]]  # non-empty so Mojo knows it's List[List[Int]]
    var row_count: Int = 0
    var width: Int = 0

    for line_slice in text.splitlines():
        var trimmed = line_slice.strip()
        if len(trimmed) == 0:
            continue

        var line = String(trimmed)

        # Build row as a list of 0/1 values
        var row = [0]
        var count: Int = 0

        for ch in line:
            var s = String(ch)
            if s == "." or s == "@":
                var v: Int = 1 if s == "@" else 0
                if count == 0:
                    row[0] = v
                else:
                    row.append(v)
                count += 1

        if count == 0:
            continue

        if row_count == 0:
            # Explicitly copy row into grid[0]
            grid[0] = row.copy()
            width = count
        else:
            # Explicitly copy row when appending
            grid.append(row.copy())

        row_count += 1

    if row_count == 0 or width == 0:
        return 0

    var height = row_count
    var accessible: Int = 0

    var y: Int = 0
    while y < height:
        var x: Int = 0
        while x < width:
            if grid[y][x] == 0:
                x += 1
                continue

            # Count neighboring '@' (value 1)
            var neighbor_rolls: Int = 0
            var dy: Int = -1
            while dy <= 1:
                var dx: Int = -1
                while dx <= 1:
                    if dx == 0 and dy == 0:
                        dx += 1
                        continue

                    var ny = y + dy
                    var nx = x + dx

                    if ny >= 0 and nx >= 0 and ny < height and nx < width:
                        if grid[ny][nx] == 1:
                            neighbor_rolls += 1
                    dx += 1
                dy += 1

            if neighbor_rolls < 4:
                accessible += 1

            x += 1
        y += 1

    return accessible


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
    print("Single run time (Mojo, Day 4.1):", elapsed_ms, "ms")

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
        "Stress test (Day 4.1):",
        iterations,
        "iterations in",
        stress_ms,
        "ms",
    )
    print("Average per run:", stress_ms / Float64(iterations), "ms")
