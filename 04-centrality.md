# Measures of Central Tendency

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
\text{SSE}\left( \bar{x} \right) = \left( x_1 - \bar{x} \right)^2 + \left( x_2 - \bar{x} \right)^2 + \ldots + \left( x_n - \bar{x} \right)^2.
$$

To minimize SSE, we take the derivative of SSE in respect to $\bar{x}$ and find its zero.

$$
\begin{aligned}
0 &= \frac{\mathrm{d} \text{SSE}}{\mathrm{d} \bar{x}} \\
&=  \frac{\mathrm{d} \left( \left( x_1 - \bar{x} \right)^2 + \left( x_2 - \bar{x} \right)^2 + \ldots + \left( x_n - \bar{x} \right)^2 \right)}{\mathrm{d} \bar{x}} \\
&= \frac{\mathrm{d} \left( \left( x_1^2 -2 x_1 \bar{x} + \bar{x}^2 \right) + \left( x_2^2 -2 x_2 \bar{x} + \bar{x}^2 \right) + \ldots + \left( x_n^2 -2 x_n \bar{x} + \bar{x}^2 \right) \right)}{\mathrm{d} \bar{x}} \\
&= \left(-2x_1 + 2\bar{x} \right) + \left( -2x_2 + 2\bar{x} \right) + \ldots + \left( -2x_n + 2\bar{x} \right) \\
&= -2x_1 -2x_2 - \ldots - 2x_n + n \left( 2\bar{x} \right) \\
-2n\bar{x} &= -2x_1 - 2x_2 - \ldots - 2x_n \\
\bar{x} &= \frac{-2x_1 - 2x_2 - \ldots - 2x_n}{-2n} \\
&= \frac{x_1+x_2+\ldots+x_n}{n}
\end{aligned}
$$

The arithmetic mean is found at $\mu = \bar{x} = \frac{x_1+x_2+\ldots+x_n}{n}$. $\square$

A less-general demonstration of the above proof is given below in the Wolfram Language.

```mathematica
Wolfram Language 14.0.0 Engine for Microsoft Windows (64-bit)
Copyright 1988-2023 Wolfram Research, Inc.

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

The *four moments* describe the *distribution* of values in a data set.
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
\begin{tikzpicture}

\begin{axis}[
    axis lines = left,
    xlabel = \(x\),
    ylabel = {\(P(x)\)},
]

\addplot [
    domain=-4:4,
    samples=100,
    color=red,
]{1/(1 * sqrt(2 * pi)) * exp(-(x - 0)^2/(2 * (1)^2))};

%\addlegendentry{\(\frac{1}{\sigma \sqrt{2 \pi}} e^{-(x-\mu)^2/(2 \sigma^2)}\)}

\end{axis}

\end{tikzpicture}
\caption{This familiar "bell curve" is a plot of the probability density function of the normal distribution. Though the curve appears to touch the horizontal axis in this plot, the values actually approach but never reach zero, even as $x$ continues infinitely far in either direction.}
\label{fig:normal}
\end{figure}

<!-- TODO: provide symbolic integrals and explain why you need to calculate
these as ranges, not P(0) (as the integral from x=0 to x=0 is 0).

This is also an opportunity to explain the 68/95/99% things.
-->

The R language provides functions `dnorm`, `pnorm`, `qnorm`, and `rnorm`.
`dnorm` is the same probability function shown above as $P(x)$.
`pnorm` gives the cumulative probability between some range of $x$ values,
which is simply a definite integral.

```r
> integrate(dnorm, -1, 1)
0.6826895 with absolute error < 7.6e-15
> integrate(dnorm, -2, 2)
0.9544997 with absolute error < 1.8e-11
> integrate(dnorm, -3, 3)
0.9973002 with absolute error < 9.3e-07
> integrate(dnorm, -Inf, 0)
0.5 with absolute error < 4.7e-05
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

## Strong and Weak Links

In a *weak-link problem*, the system is harmed by the presence of any defect.
The term itself is named for a proverbial chain, which is only as strong as the
weakest link. Many safety-, quality-, and proces-related problems require one
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

## Discussion prompts

1. Is four a lot? 

2. First battalion has an average ACFT score of 482 while second battalion has an average ACFT score of 491. Which is better? 

3. What do we do when statistics show us something that contradicts our values? For example, suppose we discover that Soldiers of a specific demographic have much lower promotion rates than their peers. 

4. Is it more important for an organization to think about variance or the 99th percentile? 

5. Given a sample set $x = \left\{ 5 \right\}$, what is the estimate of the mean ($\bar{x}$), and what is the sample variance ($s_x$)?
That is, what is the expected value ($E(x)$) of a random sample taken from $x$, and what is the average difference of variables to the expected value?
Use software to verify your answer. In the R language, this would be `mean(c(5))` and `sd(c(5))`.

6. A customer-service organization uses *average handling time* (AHT),
the expected number of minutes required to complete an interaction, to improve
service quality. What are some benefits or risks of this approach?
See <https://xkcd.com/2899/>.

7. Does the Army's standards-based approach hinder our ability to solve strong-link
problems requiring unpredictable solutions? How could an ideal institution maintain
minimum standards while enabling outliers to flourish? 

## Practical exercises

1. Calculate the influence that outliers have on different-sized datasets that contain outliers. 

2. Calculate the exponential moving average in a small dataset. 

3. Given a dataset and experimental result, identify problems caused by analyzing categorical data represented in a numeric form. 

4. Given multiple datasets with identical mean and standard deviation, use kurtosis to identify the dataset with more outliers. 

5. Design or implement an algorithm to incrementally calculate standard deviation, where the estimate of the sample standard deviation is updated with each additional value. 

6. Think backwards and try to guess what would be the zeroth moment, $\mu_0$.
