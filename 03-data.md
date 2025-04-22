# Data Operations

## Prose

In this chapter, we will discuss several practical matters for operating on data
to extract useful information, but first we should quickly discuss the mechanics
of mixing data, code, plots, mathematical notation, and tables, and prose.

Context is king. One must consider the target audience when writing reports from
any analysis.

Presentations, such as with Microsoft PowerPoint, are useful as
visual aids to speeches. The slides themselves should contain mainly plots,
sparse text, and simple tables to summarize information. Slides are a poor medium
for presenting raw data, large tables, code, and long passages of prose. Pity
statements, organized as bullet points, can be useful for both the speaker and
the audience, but full sentences are often not recommended.

Papers are favorable to slides for deep analysis. Papers vary in length. Summaries
are short and generally seek to report conclusions without detailed evidence.
A senior leader may accept the conclusions presented in an employee's report
based on trust in the person, not the persuasiveness of the analysis itself.
White papers provide enough evidence to be persuasive on their own merit, although
white papers may not provide detailed listings of the data and code used.

The sciences use *notebooks* as a means of presenting prose with in-line code,
plots, mathematical notation, and tables. Some examples of notebook interfaces
are Jupyter^[<https://jupyter.org>] (commonly used with Python, R,
Julia, and Scala), Mathematica^[<https://www.wolfram.com/mathematica/>] (the
Wolfram Language), and RStudio^[<https://posit.co/products/open-source/rstudio/>]
(R and Python). Notebook interfaces support *literate programming*, a practice
of writing code with an emphasis on human understanding above computer
interpretation [@10.1093/comjnl/27.2.97].

<!-- <https://www.sec.gov/Archives/edgar/data/1018724/000119312518121161/d456916dex991.htm> -->
<!-- <https://www.inc.com/justin-bariso/amazon-jeff-bezos-powerpoint-meetings-how-to-think.html> -->
Written prose and spoken presentation are key to aggregating and processing data
into information and then interpreting information into knowledge. Jeff Bezos
famously insists upon the use of six-page narratives at Amazon in favor of
PowerPoint^[<https://www.youtube.com/watch?v=L227qFemjqI>]. The rest of this
chapter will focus on technical matters of working with data, but look at how
data, code, figures, and mathematical notation are presented throughout. Reflect
upon how these may or may not be appropriate when writing and presenting
information, depending on format.

## Computability

A joke in computer science says that "you can write C in any language." The joke
is literally true. Assuming adequate resources (compute time, memory, storage,
and access to necessary inputs and outputs), one could implement a C interpreter
in any *Turing-compute* language [@michaelson2020programming, p. 4:13 -- 4:17] 
and execute any C program. Such an endeavor is not theoretical: *virtual machines*
and related technologies simulate and emulate entire computing machines, allowing
programs to run on systems that they were not designed for.

\begin{figure}[h]
\centering
\includegraphics[width=1.0\textwidth]{low-high-level-languages.tikz}
\label{fig:low-high-level-languages}
\end{figure}

Programming languages are imprecisely categorized as *low-level* and *high-level*.
One should view these terms as a spectrum, not dichotomies. Low-level languages
generally require more explicit specification to the machine, allowing for
greater control of the computation and often faster. The abstractions available
in high-level languages often allow the programmer to code with syntax closer to
mathematical notation. Computing in spoken language (or "natural language") has
historically failed to satisfy the high expectations popularized in science
fiction [@EWD:EWD667], although recent advances in AI have steadily improved
machines' ability to compute results from spoken or written prompts.

*Low code* platforms, such as Microsoft's
Power BI^[<https://www.microsoft.com/en-us/power-platform/products/power-bi>],
seek to democratize programming to non-developers [@bock2021low]. Low code 
platforms provide functions to process data into information with prepared
analytical and visualization features, generally using minimal programming
languages or even visual programming.

Low code platforms can excel at tasks that they were designed for but complicate
novel tasks. This tension mirrors the conflict of *narrow* and *general* AI
systems. Users trust AI systems, such as Apple's
Siri^[<https://www.apple.com/siri/>], to correctly respond to very specific tasks
("hey Siri, what's the weather tomorrow?") but do not expect these systems to
generalize to vague or contextual queries ("hey Siri, recommend a movie for me").

AI systems with natural language input will no doubt continue to advance in the
coming years, but we will always need to understand the general methods used to
structure, process, transfer, and store our data in novel situations. Where no
one has previously described an algorithm to solve new problems, we will likely
always turn to code as a reification of our mathematical ideas.

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

As an exercise, go to the SQLime Playground^[<https://sqlime.org/#deta:mb9f8wq2mq0b>] to use a DBMS named SQLite.
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
Some mathematical texts use a stricter definition for the Cartesian product where the result is a set, which does not "flatten" and therefore provides neither commutativity nor associativity.
This text uses the looser definition for compatibility with practical DBMSs, including SQLite.
(Mathematics is partly discovered and partly invented.)

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
Return to the SQLime Playground^[<https://sqlime.org/#deta:36fadcq9apak>] to demonstrate experiment with the `JOIN` operator.

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

## Grouping and Aggregation {#sec:grouping-and-aggregation}

DBMSs provide robust *grouping* functions for operating on related rows.
Return to the SQLime Playground^[<https://sqlime.org/#deta:32lpfoo57r8g>] and create a small table of hypothetical marathon times.

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

Taking aggregates from aggregates can produce different statistics from those
of the original data set. Consider the election of choices $A$ and $B$ by 100
voters as shown in figure \ref{fig:simpson-votes}. In elections, the winner may 
lose the popular vote, as aggregated district votes do not reflect the density
within their groups. Aggregation is generally a *lossy* process, where the
inputs cannot be reconstructed from the information it produces [@cai2019data].

\begin{figure}
\centering
\includegraphics{simpson-votes.tikz}
\caption{This plot shows 100 votes grouped into 10 committees. If each
committee is given only one vote, then decision $A$ will receive more committee
votes than decision $B$, having lost the *popular vote* with only 36 votes.}
\label{fig:simpson-votes}
\end{figure}

The apparent reversal of votes in figure \ref{fig:simpson-votes} is related to
*Simpson's Paradox* [@10.1111/j.2517-6161.1951.tb00088.x]. TODO: say more about
this.

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

## Functional Programming {#sec:filter-map-reduce}

SQL's declarative syntax makes it easy to write select, project, and join (SPJ) queries.
SQL grouping and aggregate functions make it possible to perform row-wise and column-wise operations.
A *functional* programming language, which emphasizes the use of pure functions
(see section \ref{sec:discrete-math}) to express algorithms, provides comparable
semantics in the *filter*, *map*, and *reduce* functions.

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

### Reduce {#sec:reduce}

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

Both filter and reduce can be implemented in terms of reduce.

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

In both cases, we use an empty array (`[]`) instead of a numeric identity as our initial accumulator value.

<!-- No bueno on the ~ character in math mode. -->
\newcommand{\infix}{\char"007E}

<!-- <https://neopythonic.blogspot.com/2019/03/why-operators-are-useful.html?m=1> -->
<!-- <https://reference.wolfram.com/language/ref/Infix.html.en> -->
Some languages differentiate `foldl` and `foldr` to differentiate left- and right-associativity.
A left-associative function would evaluate $x \infix y \infix z$ with first $x \infix y$ and then $(x \infix y) \infix z$.
(In this context, the "$\infix$" represents an arbitrary infix operator and has no specific meaning).
A right-associative function evaluates $x \char"007E y \infix z$ as $x \infix (y \infix z)$.

### Vectorized Functions and Array Programming {#sec:array-programming}

A *vectorized function* automatically iterates over array inputs.
This approach is related to *array programming*.
Automatic vectorization is less common in traditional languages (C, Java, JavaScript) and more common in scientific programming (R, Matlab, Julia).
Some examples in the R language, which one can reproduce at <https://webr.r-wasm.org/latest/>, are:

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

### Immutability {#sec:immutability}

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
The following Rust program, which one can run at the Rust Playground^[<https://play.rust-lang.org/?gist=9542264fd12645a4ee1956ab7f890812>],
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

## JavaScript Object Notation (JSON) {#sec:json}

We introduced CSV in section \ref{sec:csv} as a method for representing data in
a file. *JavaScript Object Notation* (JSON) is an alternative format [@rfc8259].
JSON's syntax is based on JavaScript. Objects in JSON are key-value pairs. The
key of a JSON object must be a double-quoted string. Values can be nested objects,
arrays, numbers, strings, Booleans, and `null`. The process of taking a data
structure from a program's memory and saving it as JSON is called *serialization*.
The inverse, reading an object into memory from a JSON input, is correspondingly
*deserialization*. Trailing commas are forbidden.

JSON is much more verbose than CSV, but less verbose than the Extensible
Markup Language (XML), which we will not discuss further. JSON is generally
"human-readable" and can be authored by hand, although not as easily as CSV.
The following code listing shows the table from section \ref{sec:csv} as JSON.

```json
[
    {
        "x": "Rob",
        "y": 0.74019382956651820,
        "z": 0.3508759018489489
    },
    {
        "x": "John",
        "y": 0.41331428270607506,
        "z": 0.2936926427452584
    },
    {
        "x": "David",
        "y": 0.37671743737357277,
        "z": 0.5676190157838865
    },
    {
        "x": "Frank",
        "y": 0.50270122376380740,
        "z": 0.7939268929144455
    }
]
```

JSON documents allow arbitrarily nested and shaped objects, but in many applications
it may be undesirable to deserialize records with missing values. Consider if
one of the below records were missing a $y$-value, or if a $z$ value were incorrectly
enclosed in double-quotes, thus forming a string instead of a numeral.

Some JSON parsers, such as Rust's Serde^[<https://serde.rs>], allow the programmer to specify
the structure of the record before parsing. Libraries may ignore or error when records
do not fit the expected shape. One can expect statically-typed languages to
require more specification before parsing and dynamically-typed languages to allow
greater flexibility at runtime (see section \ref{sec:strong-weak-types}).

## Parallelism and Concurrency {#sec:parallelism-and-concurrency}

*Parallelism* is the ability for a computing machine to perform simultaneous operations.
Two tasks are *concurrent* if their order does not matter.
Getting dressed in the morning is an example (see figure \ref{fig:get-dressed}).
When one dons their pants, shirt, coat, hat, socks, and shoes, one must don socks before shoes, but the order in
which one dons shoes and their hat does not. The hat and shoes are concurrent but the socks and shoes are *sequential*.
In practice, many programmers confuse the terms *parallel* and *concurrent* as interchangeable.

Concurrent programming can be challenging because one *process* or *thread* (sometimes called *task* or *routine*) might interfere with another,
but performance benefits often justify the additional complexity.

![Order of operation partially matters when getting dressed. Some clothing items are sequential, but others are concurrent. The system can be modeled as a directed ayclic graph (see section \ref{sec:special-cases-of-graphs}).](get-dressed.dot.pdf){#fig:get-dressed}

Some problems can be partitioned into *subproblems* which can be solved in parallel.
Other problems cannot.
Some encryption algorithms intentionally *chain* the output from one block into the next.
One cannot calculate block $n$ without first calculating block $n-1$, and $n-2$, and so on.
The reduce operation applies to this algorithm design.

Other problems can be effortlessly partitioned into subproblems and solved quickly with a *divide-and-conquer* approach.
A trivial example might be finding the minimum value in a large dataset.
One can partition the dataset, find the minimum value in each partition, and then find the minimum value among those results.
This process can be repeated.

Go to the Go Playground^[<https://go.dev/play/p/IOwH08R_z7Z>] to experiment with a divide-and-conquer `minimum` function in the Go language.

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
Now return to the Go Playground^[<https://go.dev/play/p/Vbe7BWrwlku>] for a slightly modified version of the same program.

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

#. The Excel function `VLOOKUP(lookup_value, table_array, col_index_num, range_lookup)`
searches in a table (`table_array`) for a value (`lookup_value`) and returns the
value in the numbered column (`col_index_num`)^[<https://support.microsoft.com/en-us/office/vlookup-function-0bbc8083-26fe-4963-8ab8-93a18ad188a1>].
If `range_lookup` is true, then `VLOOKUP` uses approximate matching, otherwise
exact. Assuming one corrects the SQL syntax, what is the *semantic* difference
between `VLOOKUP(x, y, z, FALSE)` and the SQL query `SELECT z FROM y WHERE x`?
Can we parameterize the SQL statement to produce the same result as `VLOOKUP`?

#. How does the CAP theorem impact intelligence and fires in relation to the command and control (C2) warfighting function (WfF)? 

#. Where should unclassified data be stored and processed? 

#. What are some methods to prevent conflicts among concurrent writes in a shared database? 

#. What can go wrong when altering database schema? 

## Practical Exercises

#. Create a custom list in SharePoint that provides multiple views showing grouped and aggregated values. 

#. Given a noisy dataset, identify problems in each column that could influence inclusion and exclusion criteria. 

#. Define an “embarrassingly parallel” problem and provide both examples and counterexamples. 

#. In section \ref{sec:reduce} we have examples of the filter and map operations
implemented in terms of reduce. Later, in our discussion of immutability in section
\ref{sec:immutability}, we learned that a sorting function can either mutate the
data in-place or copy the data, leaving the original data unchanged and returning
a new data structure with the desired property. Which design are the filter and
map operations in section \ref{sec:reduce}? Rewrite both functions in the other
paradigm.

#. The *flatten* operation promotes elements of nested collections to a single
container.
    A flawed example in Julia is

    ```julia
    julia> reduce(∪, [[:a,:b], [:c,:a], [:d,:a,:b]])
    4-element Vector{Symbol}:
     :a
     :b
     :c
     :d
    ```

    This one-line solution is a *shallow* flatten. It fails on doubly-nested
    inputs.

    ```julia
    julia> reduce(∪, [[:a,:b], [:c,:a], [:d,:a,:b], [[:e]]])
    5-element Vector{Any}:
     :a
     :b
     :c
     :d
     [:e]
    ```

    Implement a *deep* flatten that correctly traverses arbitrarily nested
    inputs. Test that the deep flatten operation correctly handles empty inputs,
    nested empty inputs (such as `[[[[]]]]`), and duplicates.

<!--
# This program has a bug!
# deep_union!([], [[1,2], [3,1], [4,1,2], [[5]]])               # works fine!
# deep_union!([], [[:a,:b], [:c,:a], [:d,:a,:b], [[:e]]])       # method error!
function deep_union!(dst, src)
    for e in src
        if e isa AbstractSet || e isa Array
            deep_union!(dst, e)
        else
            union!(dst, e)
        end
    end
    dst
end
-->
       
