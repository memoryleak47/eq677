# This generates a random gluing model of multiple 11-sized models glued together.
# The steiner system is the affine plane of order 11.

import random

# If you want another model, change this seed.
random.seed(1)

def load_magma(m):
    l = open("db/" + m).read()
    lines = l.split("\n")
    m = dict()
    for i, line in enumerate(lines):
        line = line.strip()
        for j, v in enumerate([x for x in line.split(" ") if x]):
            m[(i, j)] = int(v)
    return m

def check677(m):
    points = list(set(m.values()))
    for x in points:
        for y in points:
            f = lambda x, y: m[(x, y)]
            assert(x == f(y, f(x, f(f(y, x), y))))

def dump(m):
    points = list(set(m.values()))
    for x in points:
        for y in points:
            z = m[(x, y)]
            print(points.index(z), end=" ")
        print()

def generate_affine_plane_order_11():
    n = 11
    points = [(x, y) for x in range(n) for y in range(n)]
    blocks = []

    for m in range(n):
        for b in range(n):
            line = []
            for x in range(n):
                y = (m * x + b) % n
                line.append((x, y))
            blocks.append(line)

    for c in range(n):
        line = []
        for y in range(n):
            line.append((c, y))
        blocks.append(line)

    return points, blocks

def random_glue(points, blocks):
    m = dict()
    for a in points:
        m[(a, a)] = a

    for block in blocks:
        i = random.choice([0, 1, 2, 3])
        m11 = load_magma("11/" + str(i))
        random.shuffle(block)
        for x in range(11):
            for y in range(11):
                m[(block[x], block[y])] = block[m11[(x, y)]]
    return m

points, blocks = generate_affine_plane_order_11()
m = random_glue(points, blocks)

check677(m)
dump(m)
