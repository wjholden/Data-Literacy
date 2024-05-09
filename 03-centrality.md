# Measures of Central Tendency

## Arithmetic mean

The canonical definition of the *arithmetic mean* for a set of $n$ numbers $x$ is

$$
\bar{x} = 
\frac{
\sum_{i \in x}{i}
}{n} = 
\frac{x_1 + x_2 + x_3 + \cdots + x_4}{n}.
$$

Where does this definition come from?
Let's open a new workbook in Microsoft Excel and find out.

Leave `A1` blank.
This will become our *estimate of the mean*.
In fields `B1` through `B9`, enter integers 1 through 9.
In fields `C1` through `C9`, enter formulas `=B1-$A$1`, `=B2-$A$1`, `=B3-$A$1`, and so on (keeping `$A$1` fixed).
In fields `D1` through `D9`, enter formulas `=POWER(C1,2)`, `=POWER(C2,2)`, and so on.
In field `E1`, enter formula `=SUM(D1:D9)`.
Finally in field `F1`, enter the formula `=AVERAGE(B1:B9)`.

Now go to Data, What-If Analysis, Goal Seek.
In the Goal Seek dialog, enter `Set cell:` to `E1`, `To value:` to `0`, and `By changing cell:` to `A1`.
Click OK.
The Goal Seek function runs and should produce a value in `A1` near to that in `F1`.
(Goal Seek is not foolproof.
Enter `10` into `A1` to nudge Excel with a hint if you get a ridiculous answer.)

We have used Goal Seek to minimize the *sum of the squared differences* between our values, $x_i$, and our estimate of the mean, $\bar{x}$.

## Mode 

## Median 

## Arithmetic Mean 

## The four moments: mean, variance (and standard deviation), skewness, and kurtosis 

## Exponential moving averages (EMA) 

## Covariance 

## Outliers 

## Unbalanced data sets 

## Discussion prompts

Is four a lot? 

First battalion has an average ACFT score of 482 while second battalion has an average ACFT score of 491. Which is better? 

What do we do when statistics show us something that contradicts our values? For example, suppose we discover that Soldiers of a specific demographic have much lower promotion rates than their peers. 

Is it more important for an organization to think about variance or the 99th percentile? 

Given a sample set [Equation], what is the estimate of the mean ([Equation]), and what is the sample variance? 


## Practical exercises

Calculate the influence that outliers have on different-sized datasets that contain outliers. 

Calculate the exponential moving average in a small dataset. 

Given a dataset and experimental result, identify problems caused by analyzing categorical data represented in a numeric form. 

Given multiple datasets with identical mean and standard deviation, use kurtosis to identify the dataset with more outliers. 

Design or implement an algorithm to incrementally calculate standard deviation, where the estimate of the sample standard deviation is updated with each additional value. 

