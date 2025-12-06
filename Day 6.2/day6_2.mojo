from time import perf_counter


fn is_digit(ch: String) -> Bool:
    return (
        ch == "0" or ch == "1" or ch == "2" or ch == "3" or ch == "4" or
        ch == "5" or ch == "6" or ch == "7" or ch == "8" or ch == "9"
    )


def solve(text: String) -> Int:
    # Collect lines
    var lines = [String("")]
    var height: Int = 0
    var width: Int = 0

    for slice_line in text.splitlines():
        var line = String(slice_line)
        if height == 0:
            lines[0] = line
        else:
            lines.append(line)

        var len_line = len(line)
        if len_line > width:
            width = len_line
        height += 1

    if height == 0:
        return 0

    # Find operator row (bottom-most with '+' or '*')
    var op_row: Int = -1
    var r: Int = height - 1
    while r >= 0:
        var line = lines[r]
        var n = len(line)
        var has_op = False
        var c: Int = 0
        while c < n:
            var ch = String(line[c])
            if ch == "+" or ch == "*":
                has_op = True
                break
            c += 1
        if has_op:
            op_row = r
            break
        r -= 1

    if op_row < 0:
        return 0

    # Column blank detection: 1 = blank, 0 = non blank
    var col_blank = [Int(1)]
    var i: Int = 1
    while i < width:
        col_blank.append(1)
        i += 1

    var col: Int = 0
    while col < width:
        var is_blank = True
        r = 0
        while r < height:
            var line = lines[r]
            var n = len(line)
            var ch = " "
            if col < n:
                ch = String(line[col])
            if ch != " ":
                is_blank = False
                break
            r += 1
        if not is_blank:
            col_blank[col] = 0
        col += 1

    # Problem ranges (contiguous non-blank columns)
    var starts = [Int(0)]
    var ends = [Int(0)]
    var prob_count: Int = 0
    var in_group = False
    var group_start: Int = 0

    col = 0
    while col < width:
        if col_blank[col] == 0:
            if not in_group:
                in_group = True
                group_start = col
        else:
            if in_group:
                var group_end = col - 1
                if prob_count == 0:
                    starts[0] = group_start
                    ends[0] = group_end
                else:
                    starts.append(group_start)
                    ends.append(group_end)
                prob_count += 1
                in_group = False
        col += 1

    if in_group:
        var group_end = width - 1
        if prob_count == 0:
            starts[0] = group_start
            ends[0] = group_end
        else:
            starts.append(group_start)
            ends.append(group_end)
        prob_count += 1

    if prob_count == 0:
        return 0

    var grand_total: Int = 0

    # For each problem, read numbers column-wise (top->bottom)
    var p: Int = 0
    while p < prob_count:
        var start_col = starts[p]
        var end_col = ends[p]

        # Find operator in op_row
        var op = ""
        var op_line = lines[op_row]
        var op_len = len(op_line)
        col = start_col
        while col <= end_col and col < op_len:
            var ch = String(op_line[col])
            if ch == "+" or ch == "*":
                op = ch
                break
            col += 1

        if op == "":
            p += 1
            continue

        # Numbers: one per column in this block
        var nums = [Int(0)]
        var num_count: Int = 0

        col = start_col
        while col <= end_col:
            var s = String("")
            r = 0
            while r < op_row:
                var line = lines[r]
                var n = len(line)
                var ch = " "
                if col < n:
                    ch = String(line[col])
                if is_digit(ch):
                    s = s + ch
                r += 1

            if len(s) > 0:
                var val = Int(s)
                if num_count == 0:
                    nums[0] = val
                else:
                    nums.append(val)
                num_count += 1

            col += 1

        if num_count == 0:
            p += 1
            continue

        var res = nums[0]
        if op == "+":
            var idx: Int = 1
            while idx < num_count:
                res += nums[idx]
                idx += 1
        elif op == "*":
            var idx2: Int = 1
            while idx2 < num_count:
                res *= nums[idx2]
                idx2 += 1
        else:
            p += 1
            continue

        grand_total += res
        p += 1

    return grand_total


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
    print("Single run time (Mojo, Day 6.2):", elapsed_ms, "ms")

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
        "Stress test (Day 6.2):",
        iterations,
        "iterations in",
        stress_ms,
        "ms",
    )
    print("Average per run:", stress_ms / Float64(iterations), "ms")
