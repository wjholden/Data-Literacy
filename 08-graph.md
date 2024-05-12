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
WHERE 
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

Graphs are modeled using either an *adjacency list* or *adjacency matrix*.
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


## Dijkstra’s algorithm

## A*

## Computational complexity and Big-[Equation] 

## Power Law Distribution 

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

