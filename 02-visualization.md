# Data Visualization {#sec:visualization}

## Plots {#sec:plots}

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

As a small exercise to experiment with these four plots, go to <https://webr.r-wasm.org/latest/> to use the R language in a web browser.
R is a programming language for statistics and data visualization.

R includes several built-in data sets.
We will use included Motor Trend Cars (`mtcars`) data set.

In the *read-evaluate-print loop* (*REPL*) at the bottom-left of the screen, enter

```r
> head(mtcars)
```

to view the column names and first six rows of the Motor Trend Cars (`mtcars`) data set.
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

## Bar Plots {#sec:barplot}

In the REPL of <https://webr.r-wasm.org/latest/>, create a *bar plot* from the cylinders
(`cyl`) column of the Motor Trend Cars data set: 

```r
> barplot(mtcars$cyl)
```

![A bar plot of the number of cylinders in each car of the Motor Trend Cars data set.](mtcars-barplot.pdf){#fig:barplot}

Bar plots are useful for comparing numerical features of a data set when grouped
by some categorical variable. The categorical group is the *independent* variable.
The numerical feature, plotted as the height of the bars, is the *dependent* varible.
Independent and dependent variables are sometimes called *free* and *response*
variables. In an *interventional* study (where a researcher performs an action to
quantify the effect), the independent variable is the item changed directly and
the dependent variable is the outcome caused by the change. Bar plots almost
always require some amount of "data wrangling," such as the use of SQL aggregate
functions (more on this in section \ref{sec:grouping-and-aggregation}) such as
`MIN()`, `MAX()`, `COUNT()`, `SUM()`, and `AVG()`. Figure \ref{fig:barplot}
demonstrates a bar plot.

The width of each bar must be uniform. Only the bar height varies. As 

$$
\text{Area} = \text{Width} \times \text{Height},
$$

the exaggerated area of a wide or thin bar will mislead the reader.
For example, suppose a bar plot is intended to compare the values $x = h = \left( 3, 10, 11 \right)$,
but the bars corresponding to each observation are, respectively, $w = \left( 1, 1, 3 \right)$.
The resulting areas are $w \odot h = \left( 3, 10, 22 \right)$ (here, $\odot$ 
indicates the *element-wise* product of two vectors, also known as a *Hadamard*
product). As shown in figure \ref{fig:misleading-barplot}, the area of the third
bar is more than triple that of the second and may confuse the reader.

\begin{figure}
\centering
\includegraphics{misleading-barplot.tikz}
\caption{The bars of a bar plot should ordinarily have uniform width.
This bar plot shows values $x = \left( 3, 10, 11 \right)$, but the
width of the third bar makes this observation appear much larger than the others.}
\label{fig:misleading-barplot}
\end{figure}

## Cumulative Sums and Pareto Charts

A *Pareto chart* is a useful analytical tool to show the relative importance of
problems in industrial settings. The chart shows the proportion of problems
discretized (see section \ref{sec:discretize}) into root causes. We can
compute these cumulative sums using reduce and visualize them with a bar plot.

The following example uses data gathered from
CrossFit.com^[<https://games.crossfit.com/article/complete-list-athletes-currently-serving-sanctions>].
The `%>%` operator, from the `dplyr` package, anonymously "pipes" the output
from one function into the first argument of the next function.
Structurally, the `%>%` produces a left-to-right order of operations that
can be easier to write, read, and maintain than functions written in prefix and
infix notation. `dplyr` uses `mutate` as row-wise `map` operation with support
for aggregate functions (such as `sum(n)` below;
see also section \ref{sec:grouping-and-aggregation}).

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
```

This dataset does not quite show the famous "Pareto Principle" where 20% of 
problems cause 80% of problems, but we do see that the distribution is not
uniform. The first category accounts for over half of the observations.
Using R's `ggplot` library, we show the resulting Pareto chart with the bars
and cumulative sum line.

```r
df %>% ggplot(aes(x = reorder(Violation, -Proportion))) +
	geom_bar(aes(weight = Proportion)) +
	geom_line(aes(y = CumSum, group=1)) +
	xlab("Drug Violation") +
	ylab("Proportion")
```

![A Pareto chart shows the relative and cumulative proportions of discretized quantities, sorted in decreasing incidence. Frequently used in quality control processes, such as Lean Six Sigma, Pareto charts may show that only one or a few causes lead to a significant proportion of problems.](pareto-chart.pdf)

## Box Plots {#sec:boxplot}

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

R provides `boxplot` function to render box plots. Return to the <https://webr.r-wasm.org/latest/>
site and experiment with this function.

```r
> boxplot(mtcars$mpg)
```

![R's `boxplot` function creates box-and-whisker plots with four quartiles.
Box plots are centered at the median of the data. Outliers may be shown as dots
or cicles beyond the 0th and 100th percentile markers.](mtcars-boxplot.pdf){#fig:boxplot}

## Histograms {#sec:histogram}

```r
> hist(mtcars$mpg)
```

![todo](mtcars-hist.pdf){#fig:histogram}

## Scatter Plots {#sec:scatter}

```r
> plot(mtcars$wt, mtcars$mpg)
```

![todo](mtcars-plot.pdf){#fig:scatter}

Todo: change base R to ggplot so we can show color.

## Heat Maps {#sec:heatmap}

A *heat map* (sometimes written *heatmap*) is a plot that uses a continuous range
of colors to overlay values onto a two-dimensional plot. Heat maps are especially
useful for overlaying information onto geographical maps.
Figure \ref{fig:military-expenditure-europe} demonstrates a map of Europe where
countries are colored by military expenditures as a fraction of their economies.

![A heat map showing military expenditures in European countries. This plot was created in Wolfram Mathematica with input `GeoRegionValuePlot[Flatten[{CountryData[#, "Polygon"] -> CountryData[#, "MilitaryExpenditureFraction"]} & /@ CountryData["Europe"]]]`.](MilitaryExpenditureFraction.pdf){#fig:military-expenditure-europe}

## Linear and logarithmic scales {#sec:scales}

Scientists use the term *order of magnitude* to compare values only by the power of $10$.
One would say $a = 1.6 \times 10^{3}$ is three orders of magnitude smaller than $b = 8.3 \times 10^{6}$,
which is to say $b/a \approx \num{1000}$.

The *scale* of an axis, such as in bar plot, is the spacing between values.
A *linear scale* might show marks at 10, 20, 30, 40, and so on.
A *logarithmic scale* might show marks at 10, 100, $\num{1000}$, $\num{10000}$, and so on.

![](barplot-linear-scale.pdf){width=50%}
![](barplot-log-scale.pdf){width=50%}
\begin{figure}[!ht]
\caption{These two bar plots show the same data using different scales. The left plot uses a linear scale, where successive marks have a constant \textit{additive} distance. The right plot uses a logarithmic scale, where succesive marks have a constant \textit{multiplicative} difference. A logarithmic scale is useful when values differ by orders of magnitude, as the large values obscure differences among the smaller values. Observe that the third and fourth values appear nearly the same on a linear scale, but are clearly different on a logarithmic scale. *TODO: why isn't this appearing with the two barplots?*}
\label{fig:scales}
\end{figure}

Logarithmic scales can be useful for comparing values that differ by more than one order of magnitude.
For example, suppose feature of a data set contains categories $a$, $b$, $c$, and $d$, and the count of each category is

| Category | Count        |
|----------|--------------|
| $a$      | \num{10736} |
| $b$      | \num{1711}  |
| $c$      | \num{398}   |
| $d$      | \num{319}   |

Return to <https://webr.r-wasm.org/latest/> and plot this data with linear and logarithmic scales:

```
> category_counts <- c(10736, 1711, 398, 319)
> category_counts
[1] 10736  1711   398   319
> barplot(category_counts)
> barplot(category_counts, log="y")
```

These plots are shown in figure \ref{fig:scales}.

## Logarithms and exponentiation {#sec:log_exp}

A logarithm is the inverse of exponentiation. If 

$$
a^b = \underbrace{a \times a \times a \times \cdots \times a}_{b \text{ terms of } a} = c,
$$

then 

$$
\log_a c = b.
$$

In this case, $a$ is the *base* of the logarithmc, and we read $\log_a c$ as
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
\begin{tikzpicture}
    \begin{axis}[
        axis lines = left,
        xlabel = {\(x\)},
        ylabel = {\(e^x\)},
    ]
        \addplot [
            domain=-4:4,
            samples=100,
            color=red,
        ]{exp(x)};
    \end{axis}
\end{tikzpicture}
\begin{tikzpicture}
    \begin{axis}[
        axis lines = left,
        xlabel = {\(x\)},
        ylabel = {\(\ln x\)},
    ]
        \addplot [
            domain=-4:4,
            samples=100,
            color=blue,
        ]{ln(x)};
\end{axis}
\end{tikzpicture}
\caption{The exponential function, $e^x$, models iterated multiplication and grows quickly.
The domain (allowable inputs) for $e^x$ are all real numbers, but the range (possible outputs) are strictly positive reals.
It is not possible for $e^x$ to produce a zero or negative output if $x$ is a real number.
The logarithmic function, $\ln x$, inverts exponentiation and grows very slowly.
The domain and range for $\ln x$ are the reverse of $e^x$: the domain of $\ln x$ is the positive reals and range is all reals.}
\label{fig:exp}
\end{figure}

## Relationships

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
\begin{tikzpicture}
    \begin{axis}[
        axis lines = left,
        xlabel = {\(x\)},
        ylabel = {\(e^x\)},
        ymode = log,
    ]
        \addplot [
            domain=0:10,
            samples=100,
            color=red,
        ]{exp(x)};
    \end{axis}
\end{tikzpicture}
\begin{tikzpicture}
    \begin{axis}[
        axis lines = left,
        xlabel = {\(x\)},
        ylabel = {\(x^2\)},
        ymode = log,
    ]
        \addplot [
            domain=0:10,
            samples=100,
            color=blue,
        ]{x*x};
    \end{axis}
\end{tikzpicture}
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

## Sigmoid Curves {#sec:sigmoid}

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
\begin{tikzpicture}
    \begin{axis}[
        axis lines = left,
        xlabel = {\(x\)},
        ylabel = {\(\sigma \left( x \right) = \frac{e^x}{1+e^x}\)},
    ]
        \addplot [
            domain=-5:5,
            samples=100,
        ]{exp(x)/(1+exp(x))};
        
        \addplot [
            only marks,
            mark = *,
            mark size = 1pt,
            point meta=explicit symbolic,
            nodes near coords,
            coordinate style/.from=right
        ] coordinates {
            (0, 0.5) [Inflection Point]
        };
    \end{axis}
\end{tikzpicture}
\caption{todo sigmoid.}
\label{fig:sigmoid}
\end{figure}

## Logistic Curves {#sec:logistic}
<!-- <https://www.researchgate.net/publication/233238354_Math-alive_using_original_sources_to_teach_mathematics_in_social_context> --> 
The *logistic* function^[@Shulman01011998] is a parameterized sigmoid function of the form

$$
\frac{L}{1+e^{-k \left( x-x_0 \right)}}.
$$

Figure \ref{fig:logistic} shows a logistic function

$$
l(x) = \frac{100}{1+2.75 e^{-0.4x}}
$$

plotted over the domain $0 \le x \le 5$. The parameters and domain are chosen carefully to provide aconfusing plot. The curve in figure \ref{fig:logistic}
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
\begin{tikzpicture}
    \begin{axis}[
        axis lines = left,
        xlabel = {\(x\)},
        ylabel = {\(\frac{100}{1 + 2.75 e^{-0.4x}}\)},
    ]
        \addplot [
            domain=0:5,
            samples=100,
        ]{100/(1+2.75 * exp(-0.4 * x))};
    \end{axis}
\end{tikzpicture}
\caption{todo logistic.}
\label{fig:logistic}
\end{figure}

## Discussion prompts

1. Like a barplot, a pie chart shows the relative sizes of categorical values.
What are some advantages and disadvantages of using pie charts?

2. What are some plot practices, such as inconsistent scales, that would be
misleading to the reader? 

3. Consider a situation where the sigmoid and logistic curves might reasonably
model constrained exponential growth. If one only observes the center of this
system, then the slow initial growth and diminishing returns might not be 
visible in a scatter plot of the data. Discuss graphical and analytical methods
one might use to predict the future behavior of the uncertain system.

## Practical exercises

1. Given a dataset, plot the data and explain why this plot technique is appropriate. 

2. Be creative and construct intentionally misleading plots, then try to "spot the flaw" in one another's work.  

3. Plot our logistic function from section \ref{sec:logistic},
$l(x) = \frac{100}{1+2.75 e^{-0.4x}}$, on a logarithmic scale and manipulate
the domain. Does the logistic function still look linear on a logarithmic scale?

4. Spot the flaw in figure \ref{fig:bad-barplot-exercise}.

\begin{figure}
\centering
\begin{tikzpicture}
\filldraw[black, fill=red!5] (0,0) rectangle (1,1.5);
\filldraw[black, fill=green!5] (2,0) rectangle (3,2.5);
\filldraw[black, fill=blue!5] (4,0) rectangle (5,5);
\node[text width=1] at (.275,.5) {20};
\node[text width=1] at (2.275,.5) {30};
\node[text width=1] at (4.275,.5) {50};
\end{tikzpicture}
\caption{These bars represent values 20, 30, and 50.}
\label{fig:bad-barplot-exercise}
\end{figure}

5. Spot the flaw in figure \ref{fig:bad-piechart-exercise}.

\begin{figure}
\centering
\begin{tikzpicture}
\pie[hide number, radius=2]{42/5,28/3,21/2,9/1}
\end{tikzpicture}
\caption{These bars represent values 5, 3, 2, and 1.}
\label{fig:bad-piechart-exercise}
\end{figure}
