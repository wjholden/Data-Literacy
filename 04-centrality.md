# Distributions and Measures of Central Tendency

## Distributions

Repeated flips of a fair coin should produce about 50% heads and 50% tails.
Repeated rolls of a fair die should land on each side about $1/6$ of the time.
The outcomes of the coin and the die fit a *uniform distribution*, which means
that all outcomes are equally likely.

An understanding of the distribution of data can be an extremely powerful tool
because it gives enables us to apply deductive reasoning techniques at low
cost and high confidence.

For example, in the video game Borderlands 3, players estimate that they win
valuable items in about 10% of attempts. If it takes two minutes per "farming"
attempt, then in one hours a player can expect to win approximately
$(0.10) 60 / 2 = 3$ of these valuable items. Sales teams can apply similar
analysis: if the company expects to close a deal in 10% of customer interactions,
then this knowledge can inform decisions about advertising and outreach.

The same sales team might also notice that not all of their salespeople are
equally successful. The company might identify high-performing salespeople to
sustain, and it correspondingly might identify low-performing salespeople 
improve.

In this chapter, we will learn about the least squares method, basic statistics,
and a few practical considerations for studying distributions, their
central tendencies, and their extrema.

## Least squares method  {#sec:least-squares-method}

The canonical definition of the *arithmetic mean* for a set of $n$ numbers $x$ is

$$
\bar{x} = 
\frac{1}{n} \sum_{i = 1}^{n}{x_i}= 
\frac{x_1 + x_2 + x_3 + \cdots + x_4}{n}.
$$

Where does this definition come from?
Let's open a new workbook in Microsoft Excel and find out.

Leave A1 blank.
This will become our *estimate of the mean*.
In fields B1 through B9, enter integers 1 through 9.
In fields C1 through C9, enter formulas `=B1-$A$1`, `=B2-$A$1`, `=B3-$A$1`, and so on (keeping `$A$1` fixed).
In fields D1 through D9, enter formulas `=POWER(C1,2)`, `=POWER(C2,2)`, and so on.
In field E1, enter formula `=SUM(D1:D9)`.
Finally in field F1, enter the formula `=AVERAGE(B1:B9)`.

Now go to Data, What-If Analysis, Goal Seek.
In the Goal Seek dialog, enter `Set cell:` to `E1`, `To value:` to `0`, and `By changing cell:` to `A1`.
Click OK.
The Goal Seek function runs and should produce a value in A1 near to that in F1.
(Goal Seek is not foolproof. Enter `10` into A1 to nudge Excel with a hint if you get a ridiculous answer.)

We have used Goal Seek to minimize the *sum of the squared differences* between our values, $x_i$, and our estimate of the mean, $\bar{x}$.
This *least squares method* dates back to Carl Friedrich Gauss and Adrien-Marie Legendre in the 1800s [@legendre_1805] [@legendre_translation].

Squaring the errors makes the values positive, which prevents underestimates from negating overestimates.
One might also consider the *absolute value* (`ABS` in Excel) as an alternative, but there is a second reason for squaring the errors.
Squaring the errors penalizes large errors more than small errors.
Accepting small errors but avoiding large errors is the bias that gives the least squares method its strength.

Readers familiar with calculus may recognize that one can find the arithmetic mean, $\mu$, by finding the zero in the *derivative* for the sum of the squared errors (SSE) function.
Let $X$ be a sample of size $n$.

$$
X = \left\{ x_1, x_2, \ldots , x_n \right\}.
$$

Then the errors of our estimate of the mean, $\bar{x}$, can be found using the error function

$$
\text{Err}\left( \bar{x} \right) = X - \bar{x} = \left\{ x_1 - \bar{x} , x_2 - \bar{x}, \ldots , x_n - \bar{x} \right\},
$$

and the sum of the squared errors is

$$
\text{SSE}\left( \bar{x} \right) = \left( x_1 - \bar{x} \right)^2 + \left( x_2 - \bar{x} \right)^2 + \cdots + \left( x_n - \bar{x} \right)^2.
$$

To minimize SSE, we take the derivative of SSE in respect to $\bar{x}$ and find its zero.

$$
\begin{aligned}
0 &= \frac{\mathrm{d} \text{SSE}}{\mathrm{d} \bar{x}} \\
&=  \frac{\mathrm{d} \left( \left( x_1 - \bar{x} \right)^2 + \left( x_2 - \bar{x} \right)^2 + \cdots + \left( x_n - \bar{x} \right)^2 \right)}{\mathrm{d} \bar{x}} \\
&= \frac{\mathrm{d} \left( \left( x_1^2 -2 x_1 \bar{x} + \bar{x}^2 \right) + \left( x_2^2 -2 x_2 \bar{x} + \bar{x}^2 \right) + \cdots + \left( x_n^2 -2 x_n \bar{x} + \bar{x}^2 \right) \right)}{\mathrm{d} \bar{x}} \\
&= \left(-2x_1 + 2\bar{x} \right) + \left( -2x_2 + 2\bar{x} \right) + \cdots + \left( -2x_n + 2\bar{x} \right) \\
&= -2x_1 -2x_2 - \cdots - 2x_n + n \left( 2\bar{x} \right) \\
-2n\bar{x} &= -2x_1 - 2x_2 - \cdots - 2x_n \\
\bar{x} &= \frac{-2x_1 - 2x_2 - \cdots - 2x_n}{-2n} \\
&= \frac{x_1+x_2+\cdots+x_n}{n}
\end{aligned}
$$

The arithmetic mean is found at $\mu = \bar{x} = \frac{x_1+x_2+\cdots+x_n}{n}$. $\square$

A less-general demonstration of the above proof is given below in the Wolfram Language.

```mathematica
In[1]:= X := {x1, x2, x3}

In[2]:= Err[mu_] := X - mu

In[3]:= SSE[mu_] := Total[Err[mu]^2]

In[4]:= SSE'[mu]

Out[4]= -2 (-mu + x1) - 2 (-mu + x2) - 2 (-mu + x3)

In[5]:= Solve[SSE'[mu] == 0, mu]

                x1 + x2 + x3
Out[5]= {{mu -> ------------}}
                     3
```

## Expected values

The term *average* can refer to *mean*, *median*, and *mode*.
Mean only applies to interval and ratio data.
Median is simply the middle value among ordinal, interval, and ratio data.
Mode is the "commonest" (most frequent) value among nominal, ordinal, interval, and ratio data.

| Average  | Levels of measurement | Symbols            |
|----------|-----------------------|--------------------|
| Mean     | Interval, ratio       | $\mu$, $\bar{x}$   |
| Median   | Ordinal, interval, ratio | (None)          |
| Mode     | Nominal, ordinal, interval, ratio | (None) |

All three of these *measures of central tendency* enable us to find the *expected value* in a data set, $E(x)$.
Population means are assigned the symbol $\mu$.
Estimates of the population mean (the sample mean) usually use the name of the sample with a bar, such as $\bar{x}$.

Median and mode can be useful even when analyzing interval and ratio data.
Consider a classroom of 10 students who are 6 years old and 1 teacher who is 50 years old.
If one selects a random person in the room, what is the expected value for their age?
In this case, the modal value (6) is likely a better estimate than the mean value (10).

## The Four Moments {#sec:moments}

The *four moments* provide measures of central tendency among data and functions.
The first moment is the mean.
The second moment is *variance*, the expected squared difference of values to the mean.
The third moment is *skewness*, the expected cubed difference of values to the mean.
The fourth moment is *kurtosis*, the expected difference of values to the mean raised to the fourth power.

| Moment  | Name     | Definition               | Symbol     |
|---------|----------|--------------------------|------------|
| $\mu_1$ | Mean     | $E(x)$                   | $\mu$      |
| $\mu_2$ | Variance | $E(x-\mu)^2$             | $\sigma^2$ |
| $\mu_3$ | Skewness | $E(x-\mu)^3$             | $\beta_1$  |
| $\mu_4$ | Kurtosis | $E(x-\mu)^4$             | $\beta_2$  |

Variance ($\sigma^2$) is calculated from the sum of the squared differences in the random variable ($x$) and the mean ($\mu$).

$$
\begin{aligned}
\sigma^2 &= E(x-\mu)^2 \\
&= E(x^2 - 2x\mu + \mu^2) \\
&= E(x^2) - 2E(x)\mu + \mu^2 \\
&= E(x^2) - 2(\mu)\mu + \mu^2 \\
&= E(x^2) -2\mu^2 + \mu^2 \\
&= E(x^2) - \mu^2.
\end{aligned}
$$

To calculate the sample variance ($s^2$) in practice, we use the formula

$$
s^2 = \frac{1}{n-1} \sum_{i=1}^{n} {\left( x_i - \bar{x} \right)^2}.
$$

where $n$ is the number of elements in $x$.
Observe that the calculation for this statistic is similar to the least squares method in section \ref{sec:least-squares-method}.
$s^2$ is the average squared distance from the mean within the data set.

The *standard deviation* is the square root of variance,

$$
s = \sqrt{s^2}.
$$

An *outlier* is a value that is very different from other values in the data set.
In the set $x = \left\{ 15, 75, 79, 10, 7, 54, \num{4600}, 91, 45, 86 \right\}$,
one can immediately observe that the value $\num{4600}$ is substantially different from all of the other values.
Outliers can be defined in terms of the mean and standard deviation.
Outliers are any values greater than $\bar{x} + 2s$ or less than $\bar{x} - 2s$.
We can use the `mean` and `sd` functions with `subset` in the R language at <https://webr.r-wasm.org/latest/> to confirm that $\num{4600}$ is an outlier.

```
> x = c(15, 75, 79, 10, 7, 54, 4600, 91, 45, 86)
> subset(x, x <= mean(x) - sd(x) | x >= mean(x) + sd(x))
[1] 4600
```

We can use skewness ($\mu_3 = \beta_1$) to detect whether the data is imbalanced (skewed) above or below the mean.
If skewness is negative then the left tail is longer, if skewness is positive then the right tail is longer,
and if skewness is zero then the distribution is equally balanced over the mean.
Excel defines its `SKEW` function^[<https://support.microsoft.com/en-us/office/skew-function-bdf49d86-b1ef-4804-a046-28eaea69c9fa>] as

$$
\mu_3 = \frac{n}{(n-1)(n-2)} \sum{\left( \frac{x - \bar{x}}{s} \right)^3}.
$$

We can use kurtosis ($\mu_4 = \beta_2$) to detect if a data set contains outliers.
The kurtosis of the normal distribution is 3.
Karl Pearson defines the *degree of kurtosis* as $\eta = \beta_2 - 3$ [@10.1093/biomet/4.1-2.169, p. 181].
Other texts call this *excess kurtosis*. Excel's `KURT` function returns excess 
kurtosis. If `KURT` returns 0, then the distribution may fit a normal distribution
and may contain no outliers. Excel defines its `KURT`
function^[<https://support.microsoft.com/en-us/office/kurt-function-bc3a265c-5da4-4dcb-b7fd-c237789095ab>] as

$$
\mu_4 =
\frac{n(n+1)}{(n-1)(n-2)(n-3)}
\sum{\left( \frac{x-\bar{x}}{s} \right)^4}
-
\frac{3(n-1)^2}{(n-2)(n-3)}.
$$

## The Normal Distribution

The *Normal Distribution*, also known as the *Gaussian Distribution*, is a well-known
predictor of the probability of continuous outcomes. Parameterized by mean, $\mu$,
and standard deviation, $\sigma$, the *probability density function* for the
normal distribution is

$$
P(x) = \frac{1}{\sigma \sqrt{2 \pi}} e^{-(x-\mu)^2/(2 \sigma^2)}.
$$

$P(x)$ predicts the probability that an observation will have value $x$.
The *standard normal* has parameters $\mu=0$ and $\sigma=1$, with skewness $\mu_3 = 0$ and kurtosis $\mu_4 = 3$.

$P(x)$ forms a "bell curve" (see figure \ref{fig:normal}) that one might encounter in a histogram of data, but
there are other distributions of data which also form a bell-shaped curve. It is
**not** generally safe to immediately assume that data fits a normal distribution
when a histogram reflects a bell curve.

\begin{figure}
\centering
\includegraphics{normal.tikz}
\caption{This familiar "bell curve" is a plot of the probability density function of the normal distribution. Though the curve appears to touch the horizontal axis in this plot, the values actually approach but never reach zero, even as $x$ continues infinitely far in either direction.}
\label{fig:normal}
\end{figure}

Just as *density* in physics is defined as mass per volume, the probability
density function is correspondingly a rate. The units for $P(x)$ are probability
per value. If we wanted to find mass from the density of a fluid, we would 
multiply its density by the volume of fluid we have. Correspondingly, to take
the *probability mass* from $P(x)$, we multiply probabilities by the range of
$x$ values.

In a *uniform distribution*, finding the probability mass is as simple as multiplying
the probability, $U(x)$, by the range of $x$ values. For example, the probability
of getting $0.0 \le x \le 0.5$ from a uniform random number generator producing
values in the range $\left[ 0.0, 1.0 \right]$ is (informally) $U \times (0.5-0) = 0.5$.
What is the probability of getting *exactly* $0.12345$? If $U$ can produce infinitely
many digits, then the probability mass is zero. Not approximately zero --- exactly zero:
we multiplied $U \times (0.12345 - 0.12345) = U \times (0) = 0$.

Probability density is not constant in the normal distribution, therefore to get
the probability mass we can use calculus. The integral of the entire space is

<!-- <https://tex.stackexchange.com/questions/14821/whats-the-proper-way-to-typeset-a-differential-operator> -->
$$
\int_{-\infty}^{+\infty} P(x)\,dx = 1.
$$

The probabilities of an event occurring at $\pm \sigma$, $\pm 2\sigma$,
and $\pm 3\sigma$ are approximately 68%, 95%, and 99%.

$$
\begin{aligned}
\int_{-1}^{+1} P(x)\,dx \approx 0.68 \\
\int_{-2}^{+2} P(x)\,dx \approx 0.95 \\
\int_{-3}^{+3} P(x)\,dx \approx 0.99 \\
\end{aligned}
$$

Just as the probability mass was zero when the range had zero length, we also
find that the probability mass for a single value is zero.

$$
\int_{t}^{t} P(x)\,dx = 0
$$

The R language provides functions `dnorm`, `pnorm`, `qnorm`, and `rnorm`.
`dnorm` is the same probability density function shown above as $P(x)$.
`pnorm` gives the probability mass (also known as *cumulative probability*)
between some range of $x$ values, which are numerical estimations of the definite
integrals shown earlier.

```r
> integrate(dnorm, -1, 1)
0.6826895 with absolute error < 7.6e-15
> integrate(dnorm, -2, 2)
0.9544997 with absolute error < 1.8e-11
> integrate(dnorm, -3, 3)
0.9973002 with absolute error < 9.3e-07
> integrate(dnorm, -Inf, 0)
0.5 with absolute error < 4.7e-05
> integrate(dnorm, 0.12345, 0.12345)
0 with absolute error < 0
> pnorm(0)
[1] 0.5
> pnorm(1) - pnorm(-1)
[1] 0.6826895
```

The `qnorm` function is the inverse of cumulative probability, allowing us to
find an exact point $x$ in the distribution for a probability value $p=P(x)$.

The `rnorm` function generates random numbers which fit a normal distribution.
Some computing environments provide only uniformly-distributed random numbers.
In Excel, one can use the `qnorm` equivalent (`NORM.INV`) in a clever way to
create a normal distribution with

```excel
=NORM.INV(RAND(),0,1)
```

## Exponential moving averages

An *exponential moving average* (EMA) is a weighted average that creates a bias favoring recent observations.
EMAs are used in the financial sector as an implicit means of modeling stock prices with time.

One might also think of EMA as a method to estimate *instantaneous* values in
presence of errors. An example application might be a smartwatch using GPS to
estimate a runner's pace. As the runner's wrist travels back and forth, the
instantaneous velocity of the watch will not match that of the runner. An EMA
might be useful to smooth this noise.

EMA is defined *recursively* and parameterized with a weighting multiplier $0 < p < 1$.

$$
\text{EMA} \left( x, i \right) = 
\begin{cases}
p x_i + (1 - p) \text{EMA} \left( x, i - 1 \right), & \text{if $i > 1$}. \\
x_1, & \text{otherwise}.
\end{cases}
$$

EMA can be easily implemented in terms of the `reduce` operation, as shown below in JavaScript.

```
>> x = [7, 8, 9, 10, 9, 8, 7, 9, 11, 13, 15, 17]
>> p = .25
>> x.reduce((ema,v) => p * v + (1-p) * ema, x[0])
<- 12.661770105361938
```

The *base case* is at $x_1$ in mathematical notation but `x[0]` in JavaScript.
This is a matter of convention; the first element of a list is position 1 in math, but 0 in many programming languages.

## Strong and Weak Links {#sec:strong-weak-link}

In a *weak-link problem*, the system is harmed by the presence of any defect.
The term itself is named for a proverbial chain, which is only as strong as the
weakest link. Many safety-, quality-, and process-related problems require one
to eliminate weak links.

On the other hand, we sometimes encounter *strong-link problems*, where overall
success of a system depends on the very *best* individuals in the population.
Olympic athletes are a good example of a strong-link problem: it does not matter
that a country has thousands of candidate athletes who did not qualify; what
matters is that the national team selects the very best to compete on the world
stage.

Venture capital is another example of a strong-link problem. An investor takes
risks on many different companies in hopes that one "unicorn" startup will yield
a large return, offsetting losses from those unsuccessful startups.

Both weak-link and strong-link problems can be modeled in terms of variance (see
section \ref{sec:moments}).

## Inclusion Criteria

The "big" in "big data" refers data mining efforts in very large data sets that
one cannot process on small computers with traditional methods. For example,
consider an agricultural process which historically measured its yield in
volume (such as liters of milk), but later changed this metric to the financial
value of the yield (such as a dollars per shipping container, where not all 
shipping containers deliver the same product). In this hypothetical situation,
an analyst might attempt to transform one or both units to a comparable
calculated column, but the process may introduce uncertainty.

All studies, large and small, require inclusion criteria. Obvious reasons to
exclude samples include:

- Duplicated rows
- Missing values
- Obvious errors (i.e., height and weight entered as "5" and "11")

Outlier analysis can also cast out potential errors, although this technique
is inappropriate in strong- and weak-link problems where one's goal is to
investigate those outliers.

In some situations, one might decide to exclude certain samples that introduce
uncontrolled variance. An example might be a study on 35 automobiles, where
33 cars are new and 2 of the cars are 30 years old. Depending on the study, it
might be appropriate to exclude the two old cars from the study. Another example
might be a health study of 195 women and only 3 men. The researcher might choose
to exclude the men to reduce the dimensionality of the problem. We will discuss
the "curse of combinatorics" in the next chapter and see that every additional
feature increases the complexity of one's study.

Mark Twain is often quoted to have said, "There are lies, damned lies, and
statistics." A researcher can create lies by manipulating inclusion criteria.
Many statistical tests output a probability ($p$), called $p$-*values*.
Scientists engage in $p$-*hacking* when they seek to manipulate the $p$-values
in a study to coerce a desired result [@10.3389/fpsyg.2016.01832]. One must
take care to choose inclusion criteria in a logical manner that does not bias
the result of further statistical analysis. Mathematicians describe their
beliefs as "strong opinions, weakly held" as a framework for passionately
pursuing the truth --- until one realizes their opinions are not true. "Strong
opinions *strongly* held" is just politics by another means.

## Discussion prompts

#. Is four a lot? 

#. First battalion has an average ACFT score of 482 while second battalion has an average ACFT score of 491. Which is better? 

#. What do we do when statistics show us something that contradicts our values? For example, suppose we discover that Soldiers of a specific demographic have much lower promotion rates than their peers. 

#. Is it more important for an organization to think about variance or the 99th percentile? 

#. Given a sample set $x = \left\{ 5 \right\}$, what is the estimate of the mean ($\bar{x}$), and what is the sample variance ($s_x$)?
That is, what is the expected value ($E(x)$) of a random sample taken from $x$, and what is the average difference of variables to the expected value?
Use software to verify your answer. In the R language, this would be `mean(c(5))` and `sd(c(5))`.

#. A customer-service organization uses *average handling time* (AHT),
the expected number of minutes required to complete an interaction, to improve
service quality. What are some benefits or risks of this approach?
See <https://xkcd.com/2899/>.

#. Does the Army's standards-based approach hinder our ability to solve strong-link
problems requiring unpredictable solutions? How could an ideal institution maintain
minimum standards while enabling outliers to flourish? 

#. Generate a data frame of random numbers with ten rows and three columns
    in the R language^[<https://webr.r-wasm.org/latest/>].

    ```r
    data.frame(A = rnorm(10), B = rnorm(10), C = rnorm(10))
    ```
    
    The values of each column should be normally distributed, with a mean of
    about 0 and a standard deviation of about 1.

    Share the resulting data and split into groups to compete in a $p$-hacking
    game. One group should try to find an arbitrary inclusion criteria for $B$
    or $C$ that satisfies $\bar{A} \le -1$. Another group can try to find a
    different inclusion criteria that makes $\bar{A} \ge 1$. The winner is the
    group that retains the most rows using the simplest inclusion criteria.

## Practical exercises

#. Calculate the influence that outliers have on different-sized datasets that contain outliers. 

#. Calculate the exponential moving average in a small dataset. 

#. Given a dataset and experimental result, identify problems caused by analyzing categorical data represented in a numeric form. 

#. Given multiple datasets with identical mean and standard deviation, use kurtosis to identify the dataset with more outliers. 

#. Design or implement an algorithm to incrementally calculate standard deviation, where the estimate of the sample standard deviation is updated with each additional value. 

#. Think backwards and try to guess what would be the zeroth moment, $\mu_0$.
