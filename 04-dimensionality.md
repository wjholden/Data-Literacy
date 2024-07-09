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
\binom{n}{r} = 
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

## Permutations

An alternative definition for the combination formula requires *permutations* -- ordered subsets.
From set $S = \left\{ a, b, c, d \right\}$ there are twelve two-element permutations, represented here as *tuples*:
$(a,b)$, $(b,a)$, $(a,c)$, $(c,a)$, $(a,d)$, $(d,a)$, $(b,c)$, $(c, b)$, $(b, d)$, $(d, b)$, $(c,d)$, and $(d,c)$.

When counting the size of the permutation set of length $r$ chosen from a set of size $n$, we begin wtih $n$ possible elements for the first tuple element, then $n-1$ possible elements for the second tuple element, and so on until all $r$ tuple elements are filled.

$$
nPr = n \times (n-1) \times (n-2) \times \ldots \times (n-r+1) = \frac{n!}{(n-r)!}
$$

The *permutation formula* is usually defined using the *factorial* function, denoted by the "$!$" postfix operator.

$$
n! = n \times (n-1) \times (n-2) \times \ldots \times 2 \times 1 = \prod_{i=1}^{n}{i}
$$

$0!=1$ by definition.
The intuition for this is the bicycle: there was one way to choose an empty set from a set, and likewise there is one empty tuple of zero ordered elements taken from a set.

The number of $r=n$-length permutations of a set of size $n$ is simply

$$
nPn = \frac{n!}{(n-n)!} = \frac{n!}{0!} = \frac{n!}{1} = n!
$$

Now we can define the combination formula in terms of the permutation formula.
We count the number of permutations but de-duplicate this count, as combinations are unordered.
The number of duplicated entries is $rPr = r!$.

$$
nCr = \frac{nPr}{r!} = \frac{\frac{n!}{\left(n-r\right)!}}{r!} = \frac{n!}{r!\left(n-r\right)!}
$$

## The curse of combinatorics

The "curse of combinatorics" is simply that the number of possible combinations can become very large.
Consider a bicycle factory that must manufacture a part in four materials (steel, aluminum, carbon fiber, and titanium),
three sizes (small, medium, and large), five styles (road, mountain, touring, utility, and economy),
and for five markets (North America, European Union, Latin America, East Asia, and Middle East) which each have different compliance requirements.
There are $4 \times 3 \times 5 \times 5 = 300$ distinct variations of this part.
Suppose a distributor wants to warehouse 50 of each part, but expects the factory to wait until the part is sold before receiving payment.
Should the factory give the the distributor $300 \times 50 = \num{15000}$ of this part?

Now suppose an investor wants a rigorous test of the bicycle factory's products.
The investor demands that 30 copies of each part be tested in various ways.
$300 \times 30 = \num{9000}$ total parts being committed to this study might be unrealistic.

As a different example, imagine one wanted to conduct a large study on exercise and health outcomes.
Basic demographic variables include age, gender, location, height, weight, and race.
Exercise variables in this study include weekly minutes performing cardiovascular training, minutes of resistance training, and minutes of flexibility training.
Other exercise variables in this study include metrics of speed, endurance, strength, and flexibility, blood pressure, resting heart rate, body composition.


## Event spaces

## Pareto fronts

## Covariance

## Discussion prompts

## Practical exericses

1. Use a nested `sapply` statement to improve `sapply(0:4, function(r) pascal(4, r))` to iterate `pascal(n, r)` over $0 \le n \le 10$ and $0 \le r \le n$. Compare the result to `sapply(0:10, function(n) choose(n, 0:n))`. Why does the built-in `choose` function accept ranges (`0:n`) when our own `pascal` function does not?
