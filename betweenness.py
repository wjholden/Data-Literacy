from dijkstra import *

# https://snap.stanford.edu/class/cs224w-readings/brandes01centrality.pdf
sigma = defaultdict(lambda: 0)

for s in g.keys():
    for t in g.keys():
        path, _ = dijkstra(s, t)
        path = path[1:-1]
        for v in path:
            sigma[v] += 1

print(sigma)