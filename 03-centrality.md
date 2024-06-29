# Measures of Central Tendency

## Least squares method  {#section:least-squares-method}

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

## Calculus-based derivation

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
0 &= \text{SSE}' \left( \bar{x} \right) \\
&=  \left( \left( x_1 - \bar{x} \right)^2 + \left( x_2 - \bar{x} \right)^2 + \ldots + \left( x_n - \bar{x} \right)^2 \right)' \\
&= \left( \left( x_1^2 -2 x_1 \bar{x} + \bar{x}^2 \right) + \left( x_2^2 -2 x_2 \bar{x} + \bar{x}^2 \right) + \ldots + \left( x_n^2 -2 x_n \bar{x} + \bar{x}^2 \right) \right)' \\
&= \left(-2x_1 + 2\bar{x} \right) + \left( -2x_2 + 2\bar{x} \right) + \ldots + \left( -2x_n + 2\bar{x} \right) \\
&= -2x_1 -2x_2 - \ldots - 2x_n + n \left( 2\bar{x} \right) \\
-2n\bar{x} &= -2x_1 - 2x_2 - \ldots - 2x_n \\
\bar{x} &= \frac{-2x_1 - 2x_2 - \ldots - 2x_n}{-2n} \\
&= \frac{x_1+x_2+\ldots+x_n}{n}
\end{aligned}
$$

The arithmetic mean is found at $\mu = \bar{x} = \frac{x_1+x_2+\ldots+x_n}{n}$. $\square$

A less-general demonstration of the above proof is given below in the Wolfram Language.

```
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

## The Four Moments

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
Observe that the calculation for this statistic is similar to the least squares method in section \ref{section:least-squares-method}.
$s^2$ is the average squared distance from the mean within the data set.

The *standard deviation* is the square root of variance,

$$
s = \sqrt{s^2}.
$$

An *outlier* is a value that is very different from other values in the data set.
Let $x = \left\{ 15, 75, 79, 10, 7, 54, \num{4600}, 91, 45, 86 \right\}$.
One can immediately intuit that the value $\num{4600}$ is substantially different from all of the other values.
Outliers can be defined in terms of the mean and standard deviation, where outliers are any values greater than $\bar{x} + 2s$ or less than $\bar{x} - 2s$.
We can use the `mean` and `sd` functions with `subset` in the R language at https://webr.r-wasm.org/latest/ to confirm that $\num{4600}$ is an outlier.

```
> x = c(15, 75, 79, 10, 7, 54, 4600, 91, 45, 86)
> subset(x, x <= mean(x) - sd(x) | x >= mean(x) + sd(x))
[1] 4600
```

We can use skewness ($\beta_1$) to detect whether the data is imbalanced (skewed) above or below the mean. If skewness is negative then the left tail is longer, if skewness is positive then the right tail is longer, and if skewness is zero then the distribution is equally balanced over the mean.
The Excel function for skewness is `SKEW`.

We can use kurtosis ($\beta_2$) to detect if a data set contains outliers.
The kurtosis of the normal distribution is 3.
Karl Pearson defines the *degree of kurtosis* as $\eta = \beta_2 - 3$ [@10.1093/biomet/4.1-2.169, p. 181].
Other texts call this *excess kurtosis*. Excel's `KURT` function returns excess kurtosis, so when `KURT` returns 0 then the distribution may fit the normal and contains no outliers.

## Exponential moving averages

An *exponential moving average* (EMA) is a weighted average that creates a bias favoring recent observations.
EMAs are used in the financial sector as an implicit means of modeling stock prices with time.

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

## Discussion prompts

1. Is four a lot? 

2. First battalion has an average ACFT score of 482 while second battalion has an average ACFT score of 491. Which is better? 

3. What do we do when statistics show us something that contradicts our values? For example, suppose we discover that Soldiers of a specific demographic have much lower promotion rates than their peers. 

4. Is it more important for an organization to think about variance or the 99th percentile? 

5. Given a sample set $x = \left\{ 5 \right\}$, what is the estimate of the mean ($\bar{x}$), and what is the sample variance ($s_x$)?
That is, what is the expected value ($E(x)$) of a random sample taken from $x$, and what is the average difference of variables to the expected value?
Use software to verify your answer. In the R language, this would be `mean(c(5))` and `sd(c(5))`.

## Practical exercises

1. Calculate the influence that outliers have on different-sized datasets that contain outliers. 

2. Calculate the exponential moving average in a small dataset. 

3. Given a dataset and experimental result, identify problems caused by analyzing categorical data represented in a numeric form. 

4. Given multiple datasets with identical mean and standard deviation, use kurtosis to identify the dataset with more outliers. 

5. Design or implement an algorithm to incrementally calculate standard deviation, where the estimate of the sample standard deviation is updated with each additional value. 
