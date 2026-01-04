import time
from qiskit import QuantumCircuit
from qiskit_aer import AerSimulator


def parse_shape_area(lines):
    # 6 shapes, header "k:" then 3 lines
    area = [0]*6
    i = 0
    while i < len(lines):
        s = lines[i].strip()
        if not s:
            i += 1
            continue
        if ":" in s and "x" in s and s.split(":",1)[0].replace("x","").replace(" ","").isdigit():
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


def solve(text):
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


def toy_circuit():
    qc = QuantumCircuit(3, 1)
    qc.x(0)
    qc.cx(0, 1)
    qc.cx(1, 2)
    qc.measure(2, 0)
    return qc


def main():
    with open("input.txt", "r", encoding="utf-8") as f:
        text = f.read()

    t0 = time.perf_counter()
    ans = solve(text)
    t1 = time.perf_counter()

    print(ans)
    print(f"Classical time: {(t1 - t0)*1000:.3f} ms\n")

    qc = toy_circuit()
    print("Toy area-feasibility demo circuit:")
    print(qc.draw(output="text"))

    sim = AerSimulator()
    t2 = time.perf_counter()
    res = sim.run(qc, shots=1024).result()
    t3 = time.perf_counter()

    counts = res.get_counts()
    top = sorted(counts.items(), key=lambda kv: kv[1], reverse=True)[:5]
    print(f"Top measurement results (puzzle circuit): {top}")
    print(f"Simulator time: {(t3 - t2)*1000:.3f} ms")


if __name__ == "__main__":
    main()
