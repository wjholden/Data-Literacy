# Introduction

## Parameters and statistics 

Statistics are the foundation of most data mining, machine learning (ML), and artificial intelligence (AI) methods today.
A *statistic* is an estimate of a *parameter*, which is a characteristic of an entire *population*.
Statistics are calculated from taking *samples* (subsets) from the population.

For example, suppose we wanted to find the height of the tallest mountain in the world.
We might sample $n=100$ mountains at random from an almanac.
Suppose the tallest mountain in our almanac is Mount Fuji, the tallest mountain in Japan, which is 3776 meters tall.
We can safely conclude that the tallest mountain is *at least* 3776 meters tall.

Our estimate is unfortunately quite low.
Mount Everest in Nepal, the *highest* mountain in the world, stands 8849 meters above sea level.
Mauna Kea in Hawai'i, the *tallest* mountain in the world, stands 4207 meters above sea level and another 6004 meters below.
Our estimates of population parameters, *statistics*, generally improve with larger sample sizes, and many statistical methods provide a *margin of error* quantifying sampling error.

One might use statistics to create a *model* to explain a population, based upon sampling data.
Models can be useful both for describing the population and also for forming predictions.

## Nominal, ordinal, interval, and ratio variables 

There are several classes of data that a variable might fit into.
*Nominal* data is simply names or categories, with no concept of order or distance.
A movie might be animated or live-action: these are simple categories or order.

*Ordinal* data has some concept of total or partial ordering.
The ratings of a film (G, PG, PG-13, R, and so on) form a ranking, but addition is meaningless (does G + PG-13 = R?) and our concept of distance is weak at best.

*Interval* data is numerical data with a concept of distance but not multiplication.
The year when a film was produced is an example of interval data.
If two films were produced in 2000 and 2010, then it makes sense to say one was made ten years later, but we would not say that the latter film is $2010/2000 = 1.005$ times the first.

*Ratio* data is numerical data with both distance and multiplication.
The gross earnings of a film is an example of ratio data.
If the 2000 film earned one million dollars and the second earned two million dollars, then it makes sense to say the second film earned double the first.

| Name | Operations | Type |
|-|-|-|
| Nominal | $=$, $\ne$ | Categories |
| Ordinal | $<$, $>$ | Ordered categories |
| Interval | $+$, $-$ | Numbers with distance |
| Ratio | $\times$, $\div$ | Numbers with meaningful zero |

Interval data might be initially confusing to distinguish from ratio data.
One indication is the absence of a meaningful zero.
Does zero degrees Celsius or Fahrenheit mean the absence of temperature? 
No, these measurements are simply points along a scale.
Twenty degrees Celsius is not "twice" ten degrees Celsius; multiplication is not defined on interval data.

Data might be represented in numerical formats when some operations do not make sense.
Suppose a political scientist encoded voter's political party as "1", "2", "3", and "4".
Is "2" an intermediate value between "1" and "3", or are these actually nominal data where the only arithmetic operations are $=$ and $\ne$?
AI methods sometimes make incorrect assumptions about data that domain experts can easily prevent.

## Discretization 



## Missing values 

## Strong/weak and static/dynamic typing 

## Columns and rows 

## Features and individuals 

## Tables, lists, and data frames 

## Vectors and matrices

## Box plot, scatter plot, bar plot, and histogram 

## Linear and logarithmic scales 

## Functions 

## Discussion prompts

Who owns knowledge management? 

What are good and bad uses for spreadsheets? 

What is reproducibility and why would this be important for scientific inquiry? 

Why is a pie chart not recommended? 

## Practical exercises

Given a dataset, plot the data and explain why this plot technique is appropriate. 

Given a noisy and poorly structured dataset, propose a method of restructuring the data. 

Discretize the values of a dataset and explain the reasoning. 

Be creative and construct intentionally misleading plots that deliberately distort information presented.  


