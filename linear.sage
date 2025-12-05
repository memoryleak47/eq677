import itertools
import sys

def direct_product(g1, g2):
    return AdditiveAbelianGroup(list(g1.invariants()) + list(g2.invariants()), remember_generators=False)

def primepower_groups(p, n, min=1):
    if n == 0:
        yield AdditiveAbelianGroup([])
    for i in range(min, n+1):
        for rest in primepower_groups(p, n-i, min=i):
            one = AdditiveAbelianGroup([p ** i])
            yield direct_product(one, rest)

def groups_of_n(n):
    out = [AdditiveAbelianGroup([])]
    for (p, i) in factor(n):
        out2 = []
        for o in out:
            for g in primepower_groups(p, i):
                out2.append(direct_product(o, g))
        out = out2
    return out

def check_f(f, g):
    for x in g:
        for y in g:
            if x != f(y, f(x, f(f(y, x), y))):
                return False
    return True

def is_auto(e, g):
    for x in g:
        if x == g[0]: continue
        if e(x) == g[0]: return False
    return True

def filter_autos(es, g):
    for e in es:
        if is_auto(e, g):
            yield e

def f_from_g(g):
    es = list(endos(g))
    autos = list(filter_autos(es, g))
    size = len(es) * len(autos) * g.order()
    if size > 100_000: return
    print("len(es) = " + str(len(es)) + ", len(autos) = " + str(len(autos)) +  ", iterating over a space of " + str(size))
    sys.stdout.flush()
    for a in autos:
        for b in es:
            for c in g:
                def f(x, y):
                    return a(x) + b(y) + c
                yield (f, (a, b, c))

class Endo:
    def __init__(self, d):
        self.d = d

    def __call__(self, x):
        return sum(y*(self.d)[i] for i, y in enumerate(x._vector_()))

    def __str__(self):
        return "Endo(" + str(self.d) + ")"

def endos(G):
    n = len(G.gens())
    for gs in itertools.product(*([G] * n)):
        yield Endo(list(gs))

for n in range(1, 100+1):
    for g in groups_of_n(n):
        print()
        print("looking at group:", g)
        print()
        sys.stdout.flush()
        for (f, (a, b, c)) in f_from_g(g):
            if check_f(f, g):
                print()
                print("Model found!")
                print(g)
                print("f(x, y) = " + str(a.d) + "(x) + " + str(b.d) + "(y) + " + str(c))
                ll = list(g)
                n = len(ll)
                for x in range(n):
                    for y in range(n):
                        z = f(g[x], g[y])
                        i = ll.index(z)
                        print(i, end=" ")
                    print()
                sys.stdout.flush()
