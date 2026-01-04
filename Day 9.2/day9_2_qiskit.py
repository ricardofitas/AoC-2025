import time
from collections import deque

from qiskit import QuantumCircuit
from qiskit_aer import AerSimulator


def parse_points(path="input.txt"):
    pts = []
    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            s = line.strip()
            if not s:
                continue
            x, y = s.split(",")
            pts.append((int(x), int(y)))
    return pts


def solve_part2(red):
    n = len(red)
    if n < 2:
        return 0

    minx = min(p[0] for p in red)
    maxx = max(p[0] for p in red)
    miny = min(p[1] for p in red)
    maxy = max(p[1] for p in red)

    xs, ys = [], []
    for x, y in red:
        xs.extend([x - 1, x, x + 1, x + 2])
        ys.extend([y - 1, y, y + 1, y + 2])

    xs.extend([minx - 2, maxx + 3])
    ys.extend([miny - 2, maxy + 3])

    xs = sorted(set(xs))
    ys = sorted(set(ys))

    x_pos = {v: i for i, v in enumerate(xs)}
    y_pos = {v: i for i, v in enumerate(ys)}

    xlen = len(xs)
    ylen = len(ys)
    cw = xlen - 1
    ch = ylen - 1

    # walls
    vwall = [False] * (xlen * ch)   # vwall[xidx + cy*xlen]
    hwall = [False] * (cw * ylen)   # hwall[cx + yidx*cw]

    for i in range(n):
        ax, ay = red[i]
        bx, by = red[(i + 1) % n]

        if ay == by:
            yidx = y_pos[ay]
            xlo, xhi = (ax, bx) if ax <= bx else (bx, ax)
            xs0 = x_pos[xlo]
            xs1 = x_pos[xhi]
            for cx in range(xs0, xs1):
                hwall[cx + yidx * cw] = True
        else:
            xidx = x_pos[ax]
            ylo, yhi = (ay, by) if ay <= by else (by, ay)
            ys0 = y_pos[ylo]
            ys1 = y_pos[yhi]
            for cy in range(ys0, ys1):
                vwall[xidx + cy * xlen] = True

    def cid(cx, cy):
        return cy * cw + cx

    outside = [False] * (cw * ch)
    dq = deque()
    outside[0] = True
    dq.append((0, 0))

    while dq:
        cx, cy = dq.popleft()

        if cx > 0 and not vwall[cx + cy * xlen]:
            nx = cx - 1
            nid = cid(nx, cy)
            if not outside[nid]:
                outside[nid] = True
                dq.append((nx, cy))

        if cx + 1 < cw and not vwall[(cx + 1) + cy * xlen]:
            nx = cx + 1
            nid = cid(nx, cy)
            if not outside[nid]:
                outside[nid] = True
                dq.append((nx, cy))

        if cy > 0 and not hwall[cx + cy * cw]:
            ny = cy - 1
            nid = cid(cx, ny)
            if not outside[nid]:
                outside[nid] = True
                dq.append((cx, ny))

        if cy + 1 < ch and not hwall[cx + (cy + 1) * cw]:
            ny = cy + 1
            nid = cid(cx, ny)
            if not outside[nid]:
                outside[nid] = True
                dq.append((cx, ny))

    pw = cw + 1
    pref = [0] * ((cw + 1) * (ch + 1))

    for cy in range(ch):
        row_sum = 0
        dy = ys[cy + 1] - ys[cy]
        for cx in range(cw):
            inside = not outside[cid(cx, cy)]
            dx = xs[cx + 1] - xs[cx]
            w = dx * dy if inside else 0
            row_sum += w
            above = pref[cy * pw + (cx + 1)]
            pref[(cy + 1) * pw + (cx + 1)] = above + row_sum

    def rect_sum(x0, y0, x1, y1):
        # [x0..x1) x [y0..y1)
        return pref[y1 * pw + x1] - pref[y1 * pw + x0] - pref[y0 * pw + x1] + pref[y0 * pw + x0]

    best = 0
    for i in range(n):
        ax, ay = red[i]
        for j in range(i + 1, n):
            bx, by = red[j]
            xmin, xmax = (ax, bx) if ax <= bx else (bx, ax)
            ymin, ymax = (ay, by) if ay <= by else (by, ay)

            area = (xmax - xmin + 1) * (ymax - ymin + 1)
            if area <= best:
                continue

            x0 = x_pos[xmin]
            x1 = x_pos[xmax + 1]
            y0 = y_pos[ymin]
            y1 = y_pos[ymax + 1]

            if rect_sum(x0, y0, x1, y1) == area:
                best = area

    return best


def toy_circuit():
    qc = QuantumCircuit(4, 1)
    qc.x(0)
    qc.cx(0, 3)
    qc.ccx(0, 1, 3)
    qc.measure(3, 0)
    return qc


def main():
    red = parse_points("input.txt")

    t0 = time.perf_counter()
    ans = solve_part2(red)
    t1 = time.perf_counter()

    print(ans)
    print(f"Classical time: {(t1 - t0)*1000:.3f} ms\n")

    qc = toy_circuit()
    print("Toy polygon-fill flag circuit:")
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
