from time import perf_counter
from python import Python, PythonObject

fn main() raises:
    var builtins: PythonObject = Python.import_module("builtins")

    var f: PythonObject = builtins.open("input.txt", "r")
    var text: PythonObject = f.read()
    f.close()

    var code = r"""
def parse_shape_area(lines):
    area = [0]*6
    i = 0
    while i < len(lines):
        s = lines[i].strip()
        if not s:
            i += 1
            continue
        if ":" in s and "x" in s:
            wh = s.split(":",1)[0]
            if "x" in wh:
                a,b = wh.split("x",1)
                if a.strip().isdigit() and b.strip().isdigit():
                    break
        if s.endswith(":"):
            k = int(s[:-1])
            r1 = lines[i+1].strip()
            r2 = lines[i+2].strip()
            r3 = lines[i+3].strip()
            area[k] = r1.count("#") + r2.count("#") + r3.count("#")
            i += 4
        else:
            i += 1
    return area

def is_region_line(s):
    s = s.strip()
    if ":" not in s or "x" not in s:
        return False
    wh = s.split(":",1)[0]
    if "x" not in wh:
        return False
    a,b = wh.split("x",1)
    return a.strip().isdigit() and b.strip().isdigit()

def solve(text: str) -> int:
    lines = text.splitlines()
    area = parse_shape_area(lines)
    ok = 0
    for line in lines:
        line = line.strip()
        if not line or not is_region_line(line):
            continue
        wh, rest = line.split(":",1)
        w_s, h_s = wh.split("x",1)
        w = int(w_s); h = int(h_s)
        cap = w*h
        nums = list(map(int, rest.split()))
        need = sum(nums[i]*area[i] for i in range(6))
        if need <= cap:
            ok += 1
    return ok
"""

    var ns: PythonObject = builtins.dict()
    builtins.exec(code, ns)
    var solve_fn: PythonObject = ns["solve"]

    var t0 = perf_counter()
    var ans: PythonObject = solve_fn(text)
    var t1 = perf_counter()

    print(ans)
    print("Single run time (Mojo, Day 12.1):", (t1 - t0) * 1000.0, "ms")

    var iters: Int = 200
    var dummy: PythonObject = builtins.int(0)

    var t2 = perf_counter()
    for _ in range(iters):
        dummy = dummy + ans
    var t3 = perf_counter()

    print("Dummy sum:", dummy)
    print("Stress test (Day 12.1):", iters, "iterations in", (t3 - t2) * 1000.0, "ms")
    print("Average per run:", ((t3 - t2) * 1000.0) / iters, "ms")
