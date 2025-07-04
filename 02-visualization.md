# Data Visualization {#sec:visualization}

## Plots {#sec:plots}

*Plots* allow us to visualize data.
Good plots help us to quickly intuit patterns in the data that might otherwise be difficult to understand.

The term *graph* has different definitions in lower and higher mathematics.
We will explain the term "graph" in chapter \ref{chapter:graph}.
This text uses the term "plot" as the verb and noun for visualizing data with graphics.

The *bar plot* helps us to compare the count each category in a discrete (or discretized) variable.
The *box plot* helps us to see the center and variation of a numerical variable.
The *histogram* also helps us to see the center and variation of a numerical variable, often producing the familiar *bell curve* shape, where the height of the curve indicates the count of observations within the range of each "bin."
A histogram is essentially a set of bar plots over discretized numerical values.

A *scatter plot* (sometimes called an $XY$ plot) uses $x$ and $y$ axes to show relationships between two variables.
One can also color and shape the points to show third and fourth variables.
Three-dimensional $XYZ$ plots are sometimes useful, especially in video and interactive presentations.

We will often refer to the Motor Trend Cars (`mtcars`) data set, which is included
in the R language^[<https://webr.r-wasm.org/latest/>]. One can begin exploring
the  `mtcars` data set in R's *read-evaluate-print loop* (*REPL*) with command 

```r
> head(mtcars)
```
Enter

```r
> mtcars
```

to view the full data set.

Place a question mark before a function or data set name in the REPL to get help in R.
Try opening the R help for `mtcars` and `head` with the following commands:

```r
> ?mtcars
> ?head
```

### Line Plots {#sec:lineplot}

The *line plot* is among the most basic of plots. We seldom see one-dimensional
number plots in the sciences, but they are used extensively in elementary
education to develop numerical intuition in children. It can also be useful to
draw line plots to represent *continuums* of ordered data, including discretized
categories. Figure \ref{fig:lineplot-water} shows a small example of a line plot
of the relative size of bodies of water.

\begin{figure}
\centering
\includegraphics{fig/lineplot-water.tikz}
\caption{Line plots can depict the total order of ordinal data.}
\label{fig:lineplot-water}
\end{figure}

Let us quickly address a technical matter on the order of sets.
This will help us to avoid making unsafe assumptions about our data.
In section \ref{sec:levels}, we said that ordinal data supports tests of
equality and order but neither distance nor scale. However, there are many forms
of order. Two important orderings are *partial* and *total* orders. The relation
$\le$ forms a partial order on sets that are are *reflexive*, *anti-symmetric*,
and *transitive*.

1. The reflexive property means that $x \le x$.

2. The anti-symmetric property means that if $x \le y$ and $y \le x$ then $x = y$.
In other words, it is not possible for both $x < y$ and $y < x$.

3. The transitivity property means that if $x \le y$ and $y \le z$ then $x \le z$.

To form a total order, the data needs to allow one more invariant: *comparability*.

4. The comparability property means that for all possible $x$ and $y$, $x \le y$ or $y \le x$.

The integers provide all four properties and therefore form a total order.
It might not be obvious, but we have already discussed a situation where a
special value is not orderable: NaN, the "not a number" value, and missing values
that we saw in section \ref{sec:nan}.

*Periodic values* are an interesting case of ordinal data that does not form
a partial order [@Kantardzic, p. 35]. Some examples are:

- Days of the week.
- The paper, rock, scissors game.
- Angular measurements, such as around a unit circle (is $2^{\circ} \le 361^{\circ}$?) or lines of longitude around the equator.

Some cultures begin the week with Sunday, others Monday, and this convention is
purely cultural. If plotting periodic values, one must choose some order based
on similar conventions.

### Scatter Plots {#sec:scatter}

*Scatter plots* depict data in two or three spatial dimensions. A scatter plot
of two dimensions, conventionally $x$ for the horizontal axis and $y$ for
the vertical axis, the flat space is called a *Cartesian plane*.
An example scatter plot is shown in figure \ref{fig:mtcars-scatter}.
Use the R commands `plot(mtcars$mpg, mtcars$wt)` and `plot(mtcars$mpg ~ mtcars$wt)`
to recreate this plot^[<https://webr.r-wasm.org/latest/>].

\begin{figure}
\centering
\includegraphics{fig/mtcars-scatter.tikz}
\caption{A scatter plot of car efficiency and weight from the Motor Trend Cars data set. The color and shape of the points indicate the number of cylinders (4, 6, or 8).}
\label{fig:mtcars-scatter}
\end{figure}

It is possible to plot in three dimensions, but these graphics can be difficult
to understand in printed format and can be more compelling in animated videos
or in an interactive setting. The Wolfram language provides many 3D 
visualization features^[<https://reference.wolfram.com/language/howto/PlotDataIn3D.html>].

In mathematics, we often visualize the relation of functions with lines drawn
against the same Cartesian plane. In a sense, these graphics are also scatter
plots, but with a large number of samples taken from a function. Shading under
curves can be useful to explain the accumulation of values over ranges.

<!--
adf[age_] := -age/5000 + 1/50

Integrate[adf[x], {x, 0, 100}]

Table[100*NIntegrate[adf[age], {age, x, x + 1}], {x, 0, 99}]
-->

\begin{figure}
\centering
\includegraphics{fig/scatter-age-density.tikz}
\caption{Suppose, for the sake of this simple example, that the density of ages
in our human population linearly decreases until there are none of us left at
age 100. 1.99\% of us are younger than 1, 1.97\% are between 1 and 2,
1.95\% between 2 and 3, and so on until only 0.01\% are between 99 and 100.
The total area of the curve is 1, representing 100\% of all people.
The blue shaded area answers a child's question: why are there more adults
than children?}
\label{fig:scatter-age-density}
\end{figure}

Scatter plots are typically used to visualize data, but sometimes trends are
so compelling that we can gather useful information directly. In figure
\ref{fig:mtcars-scatter}, it is quite clear that four-cylinder cars are both
lighter and more efficient than eight-cylinder cars. Figure \ref{fig:scatter-age-density}
illustrates how the area under a curve is a powerful aggregation function.

### Bar Plots {#sec:barplot}

*Bar plots* relate categories to aggregated numerical features of a data set.
The categorical group is the *independent* variable.
The numerical feature, plotted as the length of the bars, is the *dependent* varible.
Independent and dependent variables are sometimes called *free* and *response*
variables. In an *interventional* study (where a researcher performs an action to
quantify the effect), the independent variable is the item changed directly and
the dependent variable is the outcome caused by the change. Bar plots almost
always require some amount of "data wrangling," such as the use of SQL aggregate
functions (more on this in section \ref{sec:grouping-and-aggregation}) such as
`MIN()`, `MAX()`, `COUNT()`, `SUM()`, and `AVG()`. Figure \ref{fig:barplot}
demonstrates a bar plot.

\begin{figure}
\centering
\includegraphics{fig/barplot.tikz}
\caption{A bar plot showing the Gross Domestic Product (GDP) of the United States from 2019--2023, according to <https://tradingeconomics.com/united-states/gdp>.
Bar plots summarize information by representing aggregates (such as sums) as functions of categories. Here, years are treated as a categorical variable.}
\label{fig:barplot}
\end{figure}

The width of each bar must be uniform. Only the bar height varies.
As $\text{Area} = \text{Width} \times \text{Height},$ the exaggerated area of a
wide or thin bar will mislead the reader.
For example, suppose a bar plot is intended to compare the values $x = h = \left( 3, 10, 11 \right)$,
but the bars corresponding to each observation are, respectively, $w = \left( 1, 1, 3 \right)$.
The resulting areas are $w \odot h = \left( 3, 10, 22 \right)$ (here, $\odot$ 
indicates the *element-wise* product of two vectors, also known as a *Hadamard*
product). As shown in figure \ref{fig:misleading-barplot}, the area of the third
bar is more than triple that of the second and may confuse the reader.

\begin{figure}
\centering
\includegraphics{fig/misleading-barplot.tikz}
\caption{The bars of a bar plot should ordinarily have uniform width.
This bar plot shows values $x = \left( 3, 10, 11 \right)$, but the
width of the third bar makes this observation appear much larger than the others.}
\label{fig:misleading-barplot}
\end{figure}

In the R language, one can create bar plots using the `barplot`. Using 
<https://webr.r-wasm.org/latest/>, recreate the figure in \ref {fig:barplot}:

```r
> gdp = data.frame(Year = 2019:2023, GDP=c(21.5,21.4,23.7,26,27.7))
> gdp
  Year  GDP
1 2019 21.5
2 2020 21.4
3 2021 23.7
4 2022 26.0
5 2023 27.7
> barplot(GDP ~ Year, gdp)
> library(tidyverse)
> ggplot(gdp, aes(x=Year, y=GDP)) + geom_col()
```

### Pareto Charts {#sec:pareto-chart}

A *Pareto chart* is a combination of a sorted box plot and line plot.
Pareto charts are frequently used in industrial settings to show cumulative
proportions among categories. A typical application for a Pareto chart is to
triage the most common causes for a problem to maximize effectiveness of
finite resources. Pareto charts are closely associated with the "Pareto Principle,"
an estimation that 80% of problems are caused by 20% of causes.

A 2014 observational study by Weisenthal et al.\ surveyed injury rates among
CrossFit athletes [@Weisenthal2014, p. 7]. The researchers report a total of 84
injuries in six movement categories, which are shown in the following table.
The movement types are presented in descending order by injury count.
The proportion is simply the injury count for the current row divided by the
total of injury counts. The cumulative sum, `cumsum` in the R language, is the
sum of the current injury proportion and those before.

| Movement Type   | Injury Count | Proportion | Cumulative Sum |
|-----------------|--------------|------------|----------------|
| Power Lifting   | 19           | 22.619%    | 22.619%        |
| Gymnastics      | 17           | 20.238%    | 42.857%        |
| Not Associated  | 16           | 19.048%    | 61.905%        |
| Olympic Lifting | 14           | 16.667%    | 78.572%        |
| Other           | 13           | 15.476%    | 94.048%        |
| Endurance       | 5            | 5.952%     | 100.000%       |

A Pareto chart for this table is shown in figure \ref{fig:pareto-injuries}.

\begin{figure}
\centering
\includegraphics{fig/pareto-injuries.tikz}
\caption{A Pareto chart shows the relative and cumulative proportions of discretized quantities, sorted in decreasing incidence. Frequently used in quality control processes, such as Lean Six Sigma, Pareto charts may show that only one or a few causes lead to a significant proportion of problems.}
\label{fig:pareto-injuries}
\end{figure}

R does not provide a native method to produce Pareto charts, but we can do so
ourselves with the `ggplot2` library^[<https://ggplot2.tidyverse.org>]. First,
we load the `tidyverse` library^[<https://www.tidyverse.org>], which will also
include the `dplyr` package^[<https://dplyr.tidyverse.org>]. `dplyr` provides
the `%>%` infix operator, which anonymously "pipes" the output from one function
into the first argument of the next function. We tally each distinct observation,
sort them in descending order by count, compute the proportion of the total,
and compute the cumulative sum. Finally, we plot the proportions as a box plot
and overlay the cumulative sums as a line plot.

This example uses data gathered from 
CrossFit.com^[<https://games.crossfit.com/article/complete-list-athletes-currently-serving-sanctions>].
`dplyr` uses `mutate` as row-wise `map` operation with support for aggregate
functions (such as `sum(n)` below; see also section \ref{sec:grouping-and-aggregation}).

```r
> library(tidyverse)
> v = c("GW1516", "GW1516", "Methenolone", "Meldonium", "GW1516", 
  "GW1516", "Oxandrolene", "GW1516", "GW1516", "Clomiphene", "Clomiphene",
  "GW1516", "Turinabol", "GW1516", "GW1516", "GW1516", "RAD140", "GW1516",
  "GW1516", "Stanozolol", "Drostanolone", "GW1516", "Clomiphene",
  "GW1516", "GW1516", "Ostarine", "S-23", "GW1516", "Clomiphene", "GW1516",
  "Meldonium", "GW1516", "GW1516", "5aAdiol", "Stanozolol", "Testosterone", 
  "Drostanolone", "GW1516", "GW1516", "Metenolone", "GW1516", "Boldenone", 
  "GW1516", "GW1516", "GW1516")
> df = data.frame(Violation = v) %>%
  count(Violation) %>%
  arrange(-n) %>%
  mutate(Proportion = 100.0 * n / sum(n)) %>%
  select(Violation, Proportion) %>%
  mutate(CumSum = cumsum(Proportion))
> df
      Violation Proportion    CumSum
1        GW1516  55.555556  55.55556
2    Clomiphene   8.888889  64.44444
3  Drostanolone   4.444444  68.88889
4     Meldonium   4.444444  73.33333
5    Stanozolol   4.444444  77.77778
6       5aAdiol   2.222222  80.00000
7     Boldenone   2.222222  82.22222
8    Metenolone   2.222222  84.44444
9   Methenolone   2.222222  86.66667
10     Ostarine   2.222222  88.88889
11  Oxandrolene   2.222222  91.11111
12       RAD140   2.222222  93.33333
13         S-23   2.222222  95.55556
14 Testosterone   2.222222  97.77778
15    Turinabol   2.222222 100.00000
> df %>% ggplot(aes(x = reorder(Violation, -Proportion))) +
	geom_bar(aes(weight = Proportion)) +
	geom_line(aes(y = CumSum, group=1)) +
	xlab("Drug Violation") +
	ylab("Proportion")
```

### Box Plots {#sec:boxplot}

\begin{figure}
\centering
\includegraphics{fig/mtcars-boxplot.tikz}
\caption{A box plot of the fuel efficiency in the Motor Trend Cars (mtcars) data set.
Box plots are centered at the median of the data. Outliers may be shown as dots
or circles beyond the 0th and 100th percentile markers.}
\label{fig:mtcars-boxplot}
\end{figure}

A box plot splits data into *quartiles*, where each quartile contains 25% of the
observations, and represents the spread of each quartile with a "box and whisker."
Box plots are only useful with numerical data. The box is centered at the *median*
(sequentially middle) value.

An *interquartile range* (IQR) is the values at the 25th and 75th *percentile*
of the values. In the Motor Trend Cars data set there are $n=32$ values.
The first quartile is the first eight values, the second quartile is the second
eight values, the third quartile is the third eight values, and the fourth
quartile is the remaining eight values. If there are an even number of values,
then the boundary between *quantiles* (the specific values marking the boundaries
of the quartiles; note the difference in spelling) is taken from the midpoint.

```r
> sort(mtcars$mpg)[8:9]
[1] 15.2 15.5
> sort(mtcars$mpg)[24:25]
[1] 22.8 22.8
> quantile(mtcars$mpg)
    0%    25%    50%    75%   100%
10.400 15.425 19.200 22.800 33.900
> 22.800-15.425
[1] 7.375
> IQR(mtcars$mpg)
[1] 7.375
```

When using a boxplot, we traditionally define *outliers*^[We will see an
alternative definition for outliers in section \ref{sec:moments}.] as any value
that is $1.5$ IQRs below the first quantile or $1.5$ IQRs above the third quantile.

$$
\text{Outliers} \left( X \right) =
\left\{ 
    x \in X |
    x < \text{Q1}-1.5\text{IQR} \lor x > \text{Q3}+1.5\text{IQR}
\right\}
$$

By this definition, the `mtcars` data set contains one outlier in the `mpg`
(miles per gallon, a measure of fuel efficiency) column.

```r
> q1 = quantile(mtcars$mpg)[2]
> q3 = quantile(mtcars$mpg)[4]
> iqr = IQR(mtcars$mpg)
> subset(mtcars, mpg < q1 - 1.5*iqr | mpg > q3 + 1.5*iqr)
                mpg cyl disp hp drat    wt qsec vs am gear carb
Toyota Corolla 33.9   4 71.1 65 4.22 1.835 19.9  1  1    4    1
```

R provides `boxplot` function to render box plots^[<https://webr.r-wasm.org/latest/>].
Recreate the box plot shown in figure \ref{fig:mtcars-boxplot} with `boxplot(mtcars$mpg)`.

### Histograms {#sec:histogram}

\begin{figure}
\centering
\includegraphics{fig/mtcars-hist.tikz}
\caption{A histogram of the fuel efficiency in the Motor Trend Cars (mtcars) data set,
the same data shown in figure \ref{fig:mtcars-boxplot}.
This histogram uses 5 bins.}
\label{fig:mtcars-hist}
\end{figure}

A *histogram* is a special case of a bar plot. Samples are aggregated into
intervals, known as *bins*, and counted. Bars representing the size of each bin
are shown in order to visualize the *distribution* of the data set.

Some distributions form the famous "bell curve" shape when plotted as a histogram.
The majority of the data fits in a few bins at the center, which quickly tapers
in the bins away from the center, and gradually reducing to thin "tails" as the
extremes far from the center.

In the R language, we can easily create histograms with the `hist` function,
such as `hist(mtcars$mpg)`.

### Heat Maps {#sec:heatmap}

A *heat map* (sometimes written *heatmap*) is a plot that uses a continuous range
of colors to overlay values onto a two-dimensional plot. Heat maps are especially
useful for overlaying information onto geographical maps.
Figure \ref{fig:military-expenditure-europe} demonstrates a map of Europe where
countries are colored by military expenditures as a fraction of their economies.

![A heat map showing military expenditures in European countries. This plot was created in Wolfram Mathematica with input `GeoRegionValuePlot[Flatten[{CountryData[#, "Polygon"] -> CountryData[#, "MilitaryExpenditureFraction"]} & /@ CountryData["Europe"]]]`.](MilitaryExpenditureFraction.pdf){#fig:military-expenditure-europe}

## Scales

In this section, we will learn to adjust the scale of a plot to better 
understand the data it represents. Scaling a plot can allow us to compare values
that are of substantially different size. The scale can also serve as a useful
tool to discover relationships, especially exponential relationships. We will
continue our exploration of logarithms and exponentiation with an introduction
to the sigmoid and logistic curves.

### Linear and logarithmic scales {#sec:scales}

Scientists use the term *order of magnitude* to compare values only by the power of $10$.
One would say $a = 1.6 \times 10^{3}$ is three orders of magnitude smaller than $b = 8.3 \times 10^{6}$,
which is to say $b/a \approx \num{1000}$.

The *scale* of an axis, such as in bar plot, is the spacing between values.
A *linear scale* might show marks at 10, 20, 30, 40, and so on.
A *logarithmic scale* might show marks at 10, 100, $\num{1000}$, $\num{10000}$, and so on.

\begin{figure}
\centering
\includegraphics{fig/scale-linear.tikz}
\caption{Are the smallest two values equal? It is difficult to tell in this plot.
On a linear scale, the smaller values are so completely dwarfed by the
maxima that they are difficult to distinguish.}
\label{fig:linear-scale}
\end{figure}

Logarithmic scales can be useful for comparing values that differ by more than one order of magnitude.
For example, let

$$
x = \left( \num{10736}, \num{5564}, \num{1711}, 398, 319, 60, 44, 29, 21 \right).
$$

Compare the bar plots of $x$ in figures \ref{fig:linear-scale} and \ref{fig:log-scale}.

\begin{figure}
\centering
\includegraphics{fig/scale-log.tikz}
\caption{Are the smallest two values equal? Clearly not. On a logarithmic scale,
we can compare values that differ by orders of magnitude.}
\label{fig:log-scale}
\end{figure}

These two bar plots show the same data using different scales. The
linear scale draws tick marks with a constant *additive*
distance. The logarithmic scale spaces tick marks with
by constant *multiplicative* change. Observe that, in both directions, values
that appear close on one plot appear distant on the other.

Return to <https://webr.r-wasm.org/latest/> and plot this data with linear and logarithmic scales:

```
> x <- c(10736, 5564, 1711, 398, 319, 60, 44, 29, 21)
> barplot(x)
> barplot(x, log="y")
```

### Logarithms and exponentiation {#sec:log_exp}

A logarithm is the inverse of exponentiation. If 

$$
a^b = \underbrace{a \times a \times a \times \cdots \times a}_{b \text{ terms of } a} = c,
$$

then 

$$
\log_a c = b.
$$

In this case, $a$ is the *base* of the logarithmic, and we read $\log_a c$ as
"the log, base $a$, of $c$."

Euler's constant, $e \approx 2.718281828459045$, is frequently associated with
exponential and logarithmic functions. When the base of a logarithm is equal to
$e$, we call this the *natural logarithm* and express it as

$$
\ln x = \log_e x.
$$

Plots of $e^x$ and $\ln x$ are shown in figure \ref{fig:exp}.

\begin{figure}
\centering
\includegraphics{fig/exp.tikz}
\caption{The exponential function, $e^x$, models iterated multiplication and grows quickly.
The domain (allowable inputs) for $e^x$ are all real numbers, but the range (possible outputs) are strictly positive reals.
It is not possible for $e^x$ to produce a zero or negative output if $x$ is a real number.
The logarithmic function, $\ln x$, inverts exponentiation and grows very slowly.
The domain and range for $\ln x$ are the reverse of $e^x$: the domain of $\ln x$ is the positive reals and range is all reals.}
\label{fig:exp}
\end{figure}

### Relationships

Figure \ref{fig:log_exp} shows exponential and quadratic functions $e^x$ and $x^2$ on logarithmic scales.
The exponential function forms a straight line when plotted this way, but the
quadratic does not. This is because $\ln e^x = x$, but $\ln x^2 = 2 \ln x$, which
means the plot of $x^2$ reveals the same slow growth that we saw in figure \ref{fig:exp}.
If the exponential function had a base $b$ other than $e$, then

$$
\ln b^x = x \ln b,
$$

where $\ln b$ is a constant factor and observable as the slope of the resulting plot.

\begin{figure}
\centering
\includegraphics{fig/log_exp.tikz}
\caption{The exponential function, $e^x$, forms a straight line when plotted on a logarithmic scale, as $\ln {e^x} = x$.
By contrast, the quadratic function, $x^2$, does not form a straight line when plotted on a logarithmic scale.
Plotting a fast-growing data series on a log scale is a quick and easy way for the data scientist to "feel" if the curve might fit an exponential behavior or not.}
\label{fig:log_exp}
\end{figure}

In his 1954 article *Relation between Weight-Lifting Totals and Body Weight*,
Lietzke claims that an athlete's "weight-lifting ability should be proportional
to the two-thirds power of the body weight" [@doi:10.1126/science.124.3220.486].
Lietzke scales both the $x$ and $y$ axes with a logarithm, where $x$ and $y$ 
respectively represent bodyweight and weightlifting total.
The resulting log-log plot shows straight line with a slope of $0.67 \approx 2/3$ indicates that if

$$
\log \text{strength} \propto \frac{2}{3} \log \text{bodyweight}
$$

then^[The symbol $\propto$ means "is proportional to."]
<!-- Don't recognize a symbol? Use <https://detexify.kirelabs.org/classify.html> --> 

$$
\text{strength} \propto \text{bodyweight}^{2/3}.
$$

Changing the scale on a plot can be a simple but powerful method to develop intuition
for the shape of the data. However, one should be cautious of over-generalization.
In section \ref{sec:logistic}, we will see misleading shape in the plot of a
logistic curve, but first we must explain the sigmoid curve in section
\ref{sec:sigmoid}.

### Sigmoid Curves {#sec:sigmoid}

The *sigmoid* function, $\sigma(x)$, can be used to model a system characterized
by competing exponential growth and decay^[The letter $\sigma$ has many meanings in 
mathematics and statistics. In section \ref{sec:moments}, we will introduce
variance and standard deviation, which use the symbols $\sigma^2$ and $\sigma$.
Even well-known symbols, such as $\pi$ and $e$, have overloaded meanings in this
field. One must take care to disambiguate meanings using prose.]. That is, the
sigmoid represents a system with limited resources.

$$
\sigma \left( x \right) = \frac{1}{1+e^{-x}} = \frac{e^x}{1+e^x}
$$

An example from epidemiology is the spread of a contagion 
within a population. Initially, very few individuals have the disease, but the 
rate at which the disease spreads quickly increases as the number of infected 
members compounds. At the same time, however, the probability that another 
individual is already infected or can resist the contagion also increases, 
slowing the spread as we reach some *inflection point*, as shown in figure
\ref{fig:sigmoid}.

<!-- <https://tex.stackexchange.com/a/563446/311890> --> 
<!-- <https://tikz.dev/pgfplots/> -->
\begin{figure}
\centering
\includegraphics{fig/sigmoid.tikz}
\caption{The sigmoid curve models a system with exponential growth and
simultaneous exponential decay. At its inflection point, the rate at which the
curve }
\label{fig:sigmoid}
\end{figure}

### Logistic Curves {#sec:logistic}
<!-- <https://www.researchgate.net/publication/233238354_Math-alive_using_original_sources_to_teach_mathematics_in_social_context> --> 
The *logistic* function^[todo @Shulman01011998] is a parameterized sigmoid function of the form

$$
\frac{L}{1+e^{-k \left( x-x_0 \right)}}.
$$

Figure \ref{fig:logistic} shows a logistic function

$$
l(x) = \frac{100}{1+2.75 e^{-0.4x}}
$$

plotted over the domain $0 \le x \le 5$. The parameters and domain are chosen
carefully to provide a confusing plot. The curve in figure \ref{fig:logistic}
forms a mostly straight line. Data sampled from this narrow range might fit a
linear model with very little error, but of course this is only because we have
zoomed into the center of the curve.

Consider a fad in the world that starts very small, but quickly spreads in
popularity as network effects cascade into increased awareness. One might call
this a "trend," and the trend line may initially appear linear or even
exponential, but as the fad grows so too might shortages, opposition, or
regulation slow its growth.

\begin{figure}
\centering
\includegraphics{fig/logistic.tikz}
\caption{The area near the inflection point of logistic and sigmoid functions
might be mistaken for linear growth. In this example, we see only $l(x)$ for
$0 \le x \le 5$. Such situations are common. Consider investment opportunities,
such as Bitcoin, which quickly rise from obscurity into a hype cycle only to
gradually fade. To maximize one's return in such markets one must invest early,
long before the trend begins to slow, but in a noisy market it is difficult to
predict if the function has already reached its inflection point or not.}
\label{fig:logistic}
\end{figure}

## Discussion prompts

#. In section \ref{sec:lineplot} we see a line plot ordering words for bodies of
water. As a group activity, create line plots to similarly order words for our
emotions. Some example words include, in no particular order:

    #. Happiness: pleased, excited, inspired, delighted, amused, joy

    #. Sadness: disappointed, blue, depressed, despondent, hopeless

    #. Anger: slighted, miffed, annoyed, frustrated, irritated, angered, 
    apoplectic, rage

#. Take a close look at the CrossFit.com data in section \ref{sec:pareto-chart}
and ask whether this "GW1516" substance is actually the most commonly *abused*
substance or whether it is the most commonly *detected* substance. Think of
other situations where problems may be over- or under-represented due to the
sensitivity of a test.

#. Like a bar plot, a pie chart shows the relative sizes of categorical values.
What are some advantages and disadvantages of using pie charts?

#. What are some plot practices, such as inconsistent scales, that would be
misleading to the reader? 

#. Consider a situation where the sigmoid and logistic curves might reasonably
model constrained exponential growth. If one only observes the center of this
system, then the slow initial growth and diminishing returns might not be 
visible in a scatter plot of the data. Discuss graphical and analytical methods
one might use to predict the future behavior of the uncertain system.

#. In addition to numerical grades, a teacher wants their students to know their
relative standing in comparison to their peers. The teacher wants to minimize
how much information students can infer about their classmates, although it is
desirable for students to know the central (mean, median, or mode) grades.
Which plot technique is better for this task: a bar plot or a box plot?

## Practical exercises

#. Given a dataset, plot the data and explain why this plot technique is appropriate. 

#. Be creative and construct intentionally misleading plots, then try to "spot the flaw" in one another's work.  

#. Plot our logistic function from section \ref{sec:logistic},
$l(x) = \frac{100}{1+2.75 e^{-0.4x}}$, on a logarithmic scale and manipulate
the domain. Does the logistic function still look linear on a logarithmic scale?

#. Spot the flaw in figure \ref{fig:bad-barplot-exercise}.

    \begin{figure}
    \centering
    \includegraphics{fig/bad-barplot-exercise.tikz}
    \caption{These bars represent values 20, 30, and 50.}
    \label{fig:bad-barplot-exercise}
    \end{figure}

#. The word "exponentially" is sometimes used as a superlative. Identify
    which, if any, among the following sentences is the term likely used in a
    valid mathematical sense.

    #. "This sandwich is exponentially better than the last one."

    #. "The stock price has grown exponentially for seven straight quarters."

    #. "Pressure increases exponentially with depth underwater." <!-- <https://oceanservice.noaa.gov/facts/pressure.html> -->

    #. "An air bike becomes exponentially more difficult with speed." <!-- <https://vikingathletics.net/assault-bike/> --> 

#. Spot the flaw in figure \ref{fig:bad-piechart-exercise}.

    \begin{figure}
    \centering
    \includegraphics{fig/bad-piechart-exercise.tikz}
    \caption{These bars represent values 5, 3, 2, and 1.}
    \label{fig:bad-piechart-exercise}
    \end{figure}
