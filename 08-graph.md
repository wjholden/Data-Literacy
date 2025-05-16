# Graph Theory {#chapter:graph}

## Vertices, edges, and paths

A *graph* ($G$) is a collection of *vertices* ($V$; also known as *nodes* or *points*) and the *edges*
($E$; also known as *relations* or *lines*; see section \ref{sec:discrete-math}) connecting them.

$$
G = \left\{ V , E \right\}
$$

Edges are conventionally named for the vertices they connect.
For example, let $V = \left\{ u , v \right\}$ and $E = \left\{ \left( u, v \right) \right\}$,
then $G = \left\{ V, E \right\}$ is a graph with two vertices, $u$ and $v$, and one edge, $(u, v)$.

Graphs can be used to model any form of *network* where the elements of sets bear relations.
Graphs can be directed, where the source and destination vertices in an edge are significant ($(u, v) \ne (v, u)$),
or undirected ($(u,v) = (v,u)$).
The number of edges associated with a vertex is its *degree*.
In directed graphs, we distinguish *in-degree* (the number of edges leading into the vertex) and *out-degree* (the number of edges originating from the vertex).

![Graphs are conventionally visualized as circles (vertices) connected by lines (edges).](paris-brussels-hague.dot.pdf)

A *path* is a series of edges that *transitively* connect two vertices that are not directly connected.
We say that $u \leadsto v$ ($u$ leads to $v$) if the graph contains some path from $u$ to $v$.

The edges of a graph may have a *distance function* ($\delta$; also known as *weight* and *cost*), which relates each edge with some real number.
Such graphs are *weighted graphs*.
For example, suppose a high-speed railroad has train stations in Paris, Brussels, and the Hague.
The distance from Paris to Brussels is about $\qty{350}\km$ and the distance from Brussels to Hague is another $\qty{180}\km$.
The total path length from Paris to Hague via Brussels is therefore $\qty{350}\km + \qty{180}\km = \qty{530}\km$.

$$
\begin{aligned}
\delta \left( \text{Paris} , \text{Hague} \right) &= \delta \left( \text{Paris} , \text{Brussels} \right) + \delta \left( \text{Brussels} , \text{Hague} \right) \\
&= \qty{350}\km + \qty{180}\km \\
&= \qty{530}\km.
\end{aligned}
$$

Let us quickly reproduce this result using a *graph database* named Neo4j.
Go to <https://console.neo4j.org> and click the "Clear DB" button.
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
Refer to section \ref{sec:filter-map-reduce} for a description of the reduce operation.

```
MATCH (:CITY {name:'Paris'})-[p:ROUTE*]->(:CITY {name:'Hague'}) 
RETURN p, REDUCE(length=0, e IN p | length + e.dist) AS distance;
```

## Connectivity and distance

A *complete* graph is *fully connected*, having paths between all pairs of vertices.
One's model for distance may be challenged in incomplete graphs, which contain partitions.

What is the driving distance from Paris, France to Sydney, Australia?
There is no route (no path) connecting Europe, by ground, to Australia, and therefore
there is no real number to quantify the distance.

Depending on the application, one might represent an unreachable node as having infinite distance,
as we will in section \ref{sec:dijkstra} with Dijkstra's algorithm.
One might also use a special, non-numeric values, as described in our discussion
of missing values in section \ref{sec:nan}.

## Special cases of graphs {#sec:special-cases-of-graphs}

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

![A directed acyclic graph (DAG) may contain no cycles. The left graph is a DAG. The right graph is not because the edges connecting $a$, $b$, and $c$ form a loop.](dag.dot.pdf)

The edges $\left( a, b \right)$, $\left( b, c \right)$, and $\left( c, a \right)$ form a cycle.
Vertices $a$, $b$, and $c$ together form a *connected component*.
A directed graph with one or more connected components is not a DAG, but the graph of components
(where vertices of the component subgraph are merged into a "super vertex") is itself a DAG.

A *tree* is another special case of an acyclic graph.
Trees are often drawn in a vertical hierarchy where each *child node* has one *parent*, and the only parent node with no parent is called the *root node*.
One's ancestral family tree is an instance of a tree.
Without a time machine, it is impossible to one to form a hereditary loop with an ancestor.

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
E = \begin{bNiceMatrix}[first-row,first-col]
& \text{Paris} & \text{Brussels} & \text{Hague} \\
\text{Paris} & 0 & 350 & \infty \\
\text{Brussels} & 350 & 0 & 180 \\
\text{Hague} & \infty & 180 & 0
\end{bNiceMatrix}.
$$

In this example, the distance of a vertex to itself is defined as zero,

$$
\delta \left( u, u \right) = 0
$$

and the distance between vertices is considered infinite if those vertices are not directly connected by an edge.

$$
\left(u , v\right) \notin E \implies \delta \left( u, v \right) = \infty
$$

The $\in$ operator and its negation, $\notin$, tests whether an object is an "element of" a set; $\in$ is read "in" and $\notin$ is read "not in."
The symbol $\implies$ is for *conditional implication* and is read "implies."
If the *statement* on the left of $\implies$ is true, then the statement on the right must also be true.

## Search algorithms

### Depth-first search

Imagine a video game where the player searches the kingdom for treasure.
The player has no knowledge of where the treasure might be, so from a starting point they fully explore the forest, mountains, and sea.
Both the starting point and the sea connect to opposite sides of the city.
Upon entering the city from the sea-side, the player explores the city and discovers the treasure.
This is an example of a *depth-first search* (DFS).

![There are many paths from the starting point to the treasure in this kingdom.](kingdom.dot.pdf)

Go to the Go Playground^[<https://go.dev/play/p/AuH2qOgSG-c>] to run the following DFS implementation, written in Go.
This implementation uses a *recursive* definition of the DFS function (the DFS function invokes itself as it explores the graph).
The function uses an external data structure (`quest`) to identify which vertices have already been discovered.
Upon successful search, the function prints its position in the graph as the recursive calls "unwind."

```go
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

### Breadth-first search {#sec:bfs}

Imagine the protagonist of our hypothetical adventure game was not a lone wanderer, but rather a field marshal commanding a large army.
This army explores one region at a time, holding each area as adjacent units proceed into their respective area.
The army incrementally expands the radius of the search *frontier* in a search technique called *breadth-first search* (BFS).
Once one unit discovers the treasure, we are certain that no shorter path was possible thanks to an *invariant* in our search algorithm.

Maintaining an invariant is essential for *mathematical induction*, where we establish that some *predicate* $P$ is true for the *base case* $P(0)$ and that $P(k)$ implies $P(k+1)$ and therefore $P(n)$ is true for all $n > 0$.
The proof for the correctness of a BFS follows:

#. Along the frontier of radius $r=0$, the BFS algorithm on graph $G$ has not discovered a path from $u$ to $v$. $\delta(u,v)$ is therefore at *closest* $r=1$.
#. At $r=1$, BFS has not found $v$ and therefore $2 \le \delta(u,v)$.
#. At $r=2$, BFS has not found $v$ and therefore $3 \le \delta(u,v)$.
#. $\vdots$
#. At $r=k$, BFS has not found $v$ and therefore $k+1 \le \delta(u,v)$.
#. $\vdots$
#. At $r=n$, BFS has located $v$ and therefore $n = \delta(u,v)$. $\square$

Go to the Go Playground^[<https://go.dev/play/p/yMIcmcsK_V9>] and run the following BFS implementation, written in Go.
This implementation uses an *iterative* BFS function.
The BFS function does not invoke itself.
Instead, the procedure adds unexplored vertices to a queue and records the "parent" of each vertex.
We "unwind" the resulting tree from child to parent nodes to construct the shortest path.

```go
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
In the following section, we will find that we can often explore graphs much faster by ordering our breadth-first traversal by cumulative path cost.

### Dijkstra's algorithm {#sec:dijkstra}

<!--
Dijkstra in Go <https://go.dev/play/p/hcNbTvGli-z/>
Too long and ugly to share.
-->

*Dijkstra's algorithm* uses a *priority queue* to visit nodes from shortest to longest path [@10.1145/3544585.3544600].
For this reason, Dijkstra's algorithm is also known as the *shortest-path first* (SPF).
Like BFS, Dikjstra's algorithm is a *greedy algorithm* that discovered a globally optimal solution by repeatedly making locally optimal decisions.
Let us turn to the Python language to demonstrate Dijkstra's algorithm on our same treasure-hunting graph, this time with edge weights.
Run this program at <https://www.python.org/shell/>.^[Python *requires* tab characters
where other languages might accept spaces and tabs interchangably. Copying this
program from a PDF will likely not work.
A plain text verion of this program is available at <https://github.com/wjholden/Data-Literacy/blob/main/dijkstra.py>.]

```python
from heapq import *
from collections import defaultdict

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
    distance = defaultdict(lambda: float('inf'))
    previous = {'start': None}
    distance[src] = 0
    queue = []
    heappush(queue, (0, src))
    while queue:
        _, current = heappop(queue)
        if current == dst:
            path = []
            parent = current
            while parent in previous:
                path.append(parent)
                parent = previous[parent]
            return path, distance[dst]
        if current in explored:
            continue
        explored.add(current)
        for neighbor in g[current]:
            d = distance[current] + w[(current, neighbor)]
            if neighbor not in explored and d < distance[neighbor]:
                distance[neighbor] = d
                previous[neighbor] = current
                heappush(queue, (d, neighbor))
    print("No path found")

path, distance = dijkstra("start", "treasure")
print("Path =", ', '.join(path))
print("Distance =", distance)
```

This program should output `Path found: 193`, and `Path: treasure castle city start`.
The shortest path from `start` to `treasure` has a total path cost of 193.

Neo4j produces the same result.
We reconstruct our graph in the Cypher language at <https://console.neo4j.org>:

```sql
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

```sql
MATCH
    (src:LOCATION {name:'start'}),
    (dst:LOCATION {name:'treasure'}),
    path = shortestPath((src)-[:CONN*]->(dst))
RETURN
    path,
    REDUCE(d=0, e in relationships(path) | d + e.distance)
    AS distance;
```

Neo4j should also report a distance of 193.

As an exercise, change `->(dst)` to `-(dst)` and re-run the query with this change.
The total distance is now 145.
The difference in `()-[]->()` and `()-[]-()` is that one is a directed edge, the other is undirected.
With `-()` instead of `->()`, Neo4j treats all edges in the graph as undirected.
Neo4j allows *parallel edges* that connect the same two vertices.

### Informed search with A*

DFS, BFS, and Dijkstra's algorithms are all *uninformed* search algorithms.
The A* algorithm is an *informed* search algorithm: it uses some *heuristic* to explore its frontier
ordered by minimum estimated distance to the destination [@4082128].

A* can solve hard problems that are not immediately recognizable as searches.
The canonical example is the 8-piece puzzle problem [@aima, p. 111--117].
An 8-piece puzzle arranges 8 movable square tiles into a $3 \times 3$ grid.
One position is empty.
The challenge is to slide the pieces until all pieces are in order.

$$
\begin{bmatrix}
5 & 4 & 1\\
& 2 & 8\\
3 & 6 & 7
\end{bmatrix}
\to
\begin{bmatrix}
1 & 2 & 3 \\
4 & 5 & 6 \\
7 & 8 & 
\end{bmatrix}
$$

The choice of heuristic function influences the path A* chooses as it searches for a solution.
One heuristic function for the 8-piece puzzle problem returns the count of mismatched pieces.
If two pieces are out of place then the heuristic distance is 2,
if three pieces are out of place then the heuristic distance is 3,
and so on.
An alternative, more sophisticated heuristic function, uses the *Manhattan distance* (also known as *Taxicab distance*) of each puzzle piece to its destination.
Manhattan distance is the sum of unsigned differences of each dimension between two coordinates in $n$-dimensional space.
The intuition is that a taxicab cannot fly in a straight line, but rather has to corner the rectangular blocks of Manhattan.

$$
d_T \left( p, q \right) = \sum_{i=i}^n{ \left| p_i - q_i \right| }
$$

For example, if $p = (x_1, y_1)$ and $q = (x_2, y_2)$, then $d_T(p,q) = \left| x_1 - x_2 \right| + \left| y_1 - y_2 \right|$.
A heuristic function for an A* solution to the 8-piece puzzle problem uses Manhattan distance to quantify the closeness of the current game state to the desired solution.
This approach works because progress *towards* the solution ultimately results in a global solution.
The design of the heuristic function is key to the informed search technique.

Some problems contain *local extrema* (local but not global minimal or maximal values) that might stop the search at a suboptimal solution.
If the problem has an extremely large space (there are too many candidate solutions to search exhaustively),
then it may be acceptable to accept a "good-enough" local best candidate solution as an approximation for the global optimum.
The *local search* technique uses an *ensemble* of *search agents* which independently search *neighborhoods* of the problem space.
A local search could be built upon A* searches from different origins.
Ideally, the A*-based local search should explore the problem with reasonable depth, mobility, and coverage [@schuurmans2001local].

Interestingly, the 8-piece and comparable 15-piece puzzle problems are not guaranteed to be solvable.
Exactly half of all possible $9! = \num{362880}$ and $15! = \num{1307674368000}$ permutations are reachable from any random 8- and 15-piece puzzle game state [@10.2307/2369492].
$n$-piece puzzles ($n \ge 3$) contain two partitions and are therefore classified as *bipartite* graphs.
The puzzle is unsolvable if started in the partition that does not contain the solution.
We must understand that search algorithms might not be able to solve a problem if the graph of the problem space contains a partition.

### A* and the Stable Marriage Problem {#sec:stable-marriage}

<!-- original research -->

We will demonstrate the A* informed search algorithm on the *Stable Marriage Problem* [@gale1962college].
The Stable Marriage Problem seeks to pair the members of two equal-sized sets to one another based upon their mutual preferences.
This problem and its solution are applied to many practical situations, including the Army Talent Alignment Process (ATAP);
see <https://www.youtube.com/watch?v=9mEBe7fzrmI> for an official and detailed explanation.
Explained with marriages, the problem has all of the men rank all of the women from most to least preferred.
Correspondingly, the women also rank all of the men from most to least preferred.

$$
\begin{aligned}
M &= \begin{bNiceMatrix}[first-row,first-col]
& \text{Woman 1} & \text{Woman 2} & \text{Woman 3} \\ 
\text{Man 1} & 1 & 2 & 3 \\
\text{Man 2} & 3 & 1 & 2 \\
\text{Man 3} & 2 & 3 & 1
\end{bNiceMatrix}\\
W &= \begin{bNiceMatrix}[first-row,first-col]
& \text{Man 1} & \text{Man 2} & \text{Man 3} \\ 
\text{Woman 1} & 3 & 1 & 2 \\
\text{Woman 2} & 2 & 3 & 1 \\
\text{Woman 3} & 1 & 2 & 3
\end{bNiceMatrix}
\end{aligned}
$$

Each man is paired to one woman.
The set of matching is considered "stable" when there is no "*rogue couple*": where a man $x$ who prefers some other woman $y$ to his current wife *and* where woman $y$ prefers man $x$ to her current husband.
(The solution does not need to give everyone their first preference.
It is acceptable for some person to prefer someone other than their current spouse; instability occurs only when that person requites.)
In the above example, the pairing $\left[ 1, 2, 3 \right]$ (man 1 paired to woman 1, man 2 with woman 2, 3 with 3) is stable.
The pairing $\left[ 3, 2, 1 \right]$ (man 1 paired to woman 3, man 2 with woman 2, 3 with 1) is not stable because man 1 perfers woman 2 to his current spouse (woman 3) and woman 2 prefers man 1 to her current spouse (man 2).

The Stable Marriage Problem is known to be solvable with a simple and predictable algorithm by Gale and Shapley [@gale1962college].
First, each man proposes to his most-preferred woman.
Women who receive multiple proposals maintain only their most-preferred proposal and reject the others.
Second, each rejected man proposes to his next-preferred woman; women receiving new proposals continue to maintain only the one most-preferred.
The process continues until no man is rejected in a round.
Finally, the women accept their most-preferred proposal and the algorithm terminates.

A *reduction* is a method of solving one problem by restating it in terms of another.
Reductions can be a powerful but difficult technique for solving hard problems.
By reducing the stable marriage problem to a graph search problem, we can solve the problem with an A* algorithm.
Our A* solution will be slower and less efficient than the Gale-Shapley algorithm, but it will demonstrate a method for solving difficult problems using general and reusable techniques.
The 8-piece puzzle problem could likely also be solved with a very fast and direct algorithm, but it may be very difficult for us to discover this solution.
Likewise, there may be an optimal algorithm to solve a Rubik's cube, but given an A* solver and a fast computer we may be able to find an economically-acceptable solution.

Our informed search algorithm is implemented in Julia.
This is comparable our implementation of Dijkstra's algorithm (see section \ref{sec:dijkstra}),
but instead of searching for a named destination this function instead uses its heuristic function.
If the heuristic function returns the value zero, then the search is considered successful and the program terminates.
This A* program also prints its search in the DOT graph description language, which can be rendered as a graphic using the GraphViz program.

```julia
using DataStructures

function informed_search(source, edges::Function, heuristic::Function)
    println("digraph {")

    pq = PriorityQueue()
    visited = Set()
    enqueue!(pq, source=>heuristic(source))

    while !isempty(pq)
        u = dequeue!(pq)
        push!(visited, u)
        println("\"$(u)\" [color=\"blue\"];")
        
        if heuristic(u) == 0
            println("}")
            return
        end

        for v ∈ edges(u)
            if v ∉ visited && !haskey(pq, v)
                enqueue!(pq, v=>heuristic(v))
                println("\"$(u)\" -> \"$(v)\";")
            end
        end
    end

    error("Failed to find a solution.")
end
```

The heuristic function for the stable marriage function seeks to quantify and differentiate instability by returning the sum of the squared distance (see section \ref{sec:least-squares-method}) of a rogue couple's candidate and current preferences.

```julia
function stability(men::Matrix, women::Matrix, matching)
    n = length(matching)
    wife = matching
    husband = Dict(values(matching) .=> keys(matching))
    metric = 0

    for man ∈ 1:n
        for woman ∈ 1:n
            # Candidate and current preferences for the man
            x1, x2 = men[man, woman], men[man, wife[man]]
            # Candidate and current preferences for the woman
            y1, y2 = women[woman, man], women[woman, husband[woman]]
            # The matching is unstable if, and only if, both the man
            # and the woman prefer each other to their current matches.
            if x1 < x2 && y1 < y2
                metric += (x1 - x2)^2
                metric += (y1 - y2)^2
            end
        end
    end

    return metric
end
```

The size of our problem comes from the number of possible matchings.
If there are $n$ men and $n$ women, then the first man can be matched to $n$ women,
the second man can be matched to $n-1$ women, and so on until the last man can only be matched to the only remaining woman.
Thus, 

$$
\text{Problem size} = n \cdot (n-1) \cdot (n-2) \cdot \ldots \cdot 3 \cdot 2 \cdot 1 = n!
$$

A problem of factorial size is a large combinatorial problem.
We will not attempt to visit all possible nodes in the problem space.
Instead, our edge function will generate three permutations of the current position by switching two matchings.

```julia
using StatsBase

function e(u)
    v = Set()
    for _ ∈ 1:3
        x = copy(u)
        # Sample, without replacement, two random elements to swap.
        y = sample(collect(eachindex(u)), 2, replace=false)
        x[y[1]], x[y[2]] = x[y[2]], x[y[1]]
        push!(v, x)
    end
    return v
end
```

Our ranking matrices for men, $M$, and women, $W$ are taken from example 2 from Gale and Shapley [@gale1962college].

$$
M = \left[
\begin{array}{cccc}
1 & 2 & 3 & 4 \\
1 & 4 & 3 & 2 \\
2 & 1 & 3 & 4 \\
4 & 2 & 3 & 1 \\
\end{array}
\right]
$$

$$
W = \left[
\begin{array}{cccc}
3 & 4 & 2 & 1 \\
3 & 1 & 4 & 2 \\
2 & 3 & 4 & 1 \\
3 & 2 & 1 & 4 \\
\end{array}
\right]
$$

The Julia language has a compact notation to create matrix literals.

```julia
M = [1 2 3 4; 1 4 3 2; 2 1 3 4; 4 2 3 1]
W = [3 4 2 1; 3 1 4 2; 2 3 4 1; 3 2 1 4]
```

We construct a *closure* to encapsulate $M$ and $W$ with our stability metric function into the heuristic function.

```julia
h(u) = stability(M, W, u)
```

This syntax declares a unary function `h` that will enable our A* to navigate matrices $M$ and $W$ without direct knowledge of either.

Finally, we invoke the A* informed search algorithm to navigate the Stable Marriage Problem as a graph, starting from pairings $\left[ 4 , 3, 2, 1 \right]$ (man 1 matched to woman 4, man 2 to woman 3, 3 to 2, and 4 to 1).

```
julia> informed_search([4,3,2,1], e, h)
digraph {
"[4, 3, 2, 1]" [color="blue"];
"[4, 3, 2, 1]" -> "[4, 2, 3, 1]";
"[4, 3, 2, 1]" -> "[4, 1, 2, 3]";
"[4, 3, 2, 1]" -> "[4, 3, 1, 2]";
"[4, 3, 1, 2]" [color="blue"];
"[4, 3, 1, 2]" -> "[4, 2, 1, 3]";
"[4, 3, 1, 2]" -> "[4, 1, 3, 2]";
"[4, 2, 1, 3]" [color="blue"];
"[4, 2, 1, 3]" -> "[2, 4, 1, 3]";
"[4, 2, 1, 3]" -> "[1, 2, 4, 3]";
"[2, 4, 1, 3]" [color="blue"];
"[2, 4, 1, 3]" -> "[2, 1, 4, 3]";
"[2, 4, 1, 3]" -> "[3, 4, 1, 2]";
"[3, 4, 1, 2]" [color="blue"];
}
```

(Note: this *stochastic* algorithm uses randomness in the `sample` operation.
Outputs are not deterministic.
In rare cases, this procedure may not discover the one and only solution, $\left[ 3, 4, 1, 2 \right]$.
See <https://github.com/wjholden/Data-Literacy/blob/main/StableMarriageSearch.jl> for an expanded version of this program which uses a seeded random number generator for reproducibility.)

We can input this `digraph` data into <https://dreampuf.github.io/GraphvizOnline/> to visualize the search tree, as shown in figure \ref{fig:gale-shapley-ex2}.

![A search tree of the Stable Marriage Problem, reduced to an informed search that is solvable with A*.](gale-shapley-ex2.dot.pdf){#fig:gale-shapley-ex2}

Again, we applied informed search to the Stable Marriage Problem as an exercise in artificial intelligence methods.
Though slow, an informed search can navigate difficult problems with very little information: a simple heuristic function tells A* whether it has gotten closer or farther from the solution.
This technique can be useful for solving challenging problems where an optimal solution is not known.
Moreover, we can also apply informed search to *intractable* problems where computational complexity forces us to accept approximate solutions as a compromise.

## Centrality

### Degree

The average person (ten of twelve) in the social graph shown in figure \ref{fig:friends} has *fewer* friends the average friend count of their friends.
These statistics are summarized in the following table.

![Arithmetic in graphs can produce unintuitive results. In this graph, 10 of 12 vertices has a lower degree than the average among its adjacent nodes.](friends.dot.pdf){#fig:friends}

|Name | Friends | Average friend's friend count |
|-----|---------|-------------------------------|
| Frank | 2 | 5 |
| Jasmine | 2 | 5.5 |
| Kim | 7 | 3.14 |
| David | 3 | 3.67 |
| Sven | 2 | 5 |
| Felix | 2 | 5 |
| Jean | 2 | 5 |
| Ivan | 3 | 3.67 |
| Adin | 7 | 3.14 |
| Maria | 2 | 5.5 |
| Jose | 4 | 4.5 |
| Phil | 1 | 7 |

How can this be possible?
We must be careful with definitions.
The average number of friends in this graph is $3.08$, but we are not looking at the entire graph.
Instead, we are only looking at a subgraph.
Look closely at Phil.
Phil has only one friend, Adin, and Adin is highly-connected.
In fact, Adin has more friends than all of his friends, except Kim.

Highly-connected nodes, with outlier degree, can be particularly important in many graph applications.
Transportation networks are an example: congestion at any major airport "hub" can quickly spread to adjacent airports and beyond.

*Degree centrality* is a simple and intuitive graph statistic [@FREEMAN1978215].
Simply count the in-degree, out-degree, or both, and use this metric to discover important nodes in the graph.

### Closeness

*Closeness centrality*, $C_C$, is also fairly simple to compute from the sum of unweighted distances from each node to each other node
[@10.1121/1.1906679].
Closeness centrality has been used to study criminal and terrorist networks [@Krebs2001MappingNO].

$$
C_C(x) = \frac{1}{\sum_{y \in v}{\delta(x,y)}}
$$

The following Rust program uses an unweighted invocation of Dijkstra's algorithm
in the `petgraph` crate^[The term *crate* is peculiar to Rust.
In this context, the term is interchangeable with "library,"
which is a more common programming term.]. Run this program at the Rust
Playground^[<https://play.rust-lang.org/?gist=e67ef3bc05daab21dd73a7869093d9cb>].

```rust
use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use petgraph::visit::NodeRef;
use petgraph::Graph;

fn main() {
    let mut graph: Graph<(), (), Undirected> = Graph::new_undirected();
    let a = graph.add_node(());
    let b = graph.add_node(());
    let c = graph.add_node(());
    let d = graph.add_node(());
    let e = graph.add_node(());
    graph.extend_with_edges(&[(a, b), (b, c), (c, d), (d, e)]);
    let graph = graph;
    closeness(&graph);
}

fn closeness<N, E>(graph: &Graph<N, E, Undirected>) {
    for u in graph.node_indices() {
        let delta = dijkstra(&graph, u.id(), None, |_| 1);
        let n = graph.node_count() as f64;
        let distances = delta.values().cloned().sum::<i32>() as f64;
        let closeness = (n - 1.0) / distances;
        println!(
            "Closeness score for vertex {} is {}.",
            u.index(),
            closeness
        );
    }
}
```

The output of this program should be

```
Closeness score for vertex 0 is 0.4.
Closeness score for vertex 1 is 0.5714285714285714.
Closeness score for vertex 2 is 0.6666666666666666.
Closeness score for vertex 3 is 0.5714285714285714.
Closeness score for vertex 4 is 0.4.
```

As an exercise, draw the graph described in this program (either by hand or
using software) and assess if the centrality statistics seem intuitive. 

### Betweenness

*Betweenness centrality*, $C_B$, seeks to improve upon closeness by considering edge
weights [@10.2307/3033543].
The betweenness for a vertex $x$ is calculated the proportion of shortest paths
$\sigma \left( s, t \right)$ that include $x$ as an intermediary node along
the path.

$$
C_B(x) = \sum_{s \ne x, x \ne t, s \ne t}{
\frac{
    \sigma \left( s, t | x \right)
}{
    \sigma \left( s, t \right)
}
}
$$

The computational complexity of so many pathfindings becomes significant in large
graphs and dense graphs [@10.1080/0022250X.2001.9990249] [@brandes2007centrality].

### PageRank

<!-- <http://infolab.stanford.edu/~backrub/google.html> -->

Google's well-known *PageRank* algorithm uses a creative edge weighting function
based on a page's importance or quality [@BRIN1998107].
The metric is parameterized with a dampening factor, $d$, and requires multiple
iterations.

$$
\text{PR} \left( u \right) = \sum_{v \in B_u}{\frac{\text{PR} \left( v \right)}{L \left( v \right)}}
$$

This peculiar concept of a page's "reputation" is central to the efficacy of the
Google search engine.

There are many more measures of centrality in graphs beyond degree centrality,
closeness, betweenness, and PageRank. Depending on the application, any or none
of these metrics may be useful. The analyst studying a graph might consider
these metrics "man-made"; they were created by scientists and mathematicians
seeking to model a problem. These metrics can be customized to the user's needs
to best approach their unique problem domain.

<!-- We'll just have to try this again for the next edition.

## Power Law distribution

todo [@10.1126/science.286.5439.509]

## Minimum Spanning Tree

todo

## NP-completeness

todo 
-->

## Probability Spaces {#sec:graph-probability}

Probability questions are notoriously unintuitive. Using a graph, we can visualize
and understand probability spaces by plotting each possible event. Consider the
following example: a basketball player scores $1/3$ of 3-point attempts. What is
the probability that they score exactly once after two throws?

We can model the first throw as 3, 0, and 0: there is a $1/3$ chance that they
score, and $2 \times 1/3$ chance that they score zero.

The second throw creates three branches from the first three outcomes (assuming
independence, which we infer from the problem formulation). If the player
scores, then the outcomes are 6, 3, and 3. If player misses, then the outcomes
are 3, 0, and 0 and another 3, 0, and 0. In total, we have four outcomes of 3,
and therefore the probability that the player scores exactly once is $4/9$.

As shown in figure \ref{fig:graph-probability}, the event space is easy to
understand when modeled as a graph. We can verify this result using the binomial
theorem from section \ref{sec:binomial} as

$$
\begin{aligned}
p(1) &= \binom{2}{1} \left( \frac{1}{3} \right)^1 \left( 1 - \frac{1}{3} \right)^{ \left( 2 - 1 \right) } \\
&= (2) \left( \frac{1}{3} \right) \left( \frac{2}{3} \right) \\
&= \frac{4}{9}
\end{aligned}
$$

and using R's `dbinom` function as `dbinom(x=1, size=2, prob=1/3)`.

\begin{figure}[t]
\centering
\includegraphics{fig/graph-probability.tikz}
\caption{Discrete probabilities can be modeled as a graph. In this graph, each
row represents the 3-point throw of a basketball with $1/3$ chance of scoring.}
\label{fig:graph-probability}
\end{figure}

## Discussion prompts

#. A graph can be represented with an adjacency list or a matrix. What are the advantages and disadvantages of each approach? 

#. What algorithm can be used to solve the “seven ways to Kevin Bacon” problem? 

#. Is a Gantt chart a graph? How can one find the critical path of a project if represented as a graph? 

#. If the distance from Paris to Sydney is infinitely far, then can we use some 
*greater infinity* to represent the distance from London to Sydney?

#. Think of a practical problem that can be modeled as a graph, but where the
four discussed measures of center (degree centrality, closeness, betweenness,
and PageRank) are not effective. As a discussion point, consider whether values 
immediately associated with vertices and edges dominate their importance, or
if some extrinsic network effect has a greater effect.

#. A manufacturer sells systems that are made of components, as conceptualized
in figure \ref{fig:part-component-system}. Those components are assembled from
atomic parts. Many parts are interchangeable with other parts, and many
components are interchangeable with other components. How can the manufacturer
discover unused or duplicative parts and components?

    \begin{figure}
    \centering
    \includegraphics[width=1.0\textwidth]{fig/part-component-system.tikz}
    \label{fig:part-component-system}
    \caption{A \textit{dependency graph} models implication relationships in systems.}
    \end{figure}

## Practical exercises

<!-- Compare two different heuristic functions in a provided A* informed search implementation on the 8-piece puzzle problem. -->

#. Convert currency exchange rates from multiplication to addition using a logarithm, then prove that infinite arbitration is impossible given a set of exchange rates and Bellman-Ford implementation. 

#. Define a topological sorting and relate it to a workplace problem. 

#. Define the Traveling Salesman Problem (TSP) and explain the computational difficulty of this problem. 

#. Determine the minimum paving needed to fully connect a tent complex using a list of coordinates and a Prim or Kruskal implementation.  

#. Simulate an infection model in a dense social graph where edge weights represent probability of infection. 

#. Use the technique shown in section \ref{sec:graph-probability} to solve the Monty
Hall problem.
