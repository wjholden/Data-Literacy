# Data Operations

## Forms and input validation

## Relational algebra

Codd's *relational algebra* is the framework theory describing all modern *database management systems* (DBMS).
The relational algebra can be described with five primitives: *selection* ($\sigma$), *projection* ($\pi$), the *Cartesian product* ($\times$), set *union* ($\cup$), and set *difference* ($\setminus$).

Selection takes all or a subset of a table's rows.
Projection takes all or a subset of a table's columns.
For example, suppose a table's schema is defined as `CREATE TABLE WeightliftingMeet (Athlete TEXT, Lift TEXT, Mass REAL, Good BOOLEAN)`.
The query `SELECT Athlete FROM WeightliftingMeet WHERE Mass >= 100 AND Good == TRUE` performs both a selection (specified in the `WHERE` clause) and a projection (the columns specified immediately after `SELECT`, in this case `ATHLETE`).

A Cartesian product is the multiplication of sets.
Let $A = \left\{ i, j, k \right\}$ and $B = \left\{ x, y, z \right\}$.
Then $A \times B = \left\{
\left( i, x \right),
\left( i, y \right),
\left( i, z \right),
\left( j, x \right),
\left( j, y \right),
\left( j, z \right),
\left( k, x \right)
\left( k, y \right)
\left( k, z \right) \right\}.

## Filter, map, and reduce 

## Grouping and aggregation 

## Vectorized functions 

## Concurrency 

## Consistency, availability, and partition-tolerance (CAP) theorem 

## Discussion prompts

How does the CAP theorem impact intelligence and fires in relation to the command and control (C2) warfighting function (WfF)? 

Where should unclassified data be stored and processed? 

What are some methods to prevent conflicts among concurrent writes in a shared database? 

What could possibly go wrong when altering database schema? 

## Practical exercises

Create a custom list in SharePoint that provides multiple views showing grouped and aggregated values. 

Given a noisy dataset, identify problems in each column that could influence inclusion and exclusion criteria. 

Implement filter and map in terms of reduce using a programming language which provides reduce. 

Define an “embarrassingly parallel” problem and provide both examples and counterexamples. 

