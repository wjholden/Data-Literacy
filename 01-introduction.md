# Introduction

## Parameters and statistics 

Statistics are the foundation of most data mining, machine learning (ML), and artificial intelligence (AI) methods today.
A *statistic* is an estimate of a *parameter*, which is a characteristic of an entire *population*.
Statistics are calculated from taking *samples* (subsets) from the population.

For example, suppose we wanted to find the height of the tallest mountain in the world.
We might sample $n=100$ mountains at random from an almanac.
Suppose the tallest mountain in our sample is Mount Fuji.
Mount Fuji, the tallest mountain in Japan, is 3776 meters tall.
We can conclude that the tallest mountain in the world is *at least* 3776 meters tall.

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
The name of the nation where the movie was filmed is another example of nominal data.
Simple yes and no categories are also nominal data, such as whether a film does or does not have a sequel.

*Ordinal* data has ordering but not distance.
Ordinal data might be represented as ordered categories or as numerals, though these numerals do not provide meaningful addition and subtraction.
The ratings of a film (G, PG, PG-13, R, and so on) form a ranking, but addition is meaningless (does G + PG-13 = R?) and our concept of distance is weak at best.
Another example of ordinal might be the rankings the films receive at an awards ceremony, where one film is the winner and another is the runner-up.

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

Grid coordinates might be another example of interval data.
One can calculate the distance between two grid coordinates, but we would not say that coordinate 1111 is "half" of coordinate 2222.

Data might be represented in numerical formats when some operations do not make sense.
Suppose a political scientist encoded voter's political party as "1", "2", "3", and "4".
Is "2" an intermediate value between "1" and "3", or are these actually nominal data where the only arithmetic operations are $=$ and $\ne$?
AI methods sometimes make incorrect assumptions about data that domain experts can easily prevent.

## Discretization 

Measurements with arbitrarily many decimal digits of precision are *continuous*, whereas measurements with finite steps in between (including categories) are *discrete*.
For example, when driving along a road, the house numbers (150 2nd Street, 152 2nd Street, 154 2nd Street...) are discrete; there is no intermediate value between 150 and 151.
On the other hand, the grid coordinates associated with each address are continuous; one could (theoretically) specify grid coordinates to the nanometer.

It can be useful to combine continuous measurements into discrete categories.
An example might be one's birth date and birth year.
No one knows their birth *instant* with subsecond precision.
Rather, the year, year and month, or year, month, and day are almost almost always enough information.
We even combine years into groups when discussing generations and peer groups.
Combining a range of birth years into generational categories is an example of *discretization*.

## Missing values

In practice, data sets are often missing values.
Different programming languages have substantially different syntax and semantics for representing and handling missing values.

As a small exercise, open Microsoft Excel and enter the values 1, 2, 3, and 5 into cells A1, A2, A3, and A5.
Leave cell A4 blank.
In cell A6, enter the formula `=PRODUCT(A1:A5)`.
The result is $30 = 1 \cdot 2 \cdot 3 \cdot 5$.
Excel did *not* treat the missing value as a zero.

Now change cell A4 to `=NA()`.
`NA` means "value not available", an explicit indication that a value is not given.
The product in cell A6 should update to `#N/A`, which explicitly tells us that there is a problem in the calculation.

Now change cell A4 to `=1/0`.
Both cells A4 and A6 should both say `#DIV/0!`, a fault telling us that a division by zero has made further calculation impossible.

Error values propagate from source data through intermediate calculations to final results.
If we enter a formula into A7 referencing A6, such as `=SQRT(A6)`, then we will find the same faults in A7 that we see in A6.

Structured Query Language (SQL) databases use the symbol `NULL` to denote missing values.
One might build the database *schema* (the structure of the database) to explicitly forbid `NULL` values.
For example, `CREATE TABLE Race (Name TEXT NOT NULL, Time INTEGER NOT NULL)` creates a table of run times where both the name and the time must be specified.

Many programming languages support a `NaN` ("not a number") value in error conditions.
One might encounter `NaN` when dividing by zero, subtracting infinities, and parsing non-numeric words as numbers.
Comparisons with `NaN` can be confusing, such as `NaN == NaN` returning *false*.

Some programming languages will automatically *initialize* variables with some zero value.
Other languages give some `Undefined` value to uninitialized variables.
Still other languages raise an error if no explicit value is assigned to a variable.

## Strong/weak and static/dynamic typing 

Values come in many forms: categorical and numerical, ordered and unordered, discrete and continuous, defined and missing.
*Types* can be used to constrain variables to allowable values and applicable operations.

For example, suppose a database indicates how many cars a person owns.
It makes no sense to own a fractional or negative car, so we might find an existing type
(in this case, whole numbers) or define some new type to model the domain.

Some programming languages offer *dynamic* types that implicitly change the type (*cast*) of values to operate correctly.
In Chrome, Edge Chromium, or Firefox press F12 to open the developer console and enter the following into the JavaScript console:

```
>> "5" * 5
<- 25
```

Characters inside quotation marks (`"5"`) are called *strings* and are ordinarily used for text, but JavaScript automatically parses `"5" * 5` as the product of two numerical values and returns `25`.

JavaScript is notoriously inconsistent.

```
>> "5" + 5
<- "55"
```

The resulting string, `"55"`, is the *concatenation* of two strings -- perhaps not what one expects.

Many languages and environments seek to automatically parse values.
Microsoft Excel and the Python programming language are also dynamic.
Other languages, such as Java and Go, are more strict with values and do not automatically change values, especially when the conversion might be "lossy" (where information might be lost, such as approximating the exact value of $\pi$ as $3.14$, or rounding $3.14$ to $3$, or even changing $3.0$ to $3$).
These languages have both *strong* and *static* typing: the programmer must specify the type of each variable, and lossy type conversions require an explicit cast.

Excel does provide some basic functionality to set number *formats*, but this feature might not stop one from confusing one type of data for another.
Excel uses *weak* typing that does prevent one from using unexpected values.
Data analysts can benefit greatly by using the appropriate types for the values in their problem.

## Tables, lists, and data frames 

*Tables* of data are structured in *columns* and *rows*, where the rows represent the *individuals* in the data set and the columns represent the *features*.
For example, a table of employee names might have two columns (the given and surnames) and ten rows, where each row represents one of the ten employees.

In computer science, the terms *list* and *array* both refer to single-column tables, but with different internal memory representation.
The distinction is usually unimportant to data analysts.

Scientific languages, such as Julia and R, often use the term *data frame* (or *dataframe*).
Data frames often provide rich syntax for row-wise and column-wise operations.
By contrast, in an object-oriented language, such as Java and JavaScript, the idiomatic representation of a table is likely an array of objects.

## Vectors and matrices

We now quickly mention the terms *vector* and *matrix* here to disambiguate them from other terms already defined.

Arrays, lists, and columns containing numeric data may sometimes be represented with *vectors*.
Likewise, tables and data frames might be represented with *matrices*.

A vector is a quantity with both magnitude and direction, often consisting of two or more elements.

$$
\mathbf{x} = \left( x_1 , x_2 , x_3 \right)
$$

The above vector $\mathbf{x}$ has three components and length $\sqrt{x_1 ^2 + x_2^2 + x_3^2}$.

A matrix is a collection of vectors used for linear transformations.
For example, the three-component *identity matrix*

$$
I = \begin{pmatrix}1 & 0 & 0 \\ 0 & 1 & 0 \\ 0 & 0 & 1\end{pmatrix}
$$

has the property

$$
\begin{aligned}
I \mathbf{x} &= \begin{pmatrix}1 & 0 & 0 \\ 0 & 1 & 0 \\ 0 & 0 & 1\end{pmatrix} \begin{pmatrix}x_1 \\ x_2 \\ x_3\end{pmatrix} \\
&= \begin{pmatrix}
1 \cdot x_1 + 0 \cdot x_2 + 0 \cdot x_3 \\ 
0 \cdot x_1 + 1 \cdot x_2 + 0 \cdot x_3 \\
0 \cdot x_1 + 0 \cdot x_2 + 1 \cdot x_3 \\
\end{pmatrix}\\
&= \mathbf{x}
\end{aligned}
$$

Vectors and matrices form the foundations of *linear algebra*, a rich and powerful branch of mathematics that produces many of the results needed for modern statistics, ML, and AI methods.

Remember that the different in ratio and interval data was that *multiplication* is only defined for ratio data.
Similarly, multiplication is well-defined for vectors and matrices, but not on tables of data.
Depending on the problem domain, it may be inappropriate to use matrices and vectors to represent data where such operations are not necessary.

## Box plot, scatter plot, bar plot, and histogram 

## Linear and logarithmic scales 

## Functions 

## Discussion prompts

1. Who owns knowledge management? 

2. What are good and bad uses for spreadsheets? 

3. What is reproducibility and why would this be important for scientific inquiry? 

4. Why is a pie chart not recommended? 

## Practical exercises

1. Given a dataset, plot the data and explain why this plot technique is appropriate. 

2. Given a noisy and poorly structured dataset, propose a method of restructuring the data. 

3. Discretize the values of a dataset and explain the reasoning. 

4. Be creative and construct intentionally misleading plots that deliberately distort information presented.  


