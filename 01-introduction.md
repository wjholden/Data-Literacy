# Introduction

## Parameters and statistics 

Statistics are the foundation of most data mining, machine learning (ML), and artificial intelligence (AI) methods today.
A *statistic* is an estimate of a *parameter*, which is a characteristic of an entire *population*.
Statistics are calculated from taking *samples* (subsets) from the population.

For example, suppose we wanted to find the height of the tallest mountain in the world.
We might sample $n=\num{100}$ mountains at random from an almanac.
Suppose the tallest mountain in our sample is Mount Fuji.
Mount Fuji, the tallest mountain in Japan, is $\num{3776}$ meters tall.
We can conclude that the tallest mountain in the world is *at least* $\num{3776}$ meters tall.

Our estimate is unfortunately quite low.
Mount Everest in Nepal, the *highest* mountain in the world, stands $\num{8849}$ meters above sea level.
Mauna Kea in Hawai'i, the *tallest* mountain in the world, stands $\num{4207}$ meters above sea level and another $\num{6004}$ meters below.
Our estimates of population parameters, *statistics*, generally improve with larger sample sizes, and many statistical methods provide a *margin of error* quantifying sampling error.

One might use statistics to create a *model* to explain a population, based upon sampling data.
Models can be useful both for describing the population and also for forming predictions.

## Levels of measurement

There are four distinct *levels of measurement* that a value may fit [@stevens1946theory].
*Nominal* data is simply names or categories, with no concept of order or distance.
A movie might be animated or live-action: these are simple categories or order.
Another example might be the film's genre (children, comedy, action, romance, documentary, etc).

*Ordinal* data has ordering but not distance.
Ordinal data might be represented as ordered categories or as numerals, though these numerals do not provide meaningful addition and subtraction.
The ratings of a film (G, PG, PG-13, R, and so on) form a ranking, but addition is meaningless (does G + PG-13 = R?) and our concept of distance is weak at best.
Another example of ordinal might be the rankings the films receive at an awards ceremony, where one film is the winner and another is the runner-up.

*Interval* data is numerical data with a concept of distance but not multiplication.
The year when a film was produced is an example of interval data.
If two films were produced in 2000 and 2010, then it makes sense to say one was made ten years later, but we would not say that the latter film is $\frac{\num{2010}}{\num{2000}} = 1.005$ times the first.

*Ratio* data is numerical data with both distance and multiplication.
The gross earnings of a film is an example of ratio data.
If the 2000 film earned one million dollars and the 2010 film earned two million dollars, then it makes sense to say the second film earned double the first.

| Name | Operations | Type |
|-|-|-|
| Nominal | $=$, $\ne$ | Categories |
| Ordinal | $<$, $>$ | Ordered categories |
| Interval | $+$, $-$ | Numbers with distance |
| Ratio | $\times$, $\div$ | Numbers with meaningful zero |

Interval data might be initially confusing to distinguish from ratio data.
One indication is the absence of a meaningful zero.
Does zero degrees Celsius or zero degrees Fahrenheit mean the absence of temperature? 
No. These temperature measurements are simply points along a *scale*.
Twenty degrees Celsius is not "twice" ten degrees Celsius; multiplication is not defined on interval data.

Grid coordinates are another example of interval data.
One can calculate the distance between two grid coordinates, but we would not say that coordinate 1111 is "half" of coordinate 2222.

Women's pant sizes in the United States, with the confusing size "00," is yet another example of interval data.

Data might be represented in numerical formats when some operations do not make sense.
Suppose a political scientist encoded voter's political party as "1", "2", "3", and "4".
Is "2" an intermediate value between "1" and "3", or are these actually nominal data where the only arithmetic operations are $=$ and $\ne$?
AI methods may form incorrect assumptions about data that domain experts can easily prevent.

## Discretization {#section:discretize}

Measurements with arbitrarily many decimal digits of precision are *continuous*, whereas measurements with finite steps in between (including categories) are *discrete*.
For example, when driving along a road, the house numbers (150 2nd Street, 152 2nd Street, 154 2nd Street...) are discrete; there is no intermediate value between $\num{150}$ and $\num{151}$.
On the other hand, the grid coordinates associated with each address are continuous; one could (theoretically) specify grid coordinates to the nanometer.

It can be useful to combine continuous measurements into discrete categories.
An example might be one's birth date and birth year.
No one knows their birth *instant* with subsecond precision.
Rather, the year, year and month, or year, month, and day are almost almost always enough information.
We even combine years into groups when discussing generations and peer groups.
Combining a range of birth years into generational categories is an example of *discretization*.

## Missing values

In practice, *data sets* often have missing values.
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
For example, `CREATE TABLE Run (Name TEXT NOT NULL, Time INTEGER NOT NULL, Distance REAL NOT NULL)` defines a table *schema* where each of the three columns must be specified.
Many programming languages (including C, Java, and JavaScript) also use the term `null` for variables that do not reference any specific value.

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
Go to https://jsfiddle.net or press F12 to open the developer console in most modern browsers.
Enter the following into the JavaScript console:

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

<!-- https://drops.dagstuhl.de/storage/01oasics/oasics-vol076-plateau2019/OASIcs.PLATEAU.2019.6/OASIcs.PLATEAU.2019.6.pdf -->

*Tables* of data are structured in *columns* and *rows*, where the rows represent the *individuals* or *observations* in the data set and the columns represent the *features*.
For example, a table of employee names might have two columns (the given and surnames) and ten rows, where each row represents one of the ten employees.

In computer science, the terms *list* and *array* both refer to single-column tables, but with different internal memory representation.
The distinction is usually unimportant to data analysts.

Scientific languages, such as Julia and R, often use the term *data frame* (or *dataframe*) as their method for representing tables of data.
Data frames often provide rich syntax for row-wise and column-wise operations.
By contrast, in an object-oriented language, such as Java and JavaScript, the idiomatic representation of a table is likely an array of objects.

<!-- ## Forms and input validation -->



## Vectors and matrices

We now quickly mention the terms *vector* and *matrix* here to disambiguate them from other terms already defined.

Arrays, lists, and columns containing numeric data may sometimes be represented with *vectors*.
Likewise, tables and data frames might be represented with *matrices*.

A vector is a quantity with both magnitude and direction, often consisting of two or more elements.

$$
\mathbf{x} = \left( x_1 , x_2 , x_3 , \cdots , x_n \right)
$$

The above vector $\mathbf{x}$ has three components and length $\sqrt{x_1 ^2 + x_2^2 + x_3^2 + \cdots + x_n^2}$.

A matrix is a collection of vectors used for linear transformations.
For example, the three-component *identity matrix*

$$
I_3 = \begin{pmatrix}1 & 0 & 0 \\ 0 & 1 & 0 \\ 0 & 0 & 1\end{pmatrix}
$$

has the property

$$
\begin{aligned}
I_3 \mathbf{x} &= \begin{pmatrix}1 & 0 & 0 \\ 0 & 1 & 0 \\ 0 & 0 & 1\end{pmatrix} \begin{pmatrix}x_1 \\ x_2 \\ x_3\end{pmatrix} \\
&= \begin{pmatrix}
1 \cdot x_1 + 0 \cdot x_2 + 0 \cdot x_3 \\ 
0 \cdot x_1 + 1 \cdot x_2 + 0 \cdot x_3 \\
0 \cdot x_1 + 0 \cdot x_2 + 1 \cdot x_3 \\
\end{pmatrix}\\
&= \mathbf{x}.
\end{aligned}
$$

Vectors and matrices form the foundations of *linear algebra*, a rich and powerful branch of mathematics that produces many of the results needed for modern statistics, ML, and AI methods.

Remember that the different in ratio and interval data was that *multiplication* is only defined for ratio data.
Similarly, multiplication is well-defined for vectors and matrices, but not on tables of data.
Depending on the problem domain, it may be inappropriate to use matrices and vectors to represent data where such operations are not necessary.

## Data visualization with plots

*Plots* allow us to visualize data.
Good plots help us to quickly intuit patterns in the data that might otherwise be difficult to understand.

(Note: the term *graph* has different definitions in lower and higher mathematics.
We will explain the term "graph" in chapter \ref{chapter:graph}.
This text uses the term "plot" as the verb and noun for visualizing data with graphics.)

The *bar plot* helps us to compare the count each category in a discrete (or discretized) variable.
The *box plot* helps us to see the center and variation of a numerical variable.
The *histogram* also helps us to see the center and variation of a numerical variable, often producing the familiar *bell curve* shape, where the height of the curve indicates the count of observations within the range of each "bin."
A histogram is essentially a set of bar plots over discretized numerical values.

A *scatter plot* (sometimes called an $XY$ plot) uses $x$ and $y$ axes to show relationships between two variables.
One can also color and shape the points to show third and fourth variables.
Three-dimensional $XYZ$ plots are sometimes useful, especially in video and interactive presentations.

As a small exercise to experiment with these four plots, go to https://webr.r-wasm.org/latest/ to use the R language in a web browser.
R is a programming language for statistics and data visualization.

R includes several built-in data sets.
In the *read-evaluate-print loop* (*REPL*), enter

```
> head(mtcars)
```

to view the column names and first six rows of the Motor Trend Cars (`mtcars`) data set.
Now enter the following commands to quickly visualize a few columns in the data set.

```
> barplot(mtcars$cyl)
> boxplot(mtcars$mpg)
> hist(mtcars$mpg)
> plot(mtcars$wt, mtcars$mpg)
```

## Linear and logarithmic scales

Scientists use the term *order of magnitude* to compare values only by the power of $10$.
One would say $a = 1.6 \times 10^{3}$ is three orders of magnitude smaller than $b = 8.3 \times 10^{6}$,
which is to say $b/a \approx \num{1000}$.

The *scale* of an axis, such as in bar plot, is the spacing between values.
A *linear scale* might show marks at 10, 20, 30, 40, and so on.
A *logarithmic scale* might show marks at 10, 100, $\num{1000}$, $\num{10000}$, and so on.

Logarithmic scales can be useful for comparing values that differ by more than one order of magnitude.
For example, suppose feature of a data set contains categories $a$, $b$, $c$, and $d$, and the count of each category is

| Category | Count        |
|----------|--------------|
| $a$      | \num{10736} |
| $b$      | \num{1711}  |
| $c$      | \num{398}   |
| $d$      | \num{319}   |

Return to https://webr.r-wasm.org/latest/ and plot this data with linear and logarithmic scales:

```
> category_counts <- c(10736, 1711, 398, 319)
> category_counts
[1] 10736  1711   398   319
> barplot(category_counts)
> barplot(category_counts, log="y")
```

## Sets, relations, functions, and algorithms {#section:discrete-math}

We now introduce a few terms from *discrete mathematics* that are fundamental to all analysis.
A *set* is an unordered collection of *distinct* elements.
Sets may be finite or infinite in size.
Sets are denoted with curly braces, and the empty set has the special symbol $\emptyset = \left\{ \right\}$.
An example of a set might be

$$
W = \left\{
\textrm{Sunday},
\textrm{Monday},
\textrm{Tuesday},
\textrm{Wednesday},
\textrm{Thursday},
\textrm{Friday},
\textrm{Saturday}
\right\}
$$

A *relation* is an association between members of sets.
Relations can be used to model any relationship between members any two sets, or even members in the same set.
An example might be the relation between integers and elements of $W$ with that many letters, i.e. 6 has a relation on Sunday, Monday, and Friday, 7 has a relation on Tuesday, 8 has a relation on Thursday and Saturday, and 9 has a relation on Wednesday.
The term "relation" is seldom used outside of discrete mathematics, but there is a *special case* of a relation that occurs in all mathematical disciplines: *functions*.

A *function* is a relation that uniquely relates members of one set (the *domain*) to another set (the *range*).
An example of some functions might be:

$$
\begin{aligned}
\textrm{Translate} \left( \textrm{Friday}, \textrm{English}, \textrm{German} \right) &= \textrm{Freitag} \\
\textrm{Length} \left( \textrm{Wednesday} \right) &= 9 \\
\textrm{Distance} \left( \textrm{Thursday} , \textrm{Tuesday} \right) &= -2 \\
\textrm{DaysOfLength} \left( 6 \right) &= \left\{ \textrm{Sunday} , \textrm{Monday} , \textrm{Friday} \right\} \\
\textrm{Sunday} &= \textrm{Next} \left( \textrm{Saturday} \right) \\
&= \textrm{Previous} \left( \textrm{Monday} \right) \\
&= \textrm{Previous} \left( \textrm{Previous} \left( \textrm{Tuesday} \right) \right) \\
&= \textrm{Previous} \left( \textrm{Next} \left( \textrm{Sunday} \right) \right)
\end{aligned}
$$

Each of these functions accepts one or more *parameters* as *arguments* and returns the unique corresponding value (if any) from its range.
It may appear that the third function, DaysOfLength, has returned three values, but actually this function has returned a single set which contains three values.

Many programming languages use the term "function" as a synonym for *procedure*, *subroutine*, and *method*.
Functions are "pure" if they have no side-effects, such as mutating a value outside of the function.

The mathematical definition of the term *algorithm* is the set of instructions necessary to solve a problem.
Long division, a procedure for manually dividing numbers, is an example of an algorithm.
The term "algorithm" has recently entered the popular lexicon in relation to AI systems.
Here, the instructions of the algorithm are part of a model, which is created from data.

<!-- ## Bisection algorithm (todo) -->

## Abstraction

Take any three-digit decimal (base 10) number, reverse the digits, and their difference will always be divisible by both 9 and 11.
For example, $321-123=198$; $198 \div 9 = 22$ and $198 \div 11 = 18$.

How and why would this strange property hold?
The proof is quite easy using algebra. We change the digits of a three-digit number into variables.
Some three-digit number $abc = 100a + 10b + c$.

$$
\begin{aligned}
abc - cba &= (100a + 10b + c) - (100c + 10b + a) \\
&= 100a + 10b + c - 100c - 10b - a \\
&= 99a - 99c \\
&= 99(a - c) \square
\end{aligned}
$$

By *abstracting* numerals into variables, the claim becomes easy to verify.

Here is another example of abstraction.
How does one calculate 30% of 70 without a calculator?
First, observe that

$$
x \% \text{ of } y = \frac{x \times y}{100}.
$$

So, 30% of 70 is $\frac{30 \times 70}{100} = \frac{\num{2100}}{100} = 21$.

Abstraction can be a powerful tool for solving problems and developing proofs.
In the field of computer networking, countless problems are solved by the pattern, "we have more than one thing, but it is inconvenient to operate more than one of these things, so we built a method to abstractly represent arbitrarily many of these things as super-things."

## Discussion prompts

1. Who owns knowledge management? 

2. What are good and bad uses for spreadsheets? 

3. What is reproducibility? Why would this be important for scientific inquiry? 

4. Like a barplot, a pie chart shows the relative sizes of categorical values.
What are some advantages and disadvantages of using pie charts?

5. A manager sends an Excel spreadsheet to their employees, telling them to each
enter information and send it back. What are some challenges the manager might
experience while merging these spreadsheets?

## Practical exercises

1. Create a small survey using Microsoft Forms (part of Office 365) or Google Forms (part of Google Docs).
Compare this experience to the hypothetical manager who gathered information by
manually merging spreadsheets.

2. Given a dataset, plot the data and explain why this plot technique is appropriate. 

3. Given a noisy and poorly structured dataset, propose a method of restructuring the data. 

4. Discretize the values of a dataset and explain the reasoning. 

5. Be creative and construct intentionally misleading plots that deliberately distort information presented.  

