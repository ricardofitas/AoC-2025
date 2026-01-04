import time
from qiskit import QuantumCircuit
from qiskit_aer import AerSimulator


def parse_line(line: str):
    line = line.strip()
    if not line:
        return None

    # diagram in [...]
    a = line.index('[')
    b = line.index(']')
    diagram = line[a+1:b]
    n = len(diagram)
    target = 0
    for i, ch in enumerate(diagram):
        if ch == '#':
            target |= 1 << i

    # parse (...) until '{'
    buttons = []
    i = b + 1
    while i < len(line):
        if line[i] == '{':
            break
        if line[i] != '(':
            i += 1
            continue
        j = line.index(')', i)
        inside = line[i+1:j].strip()
        mask = 0
        if inside:
            for tok in inside.split(','):
                tok = tok.strip()
                if tok:
                    mask |= 1 << int(tok)
        buttons.append(mask)
        i = j + 1

    return (n, target, buttons)


def min_weight(target, buttons):
    # Meet-in-the-middle
    m = len(buttons)
    mid = m // 2
    A = buttons[:mid]
    B = buttons[mid:]

    best = {}
    for s in range(1 << len(A)):
        x = 0
        w = 0
        for i in range(len(A)):
            if (s >> i) & 1:
                x ^= A[i]
                w += 1
        if x not in best or w < best[x]:
            best[x] = w

    ans = 10**9
    for s in range(1 << len(B)):
        x = 0
        w = 0
        for i in range(len(B)):
            if (s >> i) & 1:
                x ^= B[i]
                w += 1
        need = target ^ x
        if need in best:
            ans = min(ans, w + best[need])
    return ans


def solve(text: str) -> int:
    total = 0
    for line in text.splitlines():
        parsed = parse_line(line)
        if not parsed:
            continue
        _, target, buttons = parsed
        total += min_weight(target, buttons)
    return total


def toy_circuit():
    qc = QuantumCircuit(4, 1)
    qc.x(0)
    qc.cx(0, 1)
    qc.cx(1, 2)
    qc.cx(2, 3)
    qc.measure(3, 0)
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
    print("Toy GF(2) toggle demo circuit:")
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
