from z3 import *

x = [10, 85, 50, 7, 52, 24, 56, 55, 2, 28]

print("True mean: ", sum(x)/len(x))

mu = Real("mu")
#se = [Real(f"e{i}") for i in range(len(x))]
sse = Real("sse")

opt = Optimize()
#for i in range(len(x)):
#    opt.add(se[i] == (x[i] - mu)**2)
opt.add(sse == sum((x - mu)**2 for x in x))
opt.add(mu <= max(x))
opt.add(mu >= min(x))
opt.minimize(sse)
print(opt)
opt.check()
opt.model()
