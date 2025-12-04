from time import perf_counter


def solve(text: String) -> Int:
    # Build grid as list of rows, each row is list[Int] with 1 for '@' and 0 for '.'
    var grid = [[0]]  # non-empty so Mojo infers List[List[Int]]
    var row_count: Int = 0
    var width: Int = 0

    for line_slice in text.splitlines():
        var trimmed = line_slice.strip()
        if len(trimmed) == 0:
            continue

        var line = String(trimmed)

        var row = [0]
        var count: Int = 0

        for ch in line:
            var s = String(ch)
            if s == "@":
                if count == 0:
                    row[0] = 1
                else:
                    row.append(1)
                count += 1
            elif s == ".":
                if count == 0:
                    row[0] = 0
                else:
                    row.append(0)
                count += 1

        if count == 0:
            continue

        if row_count == 0:
            grid[0] = row.copy()
            width = count
        else:
            grid.append(row.copy())

        row_count += 1

    if row_count == 0 or width == 0:
        return 0

    var height = row_count
    var total_removed: Int = 0

    # Iteratively remove accessible rolls until none remain
    while True:
        # lists of positions to remove this round
        var to_remove_y = [0]
        var to_remove_x = [0]
        var rem_count: Int = 0

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
                    if rem_count == 0:
                        to_remove_y[0] = y
                        to_remove_x[0] = x
                    else:
                        to_remove_y.append(y)
                        to_remove_x.append(x)
                    rem_count += 1

                x += 1
            y += 1

        if rem_count == 0:
            break

        # Remove all marked rolls in this round
        var i: Int = 0
        while i < rem_count:
            var ry = to_remove_y[i]
            var rx = to_remove_x[i]
            if grid[ry][rx] == 1:
                grid[ry][rx] = 0
                total_removed += 1
            i += 1

    return total_removed


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
    print("Single run time (Mojo, Day 4.2):", elapsed_ms, "ms")

    # CPU stress test (heavier than Part 1, so keep iterations modest)
    var iterations: Int = 200
    var t2 = perf_counter()
    var s: Int = 0
    for _ in range(iterations):
        s += solve(text)
    var t3 = perf_counter()

    var stress_ms = (t3 - t2) * 1000.0
    print("Dummy sum:", s)
    print(
        "Stress test (Day 4.2):",
        iterations,
        "iterations in",
        stress_ms,
        "ms",
    )
    print("Average per run:", stress_ms / Float64(iterations), "ms")
