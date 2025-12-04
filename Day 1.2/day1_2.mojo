from time import perf_counter


def hits_on_move(pos: Int, direction: String, dist: Int) -> Int:
    """
    Return number of times we hit 0 during this move, starting from pos.
    """
    if dist <= 0:
        return 0

    s = pos % 100
    hits = 0

    if direction == "R":
        delta = (100 - s) % 100
        if delta == 0:
            delta = 100
        if dist >= delta:
            hits = 1 + (dist - delta) // 100
    elif direction == "L":
        delta = s % 100
        if delta == 0:
            delta = 100
        if dist >= delta:
            hits = 1 + (dist - delta) // 100
    else:
        hits = 0

    return hits


def solve(text: String) -> Int:
    pos: Int = 50
    hits: Int = 0

    for line_slice in text.splitlines():
        trimmed = line_slice.strip()
        if len(trimmed) == 0:
            continue

        line = String(trimmed)
        dir_slice = line[0:1]
        direction = String(dir_slice)
        dist_slice = line[1:]

        # Carefully parse distance
        try:
            dist: Int = Int(dist_slice)
        except:
            continue

        # Count hits during this move
        h = hits_on_move(pos, direction, dist)
        hits += h

        # Update position for next instruction
        if direction == "R":
            pos = (pos + dist) % 100
        elif direction == "L":
            pos = (pos - dist) % 100

    return hits


def main():
    # Read input.txt
    try:
        f = open("input.txt", "r")
    except:
        print("Error: could not open input.txt")
        return

    text = f.read()
    f.close()

    # Single run timing
    t0 = perf_counter()
    answer = solve(text)
    t1 = perf_counter()
    elapsed_ms = (t1 - t0) * 1000.0

    print(answer)
    print("Single run time (Mojo, Part 2):", elapsed_ms, "ms")

    # CPU stress test: many iterations
    iterations: Int = 10_000
    t2 = perf_counter()
    s: Int = 0
    for _ in range(iterations):
        s += solve(text)
    t3 = perf_counter()

    stress_ms = (t3 - t2) * 1000.0
    print("Dummy sum:", s)
    print(
        "Stress test (Part 2):",
        iterations,
        "iterations in",
        stress_ms,
        "ms (avg",
        stress_ms / Float64(iterations),
        "ms per run)",
    )
