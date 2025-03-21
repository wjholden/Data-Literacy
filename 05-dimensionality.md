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

Recall from section \ref{sec:discrete-math} that sets are unordered; $\left\{ a, b \right\}$ is equal to $\left\{ b, a \right\}$.

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

```r
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

we can reproduce the results of our passengers example. The `sapply` function in R is comparable to the `map` operation (see section \ref{sec:filter-map-reduce}).

```r
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
nCr = \binom{n}{r} = \frac{nPr}{r!} = \frac{\frac{n!}{\left(n-r\right)!}}{r!} = \frac{n!}{r!\left(n-r\right)!}
$$

## $n$ choose 2 {#sec:choose2}

The case $\binom{n}{2}$ occurs frequently and deserves special discussion.
The first few terms are (in Interactive Python, or IPython):

```python
In [1]: import math

In [2]: [math.comb(n, 2) for n in range(2,12)]
Out[2]: [1, 3, 6, 10, 15, 21, 28, 36, 45, 55]
```

It is not possible to choose two elements from a set of only one,
there is only one way to choose two from two,
three ways to choose two from three ($\left\{ a,b \right\}, \left\{ a,c \right\}, \left\{ b,c \right\} \subset \left\{ a,b,c \right\}$),
six ways to choose from four
($\left\{ a,b \right\}, \left\{ a,c \right\}, \left\{ a,d \right\}, \left\{ b,c \right\}, \left\{ b,d \right\}, \left\{ c,d \right\} \subset \left\{ a,b,c,d \right\}$),
and so on.
The resulting sequence of integers are called the *triangular numbers*.

Intuitively, the difference in $\binom{k+1}{2}$ and $\binom{k}{2}$ is $k$:
if we add a $(k+1)$th element to a set, then we can pair this new element with each of the $k$ existing elements.
The generalized form is

$$
\binom{n}{2} = 1 + 2 + 3 + \ldots + (n-1) = \frac{n(n-1)}{2}.
$$

We can demonstrate this identity numerically

```python
In [3]: [sum(k for k in range(n)) for n in range(2,12)]
Out[3]: [1, 3, 6, 10, 15, 21, 28, 36, 45, 55]

In [4]: [n*(n-1)//2 for n in range(2,12)]
Out[4]: [1, 3, 6, 10, 15, 21, 28, 36, 45, 55]
```

and prove with *induction*.
The basis of induction is the case $n=2$, where

$$
\binom{2}{2} = 1 = \frac{n(n-1)}{2} = \frac{2(2-1)}{2} = \frac{2(1)}{2} = 1.
$$

The inductive step is that if $\binom{k}{2} = \frac{k(k-1)}{2}$, then $\binom{k+1}{2} = \frac{(k+1)((k+1)-1)}{2}$.
Remembering that $\binom{k+1}{2} - \binom{k}{2} = k$, we find

$$
\begin{aligned}
\frac{(k+1)((k+1)-1)}{2} - \frac{k(k-1)}{2} &= \frac{(k+1)k}{2} - \frac{k(k-1)}{2} \\
&= \frac{k((k+1) - (k-1))}{2} \\
&= \frac{2k}{2} \\
&= k. \square
\end{aligned}
$$

An alternative proof is to use algebra from our definition $\binom{n}{r}=\frac{n!}{r!(n-r)!}$ as follows:

$$
\binom{n}{2} = \frac{n!}{2!(n-2)!} = \frac{(n)(n-1)(n-2)\ldots(3)(2)(1)}{(2)(n-2)(n-3)\ldots(3)(2)(1)} = \frac{(n)(n-1)}{2}.
$$

Yet another proof is to observe the series $1+2+3+\ldots+(n-1)+n$, cleverly reverse the series and add it to itself to form $(n+1)+((n-1)+2)+\ldots+(n+1)$, observe that there are $n$ of these identical terms and the original sum is half that of the second.
Though elegant, this proof technique may not as portable to other problems as computational, inductive, and algebraic methods.

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

## Subsets and Venn diagrams

A set intersection ($\cap$) of two sets is the set of all elements present in
both sets.

$$
S \cap T = \left\{
x |
x \in S 
\land
x \in T
\right\}.
$$

\begin{figure}
\centering
\begin{tikzpicture}
\draw[black] (0,0) rectangle (12,8);
\draw[black] (6,4) circle (2);
\node[text width=1] at (1,7) {$U$};
\node[text width=1] at (6,4) {$S$};
\end{tikzpicture}
\caption{A Venn diagram showing a single dimension, $S \subset U$.}
\label{fig:Venn1}
\end{figure}

The familiar Venn diagram is commonly used to plot set intersections, but this
plot is limited and is frequently misused. Traditionally, the square frame of
the plot represents the universal set, $U$. Each circle of the Venn diagram
shows a subset of $U$ along some binary attribute.
In figure \ref{fig:Venn1}, we see a degenerate Venn diagram of a single dimension.
Values of $U$ are simply in $S$ or not in $S$.

\begin{figure}
\centering
\begin{tikzpicture}
\draw[black] (0,0) rectangle (12,8);
\draw[black] (5,4) circle (2);
\draw[black] (7,4) circle (2);
\node[text width=1] at (1,7) {$U$};
\node[text width=1] at (4,4) {$S$};
\node[text width=1] at (8,4) {$T$};
\end{tikzpicture}
\caption{A Venn diagram showing two dimensions. The overlap of the circles is the intersection, $S \cap T$.}
\label{fig:Venn2}
\end{figure}

\begin{figure}
\centering
\begin{tikzpicture}
\draw[black] (0,0) rectangle (12,8);
\draw[black] (5,5) circle (2);
\draw[black] (7,5) circle (2);
\draw[black] (6,3) circle (2);
\node[text width=1] at (1,7) {$U$};
\node[text width=1] at (4,5) {$S$};
\node[text width=1] at (8,5) {$T$};
\node[text width=1] at (6,2) {$R$};
\end{tikzpicture}
\caption{A Venn diagram showing two dimensions. The overlap of all three circles is the intersection, $R \cap S \cap T$.}
\label{fig:Venn3}
\end{figure}

The Venn diagram has its more familiar structure with two and three dimensions,
as shown in figures \ref{fig:Venn2} and \ref{fig:Venn3}.

Venn diagrams are not possible in four or more dimensions -- at least, not with
circles drawn on a two-dimensional plot.
The number of subsets of $U$ is two for one dimension (a value is either in $S$
or not in $S$), four for two dimensions (a value is only in $S$, only in $T$,
in both, or in neither), eight for three dimensions ($R$, $S$, $T$, $S \cap T$, 
$S \cap R$, $S \cap T$, $T \cap R$, $R \cap S \cap T$, or none), and so on.

Another challenge one must avoid if using Venn diagrams is that the areas in
the plot may not correspond to the relative sizes of the subsets. For example,
imagine a Venn diagram showing the sets of bicycle riders and persons with only
one foot. The cyclists significantly outnumber the unipeds and their intersection
is likely quite small, therefore two circles of equal size may present a 
misleading graphic.

## Sample spaces

Imagine one wanted to conduct a large study on exercise and health outcomes.
Basic demographic variables include age, gender, location, height, weight, and race.
Exercise variables in this study include weekly minutes performing cardiovascular training, minutes of resistance training, and minutes of flexibility training.
Other exercise variables in this study include metrics of speed, endurance, strength, flexibility, blood pressure, resting heart rate, body composition, bone density, and sleep duration.

Suppose we discretize (see section \ref{sec:discretize}) each continuous variable into discrete categories.
For example, we might change the age variable from its numeric values to categories 1-10, 11-20, 21-30, and so on.
We separate height into very short, short, average, tall, and very tall.
We categorize minutes of weekly training into 0-20, 20-60, 60-120, and 120+.
Some variables are divided into very low, low, medium, high, and very high.
The process continues until all variables can be represented in discrete (sometimes ordered) categories instead of continuous numeric values.

| Variable | Categories |
|-|-|
| Age | 10 |
| Gender | 2 |
| Location | 5 |
| Height | 5 |
| Weight | 10 |
| Cardio Minutes | 4 |
| Weights Minutes | 4 |
| Stretch Minutes | 4 |
| Speed | 10 |
| Strength | 10 |
| Endurance | 10 |
| Flexbility | 10 |
| Blood Pressure | 5 |
| Heart Rate | 5 |
| Composition | 7 |
| Bone Density | 5 |
| Sleep Duration | 9 |

One might expect that, having discretized each variable, it would become easy to draw non-obvious conclusions from a reasonable sample size.
Unfortunately, there are $10 \times 2 \times 5 \times 5 \times 10 \times 4 \times 4 \times 4 \times 4 = \num{320000}$ possible combinations in the first eight variables alone.
Is it unusual for a middle-aged, very tall, very heavy, zero-exercise male living in North America to have average fitness metrics with poor body composition?
We would ideally like to sample many such persons, but even in a large study we likely would not have many individuals meeting exactly these characteristics.

*Data mining* is the search for non-obvious conclusions by analyzing data.
Data mining efforts are especially characterized by the lack of *first principles*, meaning the researcher may not have any advance hypothesis about the relationships between variables.

Suppose our fitness research showed that heavy bodyweight predicts poor speed.
This is quite obvious and likely not useful.
Suppose our fitness research showed that minutes of stretching predicted not only flexibility but also strength and body composition.
Such as result is less expected, and might (just as a hypothetical example) lead to a discovery that yoga develops muscle better than its reputation.

Data mining efforts in $n$-dimensional space are basically always complicated by this "curse of combinatorics."
When we repeatedly multiply many variables together, we find that the space of possible combinations becomes so large that even very large samples cover only tiny portions.
Our example health study has a total of $10 \times 2 \times 5 \times 5 \times 10 \times 4 \times 4 \times 4 \times 10 \times 10 \times 10 \times 10 \times 5 \times 5 \times 7 \times 5 \times 9 = \num{25200000000000}$ possible states in its *sample space*.

## Paradoxes

A *paradox* is a seemingly contradictory statement.
Large combinatorial sample spaces sometimes create unexpected situations that may seem paradoxical.

The *birthday paradox* is well-known in computer security.
Suppose there are 23 students in a class.
What is the probability that any two students share a birthday?
One might guess that the probability would be $23/365$ until we notice that **any** two students might share a birthday.
Student $s_1$ and $s_2$ might have the same birthday, $s_1$ and $s_3$, $s_2$ and $s_3$, and so on.

It is actually easier to calculate the probability that **no** students share a birthday, which we will denote with $q$.
For the first student ($s_1$), there is a (degenerate) $365/365$ probability that $s_1$ does not have share a birthday with those before because we have not considered any other students.
For $s_2$, there is a $364/365$ probability that $s_2$ has a distinct birthday from $s_1$.
For $s_3$, there is a $363/365$ probability that $s_3$ has a distinct birthday from both $s_1$ and $s_2$.
This continues for the remaining students in the class.
We multiply these probabilities together to get

$$
\begin{aligned}
q_{10} &= \frac{365}{365} \times \frac{364}{365} \times \frac{363}{365} \times \ldots \times \frac{343}{365} \\
&= \prod_{i=1}^{23}{\frac{365 - i + 1}{365}} \\
&= 0.492703.
\end{aligned}
$$

We now take $p = 1-q$ to find the probability that the event *does* occur and find the likelihood that two of our ten students is

$$
p = 1 - q = 1 - 0.492703 = 0.507297.
$$

This means that there is more than 50% chance that any two students will share a birthday in a class of 23, a surprising and unintuitive result.

## The Binomial Distribution

We now continue to another example which will demonstrate a limitation of statistical reasoning.
Suppose this class of students has a large toy box with 1000 toys.
Each time a child removes a toy, the teacher records the toy and the result of a fair coin flip.
For example,

| Toy     | Coin  |
|---------|-------|
| Shovel  | Heads |
| Racecar | Tails |
| Robot   | Heads |
| Teacup  | Tails |

After a very long time, each of the 1000 toys has been taken from the toy box 10 times.
The teacher looks over the data and is surprised to find that coin toss has always resulted in tails for each of the ten times that a child has taken the shark toy.

It should be obvious that the shark has nothing to do with the coin flip, yet unlikely events may entice one to assume causal relationships.
Consider the sample space of the coin flips.
The first flip, $c_1$, could have been heads or tails.
The second flip, $c_2$, could also have been heads or tails.
So far, the sample space contains four possible events, which we will denote HH, HT, TH, and TT.
On the third flip, the sample space again doubles in size: HHH, HHT, HTH, HTT, THH, THT, TTH, and TTT.
Each additional flip will continue to double the sample space.
By the tenth flip, the sample space contains $2^{10} = \num{1024} \approx \num{1000}$ possible events, of which HHHHHHHHHH is just one.

Upon reflection, it should be hardly surprising that one of one thousand toys would randomly associate with a one-in-one-thousand event.
To find the exact chance, we need the *binomial distribution*.
The probability of event $x$ occuring in a series of $n$ independent trials of probability $p$ is

$$
p(x) = \binom{n}{x} p^x (1-p)^{n-x}.
$$

In Excel, we use the `BINOM.DIST` function.
In R, `dbinom` in the *probability density function* (PDF) for the binomial distribution.
To find the probability that our $1/1024$ event occurs *exactly once* in $\num{1000}$ trials, we find

```r
> dbinom(1, 1000, 1/1024)
[1] 0.3679607
> choose(1000,1) * (1/1024)^1 * (1-1/1024)^(1000-1)
[1] 0.3679607
```

As an exercise, reproduce this result in Excel using the formula

```
=BINOM.DIST(1,1000,1/1024,FALSE)
```

We have several options to find the probability that *none* of our $\num{1000}$ toys associate with ten heads.
First, we can use the same `dbinom` and `BINOM.DIST` functions with $x = 0$.
Second, we can take the sum of probabilities from the range `x = 1:1000` (the probability of $x=1$, probability that $x=2$, and so on) and then subtract this from one.

```r
> dbinom(0, 1000, 1/1024)
[1] 0.3764238
> 1-sum(dbinom(1:1000, 1000, 1/1024))
[1] 0.3764238
```

Finally, statistics software often provides a *cumulative distribution function* (CDF) implementation as a shortcut for these summations.
In R, this is `pbinom`, but in Excel this is provided in `BINOM.DIST` with the final argument set to `TRUE`.

The toy shark example is intended to demonstrate how *spurious correlations* may occur in large sets of data.
The *Texas sharpshooter fallacy* can describe this effect.
A sharpshooter fires his pistol at random into a barn wall, then draws circles around clusters of bullet holes and claims to be an expert.

## Pareto frontier {#sec:pareto}

A *Pareto frontier* (also known as a *Pareto front*) is a method for visualizing the interaction of two *orthogonal* (statistically independent) features of a data set.

5/3/1 is a barbell strength training program [@Wendler2011-ar].
This program emphasizes *rep records*, where the lifter is to lift a submaximal mass as many times as possible.
This program design adds a second dimension to strength.
We say that lifter who progresses from lifting \qty{100}{\kilogram} for 6 repetitions to 9 repetitions in six months has become stronger,
even if the athlete has not directly tested their one-repetition maximum.

![The points along the red line form the Pareto front for this data set.](pareto.pdf){#fig:pareto}

Figure \ref{fig:pareto} provides an example of an athlete's rep records over a two-year period in the barbell squat.
The frontier, $P(X)$, is visible at the top-right of the scatter plot.
If, for example, this lifter were to achieve a \qty{120}{\kilogram} squat for 8 repetitions,
the lift would *dominate* the previous records at $(120,5)$ and $(116,8)$,
moving the frontier farther from the origin.

A Pareto front only makes sense when the two variables cannot be combined into one.
Consider, as an absurd example, a running race where the minutes and seconds of finishing times are recorded in separate columns.

| Athlete | Minutes | Seconds |
|---------|---------|---------|
| 1       | 18      | 34      |
| 2       | 19      | 24      |
| 3       | 20      | 01      |

There is no need to compare the three runner's run times in two dimensions: the minutes and seconds are trivially compressible into a single value with no loss of information.

In the case of the rep records shown in figure \ref{fig:pareto}, there is a general negative correlation between mass and repetitions.
This relationship can be estimated with Brzycki's formula (among others) [@brzycki1993strength], which states

$$
\text{Predicted 1-RM} = \frac{\text{Weight Lifted}}{1.0278 - 0.0289x},
$$

where $x$ is the number of repetitions performed.
Strong correlations in the columns of a data set present an opportunity to compress the data, thus reducing dimensionality, and search for non-obvious insights where one lacks first principles.

## Covariance and Correlation {#sec:cor}

In section \ref{sec:moments}, we defined variance as the average squared 
difference of a random variable $x$ to its expected value, $\bar{x}$.
*Covariance* [@10.1093/biomet/30.1-2.81] [@10.1093/biomet/33.3.239] is a similar
statistic for two variables: covariance is computed from the average product of
the differences in $x$ and $y$ to their respective expected values, $\bar{x}$
and $\bar{y}$.

$$
\text{cov}\left( x, y \right) = 
\sum_{i=1}^{n}{
  \frac
  {\left( x_i - \bar{x} \right) \left( y_i - \bar{y} \right)}
  {n-1}
}
$$

If we first *scale* vectors $x$ and $y$ such that their mean is zero and variance is 
one, then the covariance becomes *correlation*, a simple statistic to interpret.

$$
\begin{aligned}
\text{scale} \left( x \right) &= \frac{x - \bar{x}}{s_x} \\
\text{cor} \left( x, y \right) &= \text{cov} \left( 
  \text{scale} \left( x \right),
  \text{scale} \left( y \right)
  \right)
\end{aligned}
$$

The following Rust program implements both covariance and correlation statistics.
One can execute this program at the Rust Playground^[<https://play.rust-lang.org/?gist=1f3b41a17c10c354ee462062772dbd72>]
and reproduce the result in R at <https://docs.r-wasm.org/webr/latest/> with
`cov(c(5,7,3,6,8), c(65,80,50,70,90))` and `cor(c(5,7,3,6,8), c(65,80,50,70,90))`.

```rust
fn main() {
    let x = vec![5., 7., 3., 6., 8.];
    let y = vec![65., 80., 50., 70., 90.];
    println!("Covariance: {}", cov(&x, &y).unwrap());
    println!("Correlation: {}", cor(&x, &y).unwrap());
}

fn cov(x: &Vec<f64>, y: &Vec<f64>) -> Result<f64, ()> {
    if x.len() != y.len() {
        return Err(())
    }
    let xm = mean(x);
    let ym = mean(y);
    let n = x.len() as f64;
    let covariance = x.iter().zip(y.iter()).map(|(a,b)| {
        (a - xm) * (b - ym) / (n - 1.0)
    }).sum::<f64>();
    Ok(covariance)
}

fn cor(x: &Vec<f64>, y: &Vec<f64>) -> Result<f64, ()> {
    cov(&scale(x), &scale(y))
}

fn scale(v: &Vec<f64>) -> Vec<f64> {
    let mu = mean(v);
    let sigma = sd(v);
    v.iter().map(|x| {
        (x - mu) / sigma
    }).collect()
}

fn mean(v: &Vec<f64>) -> f64 {
    v.iter().sum::<f64>() / (v.len() as f64)
}

fn sd(v: &Vec<f64>) -> f64 {
    let mu = mean(v);
    let variance = v.iter().map(|x| {
        (x - mu).powi(2)
    });
    let n = v.len() as f64;
    (variance.sum::<f64>() / (n - 1.0)).sqrt()
}
```

To be more precise, the scaled covariance produces a statistic of **linear** correlation.
The correlation of the vector

$$
x = \left( -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5 \right)
$$

and its element-wise squares

$$
y = x \odot x = \left( 25, 16, 9, 4, 1, 0, 1, 4, 9, 16, 25 \right)
$$

is **zero**.

\begin{figure}
\centering
\begin{tikzpicture}

\draw[<->] (-6,0) -- (6,0);
\draw[<->] (0,-1) -- (0,3);

\draw[dotted] (-6,1.0) -- (6, 1.0);

\filldraw (-5,2.5) circle (1pt);
\filldraw (-4,1.6) circle (1pt);
\filldraw (-3,.9) circle (1pt);
\filldraw (-2,.4) circle (1pt);
\filldraw (-1,.1) circle (1pt);
\filldraw (0,0) circle (1pt);
\filldraw (1,.1) circle (1pt);
\filldraw (2,.4) circle (1pt);
\filldraw (3,.9) circle (1pt);
\filldraw (4,1.6) circle (1pt);
\filldraw (5,2.5) circle (1pt);

\end{tikzpicture}
\caption{The covariance of $x$ and $y = x \odot x$ is zero. The line of best fit for this data is shown on the dotted line, which has a Pearson correlation coefficient of $R^2=0$. Having a covariance of zero does not mean that $y$ is completely independent of $x$, but only that there is no linear dependence.}
\label{fig:quadratic-zero-r2}
\end{figure}

Again using the R language at <https://docs.r-wasm.org/webr/latest/>,

```r
> x = -5:5
> y = x^2
> cor(x,y)
[1] 0
```

There is a very recent development in the field of statistics with a new
coefficient of correlation [@10.1080/01621459.2020.1758115].
This new statistic, known as $\xi$ and pronounced "xi" or "ksaai", seeks to 
correlate $Y$ as some arbitrary function of $X$ and produces meaningful metrics 
on non-linear data. For our range $[-5,5]$ and its squares, the correlation 
coefficient is \num{0.5}.

The algorithm to compute $\xi(X,Y)$ first sorts $Y$ by $X$, then the *ranks*, 
$r$, of the resulting order of $Y$. If *order* is a list of positions
specifying the order of another list, then rank is the order of the order.
More formally, $r_i$ is the number of $j$ such that $Y_{(j)} \le Y_{(i)}$.
The statistic is

$$
\xi_n \left( X , Y \right) = 
1 - \frac{
  3 \sum_{i=1}^{n-1}{\left| r_{i+1} - r_{i} \right|}
}{n^2 -1}
$$

when there are no ties. If the data set does contain duplicates, then we also
use $l$ values, where $l_i$ is the number $j$ such that $Y_{(j)} \le Y_{(i)}$.

$$
\xi_n \left( X , Y \right) = 
1 - \frac{
  n \sum_{i=1}^{n-1}{\left| r_{i+1} - r_{i} \right|}
}{
  2 \sum_{i=1}^{n}{l_i \left( n - l_i \right)}
}
$$

A Rust implementation of this new $\xi$ statistic is given below and at the
Rust Playground^[<https://play.rust-lang.org/?gist=6336707980cfc3a54511842d937fb344>].

```rust
fn xicor(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    // This implementation does not handle duplicate values.
    let n = x.len();

    // 1) Sort y by x.
    let mut i: Vec<_> = (0..n).collect();
    i.sort_by(|&a, &b| x[a].total_cmp(&x[b]));
    let y: Vec<f32> = i.iter().map(|&i| y[i]).collect();

    // 2) Order y by sorting 1:n by y.
    let mut order: Vec<_> = (0..n).collect();
    order.sort_by(|&a, &b| y[a].total_cmp(&y[b]));

    // 3) Rank y by sorting 1:n by order.
    let mut r: Vec<_> = (0..n).collect();
    r.sort_by(|&a, &b| order[a].cmp(&order[b]));

    // Sum of absolute distances in successive y ranks.
    let mut r_consec_abs_dist = 0.0;
    for i in 1..n {
        r_consec_abs_dist += (r[i] as f32 - r[i-1] as f32).abs();
    }
    1.0 - 3.0 * r_consec_abs_dist / (n.pow(2) as f32 - 1.0)
}
```

## Causation

One must take care not to confuse correlation with causation. Consider an experiment
where seven subjects are each given a fair die and assigned "relationship" numbers
such that each pair of subjects $(x,y)$ shares a relationship $R_{xy}$ with every other subject,
as shown in figure \ref{fig:full-mesh} and enumerated in the following table.

| Subject | Relationships                               |
|---------|---------------------------------------------|
| 1       | $R_{12},R_{13},R_{14},R_{15},R_{16},R_{17}$ |
| 2       | $R_{12},R_{23},R_{24},R_{25},R_{26},R_{27}$ |
| 3       | $R_{13},R_{23},R_{34},R_{35},R_{36},R_{37}$ |
| 4       | $R_{14},R_{24},R_{34},R_{45},R_{46},R_{47}$ |
| 5       | $R_{15},R_{25},R_{35},R_{45},R_{56},R_{57}$ |
| 6       | $R_{16},R_{26},R_{36},R_{46},R_{56},R_{67}$ |
| 7       | $R_{17},R_{27},R_{37},R_{47},R_{57},R_{67}$ |

\begin{figure}
\centering
\begin{tikzpicture}
  \filldraw (2., 0.) circle (1pt);
  \filldraw (1.2469796037174672, 1.5636629649360596) circle (1pt);
  \filldraw (-0.445042, 1.9498558243636472) circle (1pt);
  \filldraw (-1.80194, 0.8677674782351165) circle (1pt);
  \filldraw (-1.80194, -0.867767) circle (1pt);
  \filldraw (-0.445042, -1.94986) circle (1pt);
  \filldraw (1.2469796037174667, -1.56366) circle (1pt);

  \draw[-] (2.,0.) -- (1.2469796037174672,1.5636629649360596);
  \draw[-] (2.,0.) -- (-0.445042,1.9498558243636472);
  \draw[-] (2.,0.) -- (-1.80194,0.8677674782351165);
  \draw[-] (2.,0.) -- (-1.80194,-0.867767);
  \draw[-] (2.,0.) -- (-0.445042,-1.94986);
  \draw[-] (2.,0.) -- (1.2469796037174667,-1.56366);
  \draw[-] (1.2469796037174672,1.5636629649360596) -- (-0.445042,1.9498558243636472);
  \draw[-] (1.2469796037174672,1.5636629649360596) -- (-1.80194,0.8677674782351165);
  \draw[-] (1.2469796037174672,1.5636629649360596) -- (-1.80194,-0.867767);
  \draw[-] (1.2469796037174672,1.5636629649360596) -- (-0.445042,-1.94986);
  \draw[-] (1.2469796037174672,1.5636629649360596) -- (1.2469796037174667,-1.56366);
  \draw[-] (-0.445042,1.9498558243636472) -- (-1.80194,0.8677674782351165);
  \draw[-] (-0.445042,1.9498558243636472) -- (-1.80194,-0.867767);
  \draw[-] (-0.445042,1.9498558243636472) -- (-0.445042,-1.94986);
  \draw[-] (-0.445042,1.9498558243636472) -- (1.2469796037174667,-1.56366);
  \draw[-] (-1.80194,0.8677674782351165) -- (-1.80194,-0.867767);
  \draw[-] (-1.80194,0.8677674782351165) -- (-0.445042,-1.94986);
  \draw[-] (-1.80194,0.8677674782351165) -- (1.2469796037174667,-1.56366);
  \draw[-] (-1.80194,-0.867767) -- (-0.445042,-1.94986);
  \draw[-] (-1.80194,-0.867767) -- (1.2469796037174667,-1.56366);
  \draw[-] (-0.445042,-1.94986) -- (1.2469796037174667,-1.56366);
\end{tikzpicture}
\caption{A \textit{full mesh} network of 7 elements contains $(7)(7-1)/2=21$ connections,
as explained in section \ref{sec:choose2}.}
\label{fig:full-mesh}
\end{figure}

Have each of your seven subjects roll their die. As there are only six sides to the die, we
are guaranteed that at least one pair of subjects will receive the same roll.
We look at our set of relationships and discover some $R_{xy}$ between those
who rolled the same number. We have a correlation, but is there a causal relationship?
Of course not.

A *mediating variable* makes establishing causality even more difficult.
(todo: say more about mediating variables)

*Confounding factors* introduce additional dimensions to a system and can make
analysis more complex or, sometimes, impossible. For example, a cohort of
hypothetical adult subjects enjoy positive health outcomes in one year having
started exercising regularly, improving sleep quality and duration, switching to
a healthy diet, stopping smoking, and always wearing blue clothes. Which of these
five variables led to improved health? Obviously, the blue clothes did *not*
contribute to the health outcomes, but in the presence of the other variables it
may be impossible to dismiss a preposterous claim that blue clothes are healthy.

An ideal study should predict, control, and explore the combinatorial space as
fully as possible. Consider another hypothetical exercise science study to
contrast the benefits of running versus cycling. If the researcher realizes that
the runners were significantly younger than the cyclists, then they may not be
able to distinguish whether the differences in the cohorts was due to activity
or age; the difference in age would be a confounding factor not *controlled* in
the study.

While mediating variables and confounding factors can create spurious or
misleading correlations, random *noise* in measurements can also obscure or
create or exaggerate the relationships between variables. Statistics like
correlation are useful to estimate an *effect size* to distinguish signal from
noise.

In a data mining effort, the interactions among features in the data set might
not be known in advance. *Big data*, large volumes of loosely-related and often
semi-structured data, may facilitate the exploration of the $n$-dimensional
space by providing vast numbers of both samples and features. In the next
section, we will learn a method for exploring the linear relationships among
the features of numerical data sets.

## Principal Component Analysis (PCA)

*Principal Component Analysis* (PCA) is a powerful technique for discovering linear
relationships among columns of data and compressing these columns into fewer dimensions
[@10.1080/14786440109462720] [@hotelling1933analysis].

PCA begins by finding all pairwise correlations among the data set's scaled columns.

```r
> head(iris[,-5])
  Sepal.Length Sepal.Width Petal.Length Petal.Width
1          5.1         3.5          1.4         0.2
2          4.9         3.0          1.4         0.2
3          4.7         3.2          1.3         0.2
4          4.6         3.1          1.5         0.2
5          5.0         3.6          1.4         0.2
6          5.4         3.9          1.7         0.4
> cor(iris[,-5])
             Sepal.Length Sepal.Width Petal.Length Petal.Width
Sepal.Length    1.0000000  -0.1175698    0.8717538   0.8179411
Sepal.Width    -0.1175698   1.0000000   -0.4284401  -0.3661259
Petal.Length    0.8717538  -0.4284401    1.0000000   0.9628654
Petal.Width     0.8179411  -0.3661259    0.9628654   1.0000000
> c = .Last.value
```

We then use a technique from linear algebra called *Singular Value Decomposition* (SVD),
which extracts a diagonal matrix $D$ from $A$ where $U' AV = D$, $U'U = I$, and $V'V = I$.
We will not discuss the details of this procedure, but will instead leave it to library software.

```r
> svd(c)$u
           [,1]        [,2]       [,3]       [,4]
[1,] -0.5210659 -0.37741762  0.7195664  0.2612863
[2,]  0.2693474 -0.92329566 -0.2443818 -0.1235096
[3,] -0.5804131 -0.02449161 -0.1421264 -0.8014492
[4,] -0.5648565 -0.06694199 -0.6342727  0.5235971
> u = .Last.value
```

We multiply our scaled data set by $U$ to compute the principal components, $\text{PC} = X U$.

```r
> head(scale(iris[,-5]) %*% u)
         [,1]       [,2]        [,3]         [,4]
[1,] 2.257141 -0.4784238  0.12727962  0.024087508
[2,] 2.074013  0.6718827  0.23382552  0.102662845
[3,] 2.356335  0.3407664 -0.04405390  0.028282305
[4,] 2.291707  0.5953999 -0.09098530 -0.065735340
[5,] 2.381863 -0.6446757 -0.01568565 -0.035802870
[6,] 2.068701 -1.4842053 -0.02687825  0.006586116
> pc = scale(iris[,-5]) %*% u
> head(pc)
         [,1]       [,2]        [,3]         [,4]
[1,] 2.257141 -0.4784238  0.12727962  0.024087508
[2,] 2.074013  0.6718827  0.23382552  0.102662845
[3,] 2.356335  0.3407664 -0.04405390  0.028282305
[4,] 2.291707  0.5953999 -0.09098530 -0.065735340
[5,] 2.381863 -0.6446757 -0.01568565 -0.035802870
[6,] 2.068701 -1.4842053 -0.02687825  0.006586116
```

The resulting matrix can be used for small but accurate linear models.
PCA can also reveal unexpected correlations among the data. 
One can think of the columns of $U$ as new dimensions that might have been hidden
among the correlated features of the original data set.
The covariance among the principal components is effectively zero.

```r
> library(tidyverse)
> cov(pc) %>% zapsmall
         [,1]      [,2]      [,3]      [,4]
[1,] 2.918498 0.0000000 0.0000000 0.0000000
[2,] 0.000000 0.9140305 0.0000000 0.0000000
[3,] 0.000000 0.0000000 0.1467569 0.0000000
[4,] 0.000000 0.0000000 0.0000000 0.0207148
```

The columns are ordered from greatest to least variance. This means that a model
might not need all four columns to form accurate predictions, as the later columns
account for very little of the variance in the data set.

<!--
I'm not really satisfied with this passage. A PCA plot would have helped.
A more compelling application would have helped more. The reader probably
won't be able to see why PCA is so interesting or useful.

**TODO**: PCA plot.
-->

todo: PCA plot.

<!-- 

https://crates.io/crates/statrs
https://crates.io/crates/nalgebra

-->

## Discussion prompts

1. <https://www.tylervigen.com/spurious-correlations> curates an entertaining collection of spurious correlations. However, not all spurious correlations might be so obvious. What are some principals we should apply to either trust or be skeptical of statistical evidence?
2. Conduct a classroom competition of "Catch the cheaters!" at <https://primerlearning.org>. Discuss the winning and losing strategies, then watch <https://www.youtube.com/watch?v=XTcP4oo4JI4>. 
3. Read the interactive article <https://www.mayerowitz.io/blog/mario-meets-pareto> [@mayerowitzMarioMeets]. Discuss the compromises inherent in a multi-objective optimization problem.
4. Controversial topics may involve several dimensions. Advocates for one position
may claim on one basis in dimension $x$, where the opposition's counterclaim is
in dimension $y$. Discuss a contemporary impasse with orthogonal or irreconcilable
aspects.

## Practical exericses

1. Use nested `sapply` statements to improve `sapply(0:4, function(r) pascal(4, r))`. Iterate `pascal(n, r)` over $0 \le n \le 10$ and $0 \le r \le n$, generating the first 11 lines of Pascal's Triangle. Compare the result to `sapply(0:10, function(n) choose(n, 0:n))`. Why does the built-in `choose` function accept ranges (`0:n`) when our own `pascal` function does not?
2. About one in twenty white males have some form of color blindness. About 70.2% of the U.S. military report themselves as white, and about 82.8% as male. Let $P(C|W \cap M)=0.05$, $P(W)=0.702$, and $P(M)=0.828$. If a Command gives a briefing to twelve random generals each year, what is the probability that one or more of those generals is color blind? (Assume, incorrectly but for the sake of simple calculation, that women and non-whites are never color blind.) Assume further that $W$ and $M$ are independent and that $P(W \cap M) = P(W)P(M) = 0.581256$, therefore $P(C|W \cap M) = \frac{P(C \cap W \cap M)}{P(W \cap M)}$ and consequently $P(C \cap W \cap M) = P(C | W \cap M) P(W \cap M) = (0.05)(0.581256) = 0.0290628$. Use this value for $p$ in your `dbinom` calculation. Based upon this result, is it wise to depend on color-coded graphics in a presentation?
3. Come up with a creative way to draw a four-dimensional Venn diagram.
4. Use Excel to reproduce the zero correlation between $x$ and $y_1 = x \odot x$ from section \ref{sec:cor}. Now update the $y$ column to $y_2 = x \odot x + x = \left( 20, 12,  6,  2,  0,  0,  2,  6, 12, 20, 30 \right)$. What is $\text{cor}\left( x, y_2 \right)$?
5. Use Excel's line of best fit feature to construct a linear models between both $x$ and $y_1$ and also $x$ and $y_2$. Observe that the $y$-intercept in both models is $10$. Try to figure out where this constant comes from. <!-- answer: mean(x) -->
