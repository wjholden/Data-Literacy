# Dimensionality

## Combinatorics

Suppose one has four children, $\left\{ a, b, c, d \right\}$, and a motorcycle.
The motorcycle can carry only one passenger, so there are four possible *combinations* of children that you can transport by motorcycle:

$$
4 = \left| \left\{
\left\{ a \right\},
\left\{ b \right\},
\left\{ c \right\},
\left\{ d \right\}
\right\} \right|.
$$

$\left| S \right| = n$ gives the *cardinality* (the size) $n$ of set $S$.

The family adds a sidecar to the motorcycle and can now transport two children at once.
There are now six ways that one can *choose* two elements from a four-element set:

$$
6 = \left|
    \left\{ a, b \right\},
    \left\{ a, c \right\},
    \left\{ a, d \right\},
    \left\{ b, c \right\},
    \left\{ b, d \right\},
    \left\{ c, d \right\}
    \right|.
$$

Recall from section \ref{section:discrete-math} that sets are unordered; $\left\{ a, b \right\}$ is equal to $\left\{ b, a \right\}$.

Two common notations for the number of possible subsets we can choose are $\binom{n}{r}$  and $nCr$.
The former is favored in higher mathematics, the latter in secondary schools.
$\binom{n}{r}$ is read "$n$ choose $r$" and $nCr$ is read "$n$ combinations taken $r$ at a time."

The family purchases a small car that can transport three passengers:

$$
\binom{4}{3} = 4 = 
\left|
    \left\{ a, b, c \right\},
    \left\{ a, b, d \right\},
    \left\{ a, c, d \right\},
    \left\{ b, c, d \right\}
\right|.
$$

The family purchases a larger car that can carry four passengers:

$$
\binom{4}{4} = 1 = \left| \left\{ a, b, c, d \right\} \right|.
$$

Finally, the family crashes the large car and is left with a bicycle.
The bicycle has no capacity to carry passengers, and therefore

$$
\binom{4}{0}  = 1 = \left| \left\{ \right\} \right| =  \left| \emptyset \right|.
$$

There is only one way to take an empty set from another set.

We will now construct a generalized function to count the number of subsets for any combination of $r$ elements taken from a set of size $n$.
Initially, consider the first element in the set.
If we choose this element, then we still to select $r-1$ elements from the remaining $n-1$ elements.
If we do not choose this element, then we still must choose $r$ elements from the remaining $n-1$ elements.
This gives us *Pascal's formula*, a *recursive* definition for counting combinations.

$$
\binom{n}{r} = \binom{n-1}{r-1} + \binom{n-1}{r-1}
$$

We need at least one *base case* to prevent this function from entering an infinite loop.
These identities should be intuitive from the earlier exercise, though the proof for the final case is left as an exercise to the reader.

$$
\binom{n}{r}
\begin{cases}
1, & \text{if $n = r$}. \\
1, & \text{if $r = 0$}. \\
n, & \text{if $r = 1$}. \\
0, & \text{if $n < r$}. \\
\end{cases}
$$

Implemented in the R language (https://webr.r-wasm.org/latest/), 

```
pascal <- function(n,r) {
  if (n < r) {
    return(0)
  } else if (n == r) {
    return(1)
  } else if (r == 0) {
    return(1)
  } else if (r == 1) {
    return(n)
  } else {
    return(pascal(n-1,r) + pascal(n-1,r-1))
  }
}
```

we can reproduce the results of our passengers example. The `sapply` function in R is comparable to the `map` operation (see section \ref{section:filter-map-reduce}).

```
> sapply(0:4, function(r) pascal(4, r))
[1] 1 4 6 4 1
```

## Event spaces

## Pareto fronts

## Covariance

## Discussion prompts

## Practical exericses

1. Use a nested `sapply` statement to improve `sapply(0:4, function(r) pascal(4, r))` to iterate `pascal(n, r)` over $0 \le n \le 10$ and $0 \le r \le n$. Compare the result to `sapply(0:10, function(n) choose(n, 0:n))`. Why does the built-in `choose` function accept ranges (`0:n`) when our own `pascal` function does not?
