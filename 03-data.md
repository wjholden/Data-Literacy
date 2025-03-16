# Data Operations

## The Relational Algebra

Codd's *relational algebra* is the framework theory describing all modern *database management systems* (DBMS) [@codd1970relational].
The relational algebra can be described with five primitives: *selection* ($\sigma$), *projection* ($\pi$), the *Cartesian product* ($\times$; also known as the *cross product*), set *union* ($\cup$), and set *difference* ($-$).

Selection takes all or a subset of a table's rows.
Projection takes all or a subset of a table's columns.
In Structured Query Languages (SQL), selection is specified in the `WHERE` clause and projection is specified in the list of columns immediately after `SELECT`.

A Cartesian product is the multiplication of sets.
If $A = \left\{ i, j \right\}$ and $B = \left\{ x, y, z \right\}$,
then $A \times B = \left\{
\left( i, x \right),
\left( i, y \right),
\left( i, z \right),
\left( j, x \right),
\left( j, y \right),
\left( j, z \right) \right\}$.
The Cartesian product produces the set of all possible pairwise combinations of elements in each set.
These composite values are called *tuples*.
Tuples may contain more than two values.
If $C = \left\{ c \right\}$, then

$$
A \times B \times C = \left\{
\left( i, x , c\right),
\left( i, y , c \right),
\left( i, z , c\right),
\left( j, x , c \right),
\left( j, y , c \right),
\left( j, z , c \right) \right\}.
$$

As an exercise, go to https://sqlime.org/#deta:mb9f8wq2mq0b to use a DBMS named SQLite.
Enter the following commands to reproduce the above Cartesian product.

```sql
CREATE TABLE A (a text);
CREATE TABLE B (b text);
CREATE TABLE C (c text);

INSERT INTO A(a) VALUES ('i'), ('j');
INSERT INTO B(b) VALUES ('x'), ('y'), ('z');
INSERT INTO C(c) VALUES ('c');

SELECT * FROM A CROSS JOIN B CROSS JOIN C;
```

This text views tuples as unordered and "flattened" sets, and therefore Cartesian products are both *commutative* ($R \times S = S \times R$) and *associative* ($R \times \left( S \times T \right) = \left( R \times S \right) \times T$).
Some mathematical texts use a stricter definition for the Cartesian product where the result is a set, which does not "flatten" and therefore provides neither commutivity nor associativity.
This text uses the looser definition for compatibility with practical DBMSs, including SQLite.
Mathematics is partly discovered and partly invented.

Set union, $\cup$, combines two sets.
Sets definitionally contain only distinct elements.
If $A = \left\{ i, j, k \right\}$ and $B = \left\{ k, l, m \right\}$, then

$$
A \cup B = \left\{ i, j, k, l, m \right\}.
$$

Set difference, $-$, retains the elements of the left set that are not present in the right set.

$$
A - B = \left\{ i, j, k \right\} - \left\{ k, l, m \right\} = \left\{ i, j \right\}.
$$

## Joining Tables

The *join* ($\bowtie$) is a combination of the Cartesian product and selection.
For example, suppose we have a tables named `Swim`, `Bike`, and `Run`.
Each table has a column that uniquely identifies an athlete.
To get a triathletes (the athletes who participate in swimming, cycling, and running),
we use an *equijoin* to find the product where the names are equal.
Return to https://sqlime.org/#deta:36fadcq9apak to demonstrate experiment with the `JOIN` operator.

```sql
CREATE TABLE IF NOT EXISTS Swim (sn TEXT UNIQUE);
CREATE TABLE IF NOT EXISTS Bike (bn TEXT UNIQUE);
CREATE TABLE IF NOT EXISTS Run (rn TEXT UNIQUE);

INSERT OR IGNORE INTO Swim (sn) VALUES
    ('John'), ('Jane'), ('Luke'), ('Phil');
INSERT OR IGNORE INTO Bike (bn) VALUES
    ('Mary'), ('Alex'), ('Jane'), ('Levi');
INSERT OR IGNORE INTO Run (rn) VALUES
    ('Mike'), ('John'), ('Jane'), ('Sven');

SELECT * FROM Swim, Bike, Run WHERE sn = bn AND sn = rn;
```

There are other syntaxes which achieve the same result using the `ON` and `USING` clauses.
As an exercise, try to predict how many rows will return from `SELECT * FROM Swim, Bike, Run` without a `WHERE` clause.

## Grouping and Aggregation {#section:grouping-and-aggregation}

DBMSs provide robust *grouping* functions for operating on related rows.
Return to https://sqlime.org/#deta:32lpfoo57r8g and create a small table of hypothetical marathon times.

```sql
CREATE TABLE IF NOT EXISTS Marathon (rn TEXT UNIQUE,
  time INTEGER,
  gender TEXT CHECK( gender IN ('M', 'F') ));

INSERT OR IGNORE INTO Marathon (rn, time, gender) VALUES
  ('Kyle', 2*60*60 + 14*60 + 22, 'M'),
  ('Hank', 2*60*60 + 10*60 + 45, 'M'),
  ('Lily', 2*60*60 + 24*60 + 47, 'F'),
  ('Emma', 2*60*60 + 22*60 + 37, 'F'),
  ('Elle', 2*60*60 + 25*60 + 16, 'F'),
  ('Fred', 2*60*60 + 6*60 + 17, 'M');

SELECT MIN(time) FROM Marathon GROUP BY (gender);
```

`MIN` is one of the *aggregate functions* in SQLite.
The `GROUP BY` clause tells the DBMS to split the rows into groups on the `gender` column.

One might be tempted to find the names of our male and female champions with
`SELECT rn, MIN(time) FROM Marathon GROUP BY (gender)`.
This may work in some DBMSs but there is a subtle bug.
It might be obvious that we want the `rn` associated with the `MIN(time)` value, but suppose we change the query to also include `MAX(time)`:

```
SELECT rn, MIN(time), MAX(time) FROM Marathon GROUP BY (gender);
```

Now it is no longer clear which `rn` the query should return.
Should the DBMS return the `rn` associated with the `MIN(time)`, the `MAX(time)`, or some other `rn` from the group?

The solution in this particular case is to nest our `MIN(time)` aggregation as a *subquery*.

```sql
SELECT * FROM Marathon
  WHERE time IN (
    SELECT MIN(time) FROM Marathon GROUP BY (gender));
```

SQL uses the *declarative programming* paradigm, where the language is used to
describe the *result* that the user^[In this context, the "user" is a programmer
or data analyst who is "using" the database or programming language] wants while
leaving the implementation details to the DBMS. Systems designed for declarative
programming often excel in situations that the developer intended but sometimes
struggle when the user needs something unusual. For situations where the user
needs to specify the detailed process to compute the result, we use the
*imperative programming* paradigm. Two specific imperative approaches are
*functional* and *object-oriented* programming. In practice, the distinctions
are often blurred by languages and databases that provide functionality from all
three.

## Functional Programming {#section:filter-map-reduce}

SQL syntax makes it easy to write select, project, and join (SPJ) queries.
SQL's grouping and aggregate functions make it possible to perform row-wise and column-wise operations.
One can find comparable semantics (with different syntax) in many programming language's *filter*, *map*, and *reduce* functions.

### Filter

Filter works much like the `WHERE` clause: it takes a subset of the rows, based off of a condition.
In JavaScript, we might filter an array with:

```javascript
>> ['cat', 'dog', 'fish', 'bird'].filter(v => v.includes('i'))
<- ['fish', 'bird']
```

### Map

Map performs the same function over each element of an input set, creating "mappings" to elements of an output set.

```javascript
>> ['fish', 'bird'].map(v => v.toUpperCase())
<- ['FISH', 'BIRD']
```

### Reduce

Reduce, also known as *fold*, performs some operation on each element of an input set and returns an *accumulator*, which is passed again to the reduce function with the next input value.
To take an array's sum, we use an initial accumulator value of 0.

```javascript
>> 15 + 25 + 35
<- 75
>> [15,25,35].reduce((a, v) => a + v, 0)
<- 75
```

For the array's product, we use 1 for the initial accumulator value.

```javascript
>> 15 * 25 * 35
<- 13125
>> [15,25,35].reduce((a, v) => a * v, 1)
<- 13125
```

Both filter and map can be implemented in terms of reduce.

```javascript
>> ['cat', 'dog', 'fish', 'bird'].reduce((a,v) => {
        if (v.includes('i')) {
            a.push(v);
        }
        return a;
    }, [])
<- ['fish', 'bird']
>> ['fish', 'bird'].reduce((a,v) => {
        a.push(v.toUpperCase());
        return a;
    }, [])
<- ['FISH', 'BIRD']
```

Here, we use an empty array (`[]`) instead of a numeric identity as our initial accumulator value.

### Vectorized Functions and Array Programming {#sec:array-programming}

A *vectorized function* automatically iterates over array inputs.
This approach is sometimes called *array programming*.
This design is less common in traditional languages (C, Java, JavaScript) and more common in scientific programming (R, Matlab, Julia).
Some examples in the R language, which one can reproduce at https://webr.r-wasm.org/latest/, are:

```r
> c(1, 2, 3) + 4
[1] 5 6 7
> c(1, 2, 3) + c(4, 5, 6)
[1] 5 7 9
> sqrt(c(1, 4, 9))
[1] 1 2 3
```

Observe that the pairwise sums in `c(1, 2, 3) + c(4, 5, 6)` are independent.
No sum depends on another, and therefore the computing machine can safely perform each operation in *parallel*.

### Immutability

Suppose one needs to write a program to sort its input. An obvious solution is
to order the inputs directly by *mutating* (changing) the memory in-place.
An alternative approach is to copy the input, order the copy, and return the ordered copy.

The Julia language provides both: Julia's \texttt{sort!\@} function mutates its input,
while the `sort` function returns a sorted copy, leaving the input unmodified.

The latter approach obviously uses more memory and will likely be slower.
Why would one use this approach? **Safety**. If a function "owning" a variable
passes an *immutable* (read-only) reference to another function, then the caller can
safely reason about the value and state of that variable after the callee returns.

Some languages provide stronger concepts of ownership and immutability than others.
The Rust language provides extensive memory safety features [@back-to-the-building-blocks]
by requiring the `mut` keyword to explicitly mark variables and function parameters mutable.
Traditionally, languages assumed the opposite and required `const` or `final`
keywords to establish invariants (with varying levels of enforcement; Java
programmers might be surprised that the `final` keyword does not make an object
read-only, but only the *reference to* an object).

Immutability is particularly useful for *thread-safety* in concurrent programming,
which we will discuss in section \ref{sec:parallelism-and-concurrency}.

## Object-Oriented Programming {#sec:oop}

Object-Oriented Programming (OOP) is a technique for modeling both the *data*
and associated *code* for a problem together [@10.1145/947955.947961] [@10.5555/3271463].
The data of an object are called *fields* and the code are called *methods*.
Many programming languages, notably C++, Python, JavaScript, and Python, emphasize OOP as the central design.

Object-orientation comes in many varieties [@10.5555/3271463]. Many OO languages
provide a method inherit data and code from other objects, often in a hierarchy.
The following Rust program, which one can run at https://play.rust-lang.org,
demonstrates a `Point` object. The object is defined as a `struct`
with two fields, `x` and `y`. The implementation for `Point` adds two methods,
a *constructor* (`new`) and a *Manhattan distance* function.

```rust
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn manhattan_distance(&self, other: &Self) -> f32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn main() {
    let a = Point::new(2.0, 4.0);
    let b = Point::new(-2.0, 3.0);
    println!("Distance: {}", a.manhattan_distance(&b));
}
```

## Parallelism and Concurrency {#sec:parallelism-and-concurrency}

*Parallelism* is the ability for a computing machine to perform simulataneous operations.
Two tasks are *concurrent* if their order does not matter.
Getting dressed in the morning is an example (see figure \ref{fig:get-dressed}).
When one dons their pants, shirt, coat, hat, socks, and shoes, one must don socks before shoes, but the order in
which one dons shoes and their hat does not. The hat and shoes are concurrent but the socks and shoes are *sequential*.
In practice, many programmers confuse the terms *parallel* and *concurrent* as interchangable.

Concurrent programming can be challenging because one *process* or *thread* (sometimes called *task* or *routine*) might interfere with another,
but performance benefits often justify the additional complexity.

![Order of operation partially matters when getting dressed. Some clothing items are sequential, but others are concurrent. The system can be modeled as a directed ayclic graph (see section \ref{section:special-cases-of-graphs}).](get-dressed.dot.pdf){#fig:get-dressed}

Some problems can be partitioned into *subproblems* which can be solved in parallel.
Other problems cannot.
Some encryption algorithms intentionally *chain* the output from one block into the next.
One cannot calculate block $n$ without first calculating block $n-1$, and $n-2$, and so on.
The reduce operation applies to this algorithm design.

Other problems can be effortlessly partitioned into subproblems and solved quickly with a *divide-and-conquer* approach.
A trivial example might be finding the minimum value in a large dataset.
One can partition the dataset, find the minimum value in each partition, and then find the minimum value among those results.
This process can be repeated.

Go to https://go.dev/play/p/IOwH08R_z7Z to experiment with a divide-and-conquer `minimum` function in the Go language.

```go
package main

import "fmt"

func min(x, y int) int {
	if x <= y {
		return x
	}
	return y
}

func minimum(x []int) int {
	fmt.Println(x)
	n := len(x)
	switch n {
	case 1:
		return x[0]
	case 2:
		return min(x[0], x[1])
	default:
		middle := n / 2
		lower := minimum(x[:middle])
		upper := minimum(x[middle:])
		return min(lower, upper)
	}
}

func main() {
	fmt.Println(minimum([]int{610, 144, 34, 21, 2584, 55, 55}))
}
```

Click the "Run" button several times and observe that the output is completely *deterministic*.
Now go to https://go.dev/play/p/Vbe7BWrwlku for a slightly modified version of the same program.

```go
	default:
		middle := n / 2
		lower := make(chan int)
		upper := make(chan int)
		go func() { lower <- minimum(x[:middle]) }()
		go func() { upper <- minimum(x[middle:]) }()
		return min(<-lower, <-upper)
	}
```

This version constructs two *channels* for communication among concurrent tasks.
We use the `go` keyword to create two *Goroutines* (threads in the Go language), which concurrently solve the `minimum` function over subproblems.
Finally, we read the results from each channel with `<-lower` and `<-upper` and return.
Click the "Run" button several times and observe that the final result is consistent, but the order of operations is not.

The computer industry has recently turned to *Graphical Processing Units* (GPU) as a fast, inexpensive, and energy-efficient method for solving highly parallelizable problems.
GPUs were originally designed to draw computer graphics, which extensively use matrix and vector multiplication.
These linear transformations can be performed in parallel and GPU makers designed their products to perform many simple calculations in parallel.

## Cumulative Sums and Pareto Charts

A *Pareto chart* is a useful analytical tool to show the relative importance of
problems in industrial settings. The chart shows the proportion of problems
discretized (see section \ref{section:discretize}) into root causes. We can
compute these cumulative sums using reduce and visualize them with a bar plot.

The following example uses data gathered from \url{https://games.crossfit.com/article/complete-list-athletes-currently-serving-sanctions}.
The `%>%` operator, from the `dplyr` package, anonymously "pipes" the output
from one function into the first argument of the next function.
Structurally, the `%>%` produces a left-to-right order of operations that
can be easier to write, read, and maintain than functions written in prefix and
infix notation. `dplyr` uses `mutate` as row-wise `map` operation with support
for aggregate functions (such as `sum(n)` below;
see also section \ref{section:grouping-and-aggregation}).

```r
> library(tidyverse)
> v = c("GW1516", "GW1516", "Methenolone", "Meldonium", "GW1516", 
  "GW1516", "Oxandrolene", "GW1516", "GW1516", "Clomiphene", "Clomiphene",
  "GW1516", "Turinabol", "GW1516", "GW1516", "GW1516", "RAD140", "GW1516",
  "GW1516", "Stanozolol", "Drostanolone", "GW1516", "Clomiphene",
  "GW1516", "GW1516", "Ostarine", "S-23", "GW1516", "Clomiphene", "GW1516",
  "Meldonium", "GW1516", "GW1516", "5aAdiol", "Stanozolol", "Testosterone", 
  "Drostanolone", "GW1516", "GW1516", "Metenolone", "GW1516", "Boldenone", 
  "GW1516", "GW1516", "GW1516")
> df = data.frame(Violation = v) %>%
  count(Violation) %>%
  arrange(-n) %>%
  mutate(Proportion = 100.0 * n / sum(n)) %>%
  select(Violation, Proportion) %>%
  mutate(CumSum = cumsum(Proportion))
> df
      Violation Proportion    CumSum
1        GW1516  55.555556  55.55556
2    Clomiphene   8.888889  64.44444
3  Drostanolone   4.444444  68.88889
4     Meldonium   4.444444  73.33333
5    Stanozolol   4.444444  77.77778
6       5aAdiol   2.222222  80.00000
7     Boldenone   2.222222  82.22222
8    Metenolone   2.222222  84.44444
9   Methenolone   2.222222  86.66667
10     Ostarine   2.222222  88.88889
11  Oxandrolene   2.222222  91.11111
12       RAD140   2.222222  93.33333
13         S-23   2.222222  95.55556
14 Testosterone   2.222222  97.77778
15    Turinabol   2.222222 100.00000
```

This dataset does not quite show the famous "Pareto principle" where 20% of 
problems cause 80% of problems, but we do see that the distribution is not
uniform. The first category accounts for over half of the observations.
Using R's `ggplot` library, we show the resulting Pareto chart with the bars
and cumulative sum line.

```r
df %>% ggplot(aes(x = reorder(Violation, -Proportion))) +
	geom_bar(aes(weight = Proportion)) +
	geom_line(aes(y = CumSum, group=1)) +
	xlab("Drug Violation") +
	ylab("Proportion")
```

![A Pareto chart shows the relative and cumulative proportions of discretized quantities, sorted in decreasing incidence. Frequently used in quality control processes, such as Lean Six Sigma, Pareto charts may show that only one or a few causes lead to a significant proportion of problems.](pareto-chart.pdf)

## The CAP Theorem

Brewer's *CAP theorem* states that a *distributed system* has at most two qualities of *consistency*, *availability*, and *partition-tolerance* [@6133253].
Consider a system of databases with many replicas.
The replicas are consistent if they contain perfect copies of the database,
and they are available only they are writable.
The distributed system is partition-tolerant if all replicas remain identical, but this is impossible if one allows writes that cannot propagate into the other partition.

The CAP theorem has many practical implications on data integrity and should be considered in design methodology.
One must anticipate server and network outages that would create a partition in the distributed in the system and then choose the desired behavior.
Can we accept lost database writes when we reconcile after a partition is restored?
Or should be accept service outages in order to protect the integrity of the database during an interruption?

A partial solution is to weaken our definition of each quality.
Perhaps a system reserves certain rows or columns that are only writable by a specific database, guaranteeing that there will be no conflict if this database continues to write to those changes during a partition.
A system might establish some form of confidence intervals in certain data, such as the position of a tracked aircraft with error margins, in recognition that imperfect information might still be useful.
Finally, a system might use a quorum model (i.e., 3 of 5 available nodes) to preserve partial availability in the majority partition.

## Discussion Prompts

1. How does the CAP theorem impact intelligence and fires in relation to the command and control (C2) warfighting function (WfF)? 

2. Where should unclassified data be stored and processed? 

3. What are some methods to prevent conflicts among concurrent writes in a shared database? 

4. What could possibly go wrong when altering database schema? 

## Practical Exercises

1. Create a custom list in SharePoint that provides multiple views showing grouped and aggregated values. 

2. Given a noisy dataset, identify problems in each column that could influence inclusion and exclusion criteria. 

3. Implement filter and map in terms of reduce using a programming language which provides reduce. 

4. Define an “embarrassingly parallel” problem and provide both examples and counterexamples. 
