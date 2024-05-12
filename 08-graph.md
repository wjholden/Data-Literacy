# Graph Theory {#chapter:graph}

## Vertices, edges, and paths

A *graph* ($G$) is a collection of *vertices* ($V$; also known as *nodes* or *points*) and the *edges*
($E$; also known as *relations* or *lines*; see section \ref{section:discrete-math}) connecting them.

$$
G = \left\{ V , E \right\}
$$

Edges are conventionally named for the vertices they connect.
For example, let $V = \left\{ u , v \right\}$ and $E = \left\{ \left( u, v \right) \right\}$,
then $G = \left\{ V, E \right\}$ is a graph with two vertices, $u$ and $v$, and one edge, $(u, v)$.

Graphs can be used to model any form of *network* where the elements of sets bear relations.
Graphs can be directed, where the source and destination vertices in an edge are significant ($(u, v) \ne (v, u)$),
or undirected ($(u,v) = (v,u)$).

The edges of a graph may have a *distance function* ($\delta$; also known as *weight* and *cost*), which relates each edge with some real number.
Such graphs are *weighted graphs*.
For example, suppose a high-speed railroad has train stations in Paris, Brussels, and the Hague.
The distance from Paris to Brussels is about $\qty{350}\km$ and the distance from Brussels to Hague is another $\qty{180}\km$.
The total length of the *path* from Paris to Hague via Brussels is therefore $\qty{350}\km + \qty{180}\km = \qty{530}\km$.

$$
\begin{aligned}
\delta \left( \text{Paris} , \text{Hague} \right) &= \delta \left( \text{Paris} , \text{Brussels} \right) + \delta \left( \text{Brussels} , \text{Hague} \right) \\
&= \qty{350}\km + \qty{180}\km \\
&= \qty{530}\km.
\end{aligned}
$$

Let us quickly reproduce this result using a *graph database* named Neo4j.
Go to https://console.neo4j.org and click the "Clear DB" button.
Enter the below *Cypher query* into the input field and press the triangle-shaped execute button.

```
CREATE
    (paris:CITY {name:"Paris"}),
    (brussels:CITY {name:"Brussels"}),
    (hague:CITY {name:"Hague"}),
    (paris)-[:ROUTE {dist:350}]->(brussels),
    (brussels)-[:ROUTE {dist:180}]->(hague);
```

This command created three vertices and two edges.
Verify and visualize this graph with the below query.
Neo4j's Cypher language uses `MATCH` as its selection ($\sigma$) operator.

```
MATCH (x:CITY)
RETURN x;
```

Finally, query the database for a path of any length (`*`) connecting Paris to Hague,
returning the path (`p`) and its cumulative distance.
Refer to section \ref{section:filter-map-reduce} for a description of the reduce operation.

```
MATCH (:CITY {name:'Paris'})-[p:ROUTE*]->(:CITY {name:'Hague'}) 
RETURN p, REDUCE(length=0, e IN p | length + e.dist) AS distance;
```

## Special cases of graphs

A *directed acyclic graph* (DAG) is a special case of a directed graph.
A *cycle* (also known as a *loop*) occurs in a directed graph when there is any path from some vertex to itself.
For example, let $G$ be a directed graph where

$$
G = \left\{ \left\{ a, b, c, d \right\} ,
\left\{
    \left( a, b \right), 
    \left( b, c \right), 
    \left( c, d \right),
    \left( c, a \right) \right\}
\right\}.
$$

The edges $\left( a, b \right)$, $\left( b, c \right)$, and $\left( c, a \right)$ form a cycle.
Vertices $a$, $b$, and $c$ together form a *connected component*.
A directed graph with one or more connected components is not a DAG, but the graph of components
(where vertices of the component subgraph are merged into a "super vertex") is itself a DAG.

A *tree* is another special case of an acyclic graph, typically in undirected graphs.
One's ancestoral family tree is an instance of a tree; without a time machine, it is impossible to one to form a loop with an ancestor.

## Representation

Graphs are modeled using either an *adjacency list* or an *adjacency matrix*.
An adjacency list might look like

$$
\begin{aligned}
\text{adj} \left( \text{Paris} \right) &= \left\{ \text{Brussels} \right\} \\
\text{adj} \left( \text{Brussels} \right) &= \left\{ \text{Paris} , \text{Hague} \right\} \\
\text{adj} \left( \text{Hague} \right) &= \left\{ \text{Brussels} \right\}. \\
\end{aligned}
$$

A separate data structure would be necessary to represent edge weights.
*Dictionary* types, such as `dict` in Python and `map` in Go, can be convenient implementations for both adjacency lists and edge weight functions.

An adjacency matrix represents edges with weights in a single data structure, such as

$$
E = \bordermatrix{
& \text{Paris} & \text{Brussels} & \text{Hague} \cr
\text{Paris} & 0 & 350 & \infty \cr
\text{Brussels} & 350 & 0 & 180 \cr
\text{Hague} & \infty & 180 & 0
}.
$$

In this example, the distance of a vertex to itself is defined as zero,

$$
\delta \left( u, u \right) = 0
$$

and the distance between vertices is considered infinite if those vertices are not directly connected by an edge.

$$
\left(u , v\right) \notin E \implies \delta \left( u, v \right) = \infty
$$

## Search algorithms

Imagine a video game where the player searches the kingdom for treasure.
The player has no knowledge of where the treasure might be, so from a starting point they fully explore the forest, mountains, and sea.
Both the starting point and the sea connect to opposite sides of the city.
Upon entering the city from the sea-side, the player explores the city and discovers the treasure.
This is an example of a *depth-first search* (DFS).

Go to https://go.dev/play/p/AuH2qOgSG-c to run the following DFS implementation, written in Go.
This implementation uses a *recursive* definition of the DFS function (the DFS function invokes itself as it explores the graph).
The function uses an external data structure (`quest`) to identify which vertices have already been discovered.
Upon successful search, the function prints its position in the graph as the recursive calls "unwind."

```
package main

import "fmt"

var g = map[string][]string{
	"start":     []string{"forest", "mountains", "sea", "city"},
	"forest":    []string{"start", "mountains", "desert", "cave"},
	"mountains": []string{"start", "forest", "glacier"},
	"desert":    []string{"forest"},
	"cave":      []string{"forest", "inferno"},
	"inferno":   []string{"cave"},
	"glacier":   []string{"mountains"},
	"sea":       []string{"start", "beach"},
	"beach":     []string{"sea", "city"},
	"city":      []string{"beach", "start", "castle"},
	"castle":    []string{"city", "treasure"},
	"treasure":  []string{"castle"}}

var quest map[string]string = make(map[string]string)

func dfs(src, dst string) bool {
	quest[src] = "discovered"

	if src == dst {
		fmt.Printf("Discovered %s:", dst)
		return true
	}

	for _, neighbor := range g[src] {
		if quest[neighbor] != "discovered" {
			if dfs(neighbor, dst) == true {
				fmt.Printf(" %s", src)
				return true
			}
		}
	}

	return false
}

func main() {
	dfs("start", "treasure")
	fmt.Println()
}
```

The program should output `Discovered treasure: castle city beach sea start`.
DFS successfully discovers the treasure, but we have no guarantee that this algorithm will find the *optimal* (shortest) path.

Imagine the protagonist of our hypothetical adventure game was not a lone wanderer, but rather a field marshall commanding a large army.
This army explores one region at a time, holding each area as adjacent units proceed into their respective area.
The army incrementally expands the radius of the search *frontier*.
Once one unit discovers the treasure, we are certain that no shorter path was possible thanks to an *invariant* in our search algorithm.

Maintaining an invariant is essential for *mathematical induction*, where we establish that some *predicate* $P$ is true for the *base case* $P(0)$ and that $P(k)$ implies $P(k+1)$ and therefore $P(n)$ is true for all $n > 0$.
The proof for the correctness of a BFS follows:

1. Along the frontier of radius $r=0$, the BFS algorithm on graph $G$ has not discovered a path from $u$ to $v$. $\delta(u,v)$ is therefore at *closest* $r=1$.
2. At $r=1$, BFS has not found $v$ and therefore $2 \le \delta(u,v)$.
3. At $r=2$, BFS has not found $v$ and therefore $3 \le \delta(u,v)$.
4. $\vdots$
5. At $r=k$, BFS has not found $v$ and therefore $k+1 \le \delta(u,v)$.
6. $\vdots$
7. At $r=n$, BFS has located $v$ and therefore $n = \delta(u,v)$. $\square$

Gp to https://go.dev/play/p/yMIcmcsK_V9 and run the following BFS implementation, written in Go.
This implementation uses an *iterative* BFS function.
The BFS function does not invoke itself.
Instead, the procedure adds unexplored vertices to a queue and records the "parent" of each vertex.
We "unwind" the resulting tree from child to parent nodes to construct the shortest path.

```
func bfs(src, dst string) map[string]string {
	parent := map[string]string{src: src}
	queue := []string{src}
	for len(queue) > 0 {
		position := queue[0]
		queue = queue[1:]
		if position == dst {
			break
		}
		for _, neighbor := range g[position] {
			if _, ok := parent[neighbor]; !ok {
				parent[neighbor] = position
				queue = append(queue, neighbor)
			}
		}
	}
	return parent
}

func main() {
	tree := bfs("start", "treasure")
	fmt.Printf("Discovered treasure:")
	position := "treasure"
	for position != tree[position] {
		position = tree[position]
		fmt.Printf(" %s", position)
	}
    fmt.Println()
}
```

This program should output `Discovered treasure: castle city start`.
BFS finds the shortest path between two vertices by *hop count*, but it does not consider edge weights.
*Dijkstra's algorithm* performs a breadth-first traversal ordered by cumulative path cost.

<!--
Dijkstra in Go https://go.dev/play/p/hcNbTvGli-z
Too long and ugly to share.
-->

Dijkstra's algorithm uses a *priority queue* to visit nodes from shortest to longest path.
For this reason, Dijkstra's algorithm is also known as the *shortest-path first* (SPF).
Like BFS, Dikjstra's algorithm is a *greedy algorithm* that discovered a globally optimal solution by repeatedly making locally optimal decisions.
Let us turn to the Python language to demonstrate Dijkstra's algorithm on our same treasure-hunting graph, this time with edge weights.
Run this program at https://www.python.org/shell/.
(Note: the Python language is very "picky" about tabs.
A plain text verion of this program is available at https://github.com/wjholden/Data-Literacy/blob/main/dijkstra.py.)

```
from heapq import *

g = dict(
    [
        ("start", ["forest", "mountains", "sea", "city"]),
        ("forest", ["start", "mountains", "desert", "cave"]),
        ("mountains", ["start", "forest", "glacier"]),
        ("desert", ["forest"]),
        ("cave", ["forest", "inferno"]),
        ("inferno", ["cave"]),
        ("glacier", ["mountains"]),
        ("sea", ["start", "beach"]),
        ("beach", ["sea", "city"]),
        ("city", ["beach", "start", "castle"]),
        ("castle", ["city", "treasure"]),
        ("treasure", ["castle"]),
    ]
)

w = dict(
    [
        (("start", "forest"), 70),
        (("start", "mountains"), 60),
        (("start", "sea"), 54),
        (("start", "city"), 81),
        (("forest", "start"), 42),
        (("forest", "mountains"), 51),
        (("forest", "desert"), 56),
        (("forest", "cave"), 63),
        (("mountains", "start"), 71),
        (("mountains", "forest"), 38),
        (("mountains", "glacier"), 72),
        (("desert", "forest"), 93),
        (("cave", "forest"), 19),
        (("cave", "inferno"), 17),
        (("inferno", "cave"), 71),
        (("glacier", "mountains"), 25),
        (("sea", "start"), 49),
        (("sea", "beach"), 88),
        (("beach", "sea"), 79),
        (("beach", "city"), 29),
        (("city", "beach"), 30),
        (("city", "start"), 33),
        (("city", "castle"), 36),
        (("castle", "city"), 39),
        (("castle", "treasure"), 76),
        (("treasure", "castle"), 76),
    ]
)


def dijkstra(src, dst):
    explored = set()
    distance = dict()
    distance[src] = 0
    queue = []
    heappush(queue, (0, src))
    while queue:
        _, current = heappop(queue)
        if current == dst:
            print("Path found": distance[dst])
            return distance[dst]
        if current in explored:
            continue
        explored.add(current)
        for neighbor in g[current]:
            if neighbor not in explored:
                d = distance[current] + w[(current, neighbor)]
                distance[neighbor] = d
                heappush(queue, (d, neighbor))
    print("No path found")

dijkstra("start", "treasure")
```

This program should output `Path found: 193`, revealing that the shortest path from `start` to `treasure` has a total path cost of 193.

Neo4j produces the same result.
We reconstruct our graph in the Cypher language at https://console.neo4j.org:

```
CREATE
    (start:LOCATION {name:'start'}),
    (inferno:LOCATION {name:'inferno'}),
    (beach:LOCATION {name:'beach'}),
    (castle:LOCATION {name:'castle'}),
    (forest:LOCATION {name:'forest'}),
    (mountains:LOCATION {name:'mountains'}),
    (desert:LOCATION {name:'desert'}),
    (cave:LOCATION {name:'cave'}),
    (glacier:LOCATION {name:'glacier'}),
    (sea:LOCATION {name:'sea'}),
    (city:LOCATION {name:'city'}),
    (treasure:LOCATION {name:'treasure'}),
    (start)-[:CONN {distance:70}]->(forest),
    (start)-[:CONN {distance:60}]->(mountains),
    (start)-[:CONN {distance:54}]->(sea),
    (start)-[:CONN {distance:81}]->(city),
    (inferno)-[:CONN {distance:71}]->(cave),
    (beach)-[:CONN {distance:79}]->(sea),
    (beach)-[:CONN {distance:29}]->(city),
    (castle)-[:CONN {distance:39}]->(city),
    (castle)-[:CONN {distance:76}]->(treasure),
    (city)-[:CONN {distance:30}]->(beach),
    (city)-[:CONN {distance:33}]->(start),
    (city)-[:CONN {distance:36}]->(castle),
    (treasure)-[:CONN {distance:76}]->(castle),
    (forest)-[:CONN {distance:42}]->(start),
    (forest)-[:CONN {distance:51}]->(mountains),
    (forest)-[:CONN {distance:56}]->(desert),
    (forest)-[:CONN {distance:63}]->(cave),
    (mountains)-[:CONN {distance:71}]->(start),
    (mountains)-[:CONN {distance:38}]->(forest),
    (mountains)-[:CONN {distance:72}]->(glacier),
    (desert)-[:CONN {distance:93}]->(forest),
    (cave)-[:CONN {distance:19}]->(forest),
    (cave)-[:CONN {distance:17}]->(inferno),
    (glacier)-[:CONN {distance:25}]->(mountains),
    (sea)-[:CONN {distance:49}]->(start),
    (sea)-[:CONN {distance:88}]->(beach);
```

and then we can use the built-in `shortestPath` function to obtain the same result.

```
MATCH
    (src:LOCATION {name:'start'}),
    (dst:LOCATION {name:'treasure'}),
    path = shortestPath((src)-[:CONN*]->(dst))
RETURN
    path,
    REDUCE(d=0, e in relationships(path) | d + e.distance) AS distance;
```

Neo4j should also report a distance of 193.

As an exercise, change `->(dst)` to `-(dst)` and re-run the query with this change.
The total distance is now 145.
The difference in `()-[]->()` and `()-[]-()` is that one is a directed edge, the other is undirected.
With `-()` instead of `->()`, Neo4j treats all edges in the graph as undirected.
Neo4j allows *parallel edges* that connect the same two vertices.

## Discussion prompts

A graph can be represented with an adjacency list or a matrix. What are the advantages and disadvantages of each approach? 

What algorithm can be used to solve the “seven ways to Kevin Bacon” problem? 

Is a Gantt chart a graph? How can one find the critical path of a project if represented as a graph? 

Which is bigger, [Equation] or [Equation]? 


## Practical exercises

Compare two different heuristic functions in a provided A* informed search implementation on the 8-piece puzzle problem. 

Convert currency exchange rates from multiplication to addition using a logarithm, then prove that infinite arbitration is impossible given a set of exchange rates and Bellman-Ford implementation. 

Define a topological sorting and relate it to a workplace problem. 

Define the Traveling Salesman Problem (TSP) and explain the computational difficulty of this problem. 

Determine the minimum paving needed to fully connect a tent complex using a list of coordinates and a Prim or Kruskal implementation.  

Simulate an infection model in a dense social graph where edge weights represent probability of infection. 

