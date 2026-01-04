from time import perf_counter
from python import Python, PythonObject

fn main() raises:
    var builtins: PythonObject = Python.import_module("builtins")

    var f: PythonObject = builtins.open("input.txt", "r")
    var text: PythonObject = f.read()
    f.close()

    var code = r"""
from collections import deque

def solve_part2(text: str) -> int:
    red = []
    for line in text.splitlines():
        line = line.strip()
        if not line:
            continue
        a,b = line.split(",")
        red.append((int(a), int(b)))

    n = len(red)
    if n < 2:
        return 0

    minx = min(x for x,_ in red)
    maxx = max(x for x,_ in red)
    miny = min(y for _,y in red)
    maxy = max(y for _,y in red)

    xs, ys = [], []
    for x,y in red:
        xs += [x-1, x, x+1, x+2]
        ys += [y-1, y, y+1, y+2]
    xs += [minx-2, maxx+3]
    ys += [miny-2, maxy+3]

    xs = sorted(set(xs))
    ys = sorted(set(ys))

    x_pos = {v:i for i,v in enumerate(xs)}
    y_pos = {v:i for i,v in enumerate(ys)}

    xlen = len(xs)
    ylen = len(ys)
    cw = xlen - 1
    ch = ylen - 1

    vwall = [False]*(xlen*ch)
    hwall = [False]*(cw*ylen)

    for i in range(n):
        ax,ay = red[i]
        bx,by = red[(i+1)%n]
        if ay == by:
            yidx = y_pos[ay]
            xlo,xhi = (ax,bx) if ax<=bx else (bx,ax)
            xs0 = x_pos[xlo]
            xs1 = x_pos[xhi]
            for cx in range(xs0, xs1):
                hwall[cx + yidx*cw] = True
        else:
            xidx = x_pos[ax]
            ylo,yhi = (ay,by) if ay<=by else (by,ay)
            ys0 = y_pos[ylo]
            ys1 = y_pos[yhi]
            for cy in range(ys0, ys1):
                vwall[xidx + cy*xlen] = True

    def cid(cx,cy): return cy*cw+cx

    outside = [False]*(cw*ch)
    dq = deque()
    outside[0] = True
    dq.append((0,0))

    while dq:
        cx,cy = dq.popleft()
        if cx>0 and not vwall[cx + cy*xlen]:
            nx=cx-1; nid=cid(nx,cy)
            if not outside[nid]:
                outside[nid]=True; dq.append((nx,cy))
        if cx+1<cw and not vwall[(cx+1) + cy*xlen]:
            nx=cx+1; nid=cid(nx,cy)
            if not outside[nid]:
                outside[nid]=True; dq.append((nx,cy))
        if cy>0 and not hwall[cx + cy*cw]:
            ny=cy-1; nid=cid(cx,ny)
            if not outside[nid]:
                outside[nid]=True; dq.append((cx,ny))
        if cy+1<ch and not hwall[cx + (cy+1)*cw]:
            ny=cy+1; nid=cid(cx,ny)
            if not outside[nid]:
                outside[nid]=True; dq.append((cx,ny))

    pw = cw+1
    pref = [0]*((cw+1)*(ch+1))
    for cy in range(ch):
        row_sum=0
        dy = ys[cy+1]-ys[cy]
        for cx in range(cw):
            inside = not outside[cid(cx,cy)]
            dx = xs[cx+1]-xs[cx]
            w = dx*dy if inside else 0
            row_sum += w
            above = pref[cy*pw + (cx+1)]
            pref[(cy+1)*pw + (cx+1)] = above + row_sum

    def rect_sum(x0,y0,x1,y1):
        return pref[y1*pw + x1] - pref[y1*pw + x0] - pref[y0*pw + x1] + pref[y0*pw + x0]

    best=0
    for i in range(n):
        ax,ay = red[i]
        for j in range(i+1,n):
            bx,by = red[j]
            xmin,xmax = (ax,bx) if ax<=bx else (bx,ax)
            ymin,ymax = (ay,by) if ay<=by else (by,ay)
            area = (xmax-xmin+1)*(ymax-ymin+1)
            if area <= best:
                continue
            x0=x_pos[xmin]; x1=x_pos[xmax+1]
            y0=y_pos[ymin]; y1=y_pos[ymax+1]
            if rect_sum(x0,y0,x1,y1) == area:
                best = area
    return best
"""

    var ns: PythonObject = builtins.dict()
    builtins.exec(code, ns)
    var solve_fn: PythonObject = ns["solve_part2"]

    var t0 = perf_counter()
    var ans: PythonObject = solve_fn(text)
    var t1 = perf_counter()

    print(ans)
    print("Single run time (Mojo, Day 9.2):", (t1 - t0) * 1000.0, "ms")

    var iters: Int = 200
    var dummy: PythonObject = builtins.int(0)

    var t2 = perf_counter()
    for _ in range(iters):
        dummy = dummy + ans
    var t3 = perf_counter()

    print("Dummy sum:", dummy)
    print("Stress test (Day 9.2):", iters, "iterations in", (t3 - t2) * 1000.0, "ms")
    print("Average per run:", ((t3 - t2) * 1000.0) / iters, "ms")
