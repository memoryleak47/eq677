from z3 import *

K = 5
F = Datatype('F')
for i in range(K):
    F.declare('z' + str(i))
F = F.create()

vals = [F.constructor(i)() for i in range(K)]

s = Solver()

add = Function('add', F, F, F)
mul = Function('mul', F, F, F)

for i, a in enumerate(vals):
    for j, b in enumerate(vals):
        s.add(add(a, b) == vals[(i + j) % K])
        s.add(mul(a, b) == vals[(i * j) % K])

BASE = "7/1"

if BASE == "7/0":
    L = ['0000', '1251', '2716', '3487', '4538', '5862', '6143', '7374', '8625']
if BASE == "7/1":
    L = ['0000', '1241', '2718', '3826', '4634', '5582', '6377', '7455', '8163']

A = [Const(f'a{i}', F) for i in range(len(L))]
B = [Const(f'b{i}', F) for i in range(len(L))]
C = [Const(f'c{i}', F) for i in range(len(L))]

def f(tr_char, x, y):
    tr = int(tr_char)

    a = mul(A[tr], x)
    b = mul(B[tr], y)
    c = C[tr]
    return add(add(a, b), c)

def expr(t, x, y):
    # returns f(y, f(x, f(f(y, x), y)))
    layer0 = f(t[0], y, x)
    layer1 = f(t[1], layer0, y)
    layer2 = f(t[2], x, layer1)
    layer3 = f(t[3], y, layer2)
    return layer3

for i in range(len(L)):
    t = L[i]
    s.add(expr(t, F.z0, F.z1) == F.z0)
    s.add(expr(t, F.z1, F.z0) == F.z1)
    s.add(expr(t, F.z0, F.z0) == F.z0)

    # To simplify
    s.add(C[i] == F.z0)

def inv(x):
    if x == 0: raise "ohno"
    for i in range(7):
        if (i*x)%7 == 1: return i

def idx_of(x, y):
    if x==0:
        if y==0: return 0
        return 1
    dd = (y*inv(x))%7
    if dd==0: return 2
    if dd==1: return 8
    if dd==2: return 7
    if dd==3: return 6
    if dd==4: return 5
    if dd==5: return 4
    if dd==6: return 3
    raise "ohno2"

def check_sol(As, Bs, Cs):
    p = []
    for i in range(7):
        for j in range(K):
            p.append((i, j))

    def f(x, y):
        if BASE == "7/0":
            l = (x[0]*4 + y[0]*1)%7
        if BASE == "7/1":
            l = (x[0]*4 + y[0]*3)%7
        i = idx_of(x[0], y[0])
        r = (As[i]*x[1] + Bs[i]*y[1] + Cs[i])%K
        return (l, r)

    for x in p: 
        for y in p: 
            assert(x == f(y, f(x, f(f(y, x), y))))

    for x in p: 
        assert(x == f(f(f(x, x), x), x))

    for i, x in enumerate(p):
        for j, y in enumerate(p):
            k = p.index(f(x, y))
            print(k, end=" ")
        print()

while s.check() == sat:
    m = s.model()
    print("solution found:")

    def zz(x):
        v = m.eval(x, model_completion=True)
        return int(str(v)[1:])

    As = [zz(A[i]) for i in range(len(L))]
    Bs = [zz(B[i]) for i in range(len(L))]
    Cs = [zz(C[i]) for i in range(len(L))]
    for i in range(len(L)):
        print(f"A[{i}] =", As[i])
        print(f"B[{i}] =", Bs[i])
        print(f"C[{i}] =", Cs[i])
    check_sol(As, Bs, Cs)

    l = []
    for i in range(len(L)):
        l.append(A[i] != vals[As[i]])
        l.append(B[i] != vals[Bs[i]])
        l.append(C[i] != vals[Cs[i]])
    s.add(Or(*l))

print("no more solutions")
