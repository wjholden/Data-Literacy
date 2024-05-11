# Graph Theory {#chapter:graph}

## Vertices and edges

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
For example, suppose a high-speed railroad has train stations in Paris, Brussels, and Amsterdam.
The distance from Paris to Brussels is about $\qty{350}\km$ and the distance from Brussels to Amsterdam is another $\qty{250}\km$.
The total length of the *path* from Paris to Amsterdam via Brussels is therefore $\qty{350}\km + \qty{250}\km = \qty{600}\km$.

$$
\begin{aligned}
\delta \left( \text{Paris} , \text{Amsterdam} \right) &= \delta \left( \text{Paris} , \text{Brussels} \right) + \delta \left( \text{Brussels} , \text{Amsterdam} \right) \\
&= \qty{350}\km + \qty{250}\km \\
&= \qty{600}\km.
\end{aligned}
$$

## Directed and undirected graphs 

## Directed acyclic graphs (DAG) and topological sorting 

## Weighted graphs 

## Breadth-first search (BFS) and depth-first search (DFS) 

## Dijkstra’s algorithm 

## Computational complexity and Big-[Equation] 

## Graph databases 

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

