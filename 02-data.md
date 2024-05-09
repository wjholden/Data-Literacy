# Data Operations

## Forms and input validation

## Relational algebra

Codd's *relational algebra* is the framework theory describing all modern *database management systems* (DBMS).
The relational algebra can be described with five primitives: *selection* ($\sigma$), *projection* ($\pi$), the *Cartesian product* ($\times$; also known as the *cross product*), set *union* ($\cup$), and set *difference* ($\setminus$).

Selection takes all or a subset of a table's rows.
Projection takes all or a subset of a table's columns.
For example, suppose a table's schema is defined as `CREATE TABLE WeightliftingMeet (Athlete TEXT, Lift TEXT, Mass REAL, Good BOOLEAN)`.
The query `SELECT Athlete FROM WeightliftingMeet WHERE Mass >= 100 AND Good == TRUE` performs both a selection (specified in the `WHERE` clause) and a projection (the columns specified immediately after `SELECT`, in this case `ATHLETE`).

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

As an exercise, go to https://sqlime.org to use a DBMS named SQLite.
Enter the following commands to reproduce the above Cartesian product.

```
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

Set difference, $\setminus$, retains the elements of the left set that are not present in the right set.

$$
A \setminus B = \left\{ i, j, k \right\} \setminus \left\{ k, l, m \right\} = \left\{ i, j \right\}.
$$

## Join

The *join* ($\bowtie$) is a combination of the Cartesian product and selection.
For example, suppose we have a tables named `Swim`, `Bike`, and `Run`.
Each table has a column that uniquely identifies an athlete.
To get a triathletes (the athletes who participate in swimming, cycling, and running),
we use an *equijoin* to find the product where the names are equal.
Return to https://sqlime.org to demonstrate experiment with the `JOIN` operator.

```
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

## Grouping and aggregation

DBMSs provide robust *grouping* functions for operating on related rows.
Return to https://sqlime.org and create a small table of hypothetical marathon times.

```
CREATE TABLE Marathon (rn TEXT UNIQUE, time INTEGER,
  gender TEXT CHECK( gender IN ('M', 'F') ));

INSERT INTO Marathon (rn, time, gender) VALUES
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

```
SELECT * FROM Marathon
  WHERE time IN (
    SELECT MIN(time) FROM Marathon GROUP BY (gender));
```

## Filter, map, and reduce

SQL syntax makes it easy to write select, project, and join (SPJ) queries.
SQL's grouping and aggregate functions make it possible to perform row-wise and column-wise operations.
One can find comparable semantics (with different syntax) in many programming language's *filter*, *map*, and *reduce* functions.

Filter works much like the `WHERE` clause: it takes a subset of the rows, based off of a condition.
In JavaScript, we might filter an array with:

```
>> ['cat', 'dog', 'fish', 'bird'].filter(v => v.includes('i'))
<- ['fish', 'bird']
```

Map performs the same function over each element of an input set, creating "mappings" to elements of an output set.

```
>> ['fish', 'bird'].map(v => v.toUpperCase())
<- ['FISH', 'BIRD']
```

Reduce, also known as *fold*, performs some operation on each element of an input set and returns an *accumulator*, which is passed again to the reduce function with the next input value.
To take an array's sum, we use an initial accumulator value of 0.

```
>> 15 + 25 + 35
<- 75
>> [15,25,35].reduce((a, v) => a + v, 0)
<- 75
```

For the array's product, we use 1 for the initial accumulator value.

```
>> 15 * 25 * 35
<- 13125
>> [15,25,35].reduce((a, v) => a * v, 1)
<- 13125
```

Both filter and map can be implemented in terms of reduce.

```
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

## Vectorized functions

A *vectorized function* automatically iterates over array inputs.
This design is less common in traditional languages (C, Java, JavaScript) and more common in scientific programming (R, Matlab, Julia).
Some examples in the R language, which one can reproduce at https://webr.r-wasm.org/latest/, are:

```
> c(1, 2, 3) + 4
[1] 5 6 7
> c(1, 2, 3) + c(4, 5, 6)
[1] 5 7 9
> sqrt(c(1, 4, 9))
[1] 1 2 3
```

Observe that the pairwise sums in `c(1, 2, 3) + c(4, 5, 6)` are independent.
No sum depends on another, and therefore the computing machine can safely perform each operation in *parallel*.

*Concurrency* is the ability for a computing machine to perform simulataneous operations.
Concurrent programming can be challenging because one *process* or *thread* (sometimes called *task* or *routine*) might interfere with another,
but performance benefits often justify the additional complexity.

# Concurrency

## Consistency, availability, and partition-tolerance (CAP) theorem 

## Discussion prompts

1. How does the CAP theorem impact intelligence and fires in relation to the command and control (C2) warfighting function (WfF)? 

2. Where should unclassified data be stored and processed? 

3. What are some methods to prevent conflicts among concurrent writes in a shared database? 

4. What could possibly go wrong when altering database schema? 

## Practical exercises

1. Create a custom list in SharePoint that provides multiple views showing grouped and aggregated values. 

2. Given a noisy dataset, identify problems in each column that could influence inclusion and exclusion criteria. 

3. Implement filter and map in terms of reduce using a programming language which provides reduce. 

4. Define an “embarrassingly parallel” problem and provide both examples and counterexamples. 
