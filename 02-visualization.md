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

As a small exercise to experiment with these four plots, go to https://webr.r-wasm.org/latest/ to use the R language in a web browser.
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

In the REPL of https://webr.r-wasm.org/latest/, create a *bar plot* from the cylinders
(`cyl`) column of the Motor Trend Cars data set: 

```r
> barplot(mtcars$cyl)
```

![A bar plot of the number of cylinders in each car of the Motor Trend Cars data set.](mtcars-barplot.pdf){#fig:barplot}

Bar plots are useful for numerical features of a data set.
In figure \ref{fig:barplot}, the horizontal axis is unlabeled and the order is left unspecified.
One might use labels, order, color, or grouping to aid the reader in interpreting data.

The width of each bar should ordinarily be uniform. As 

$$
\text{Area} = \text{Width} \times \text{Height},
$$

the enlarged area of the wider bar may mislead the reader. For example, suppose
a bar plot is intended to compare the values $x = \left( 3, 10, 11 \right)$,
but the bars corresponding to each observation are, respectively, $w = \left( 1, 1, 2 \right)$.
The resulting areas are $x \odot w = \left( 3, 10, 22 \right)$ (here, $\odot$ 
indicates the *element-wise* product of two vectors, also known as a *Hadamard*
product). As shown in figure \ref{fig:misleading-barplot}, the area of the third
bar is more than double that of the second and may mislead the reader.

\begin{figure}
\centering
\begin{tikzpicture}
\filldraw[black, fill=red!5] (1,0) rectangle (2,3);
\filldraw[black, fill=green!5] (3,0) rectangle (4,10);
\filldraw[black, fill=blue!5] (5,0) rectangle (7,11);
\node[text width=1] at (1.5,1) {3};
\node[text width=1] at (3.375,1) {10};
\node[text width=1] at (5.825,1) {11};
\end{tikzpicture}
\caption{The bars of a bar plot should ordinarily have uniform width.
This bar plot shows values $x = \left( 3, 10, 11 \right)$, but the
width of the third bar makes this observation appear much larger than the others.}
\label{fig:misleading-barplot}
\end{figure}

```r
> boxplot(mtcars$mpg)
> hist(mtcars$mpg)
> plot(mtcars$wt, mtcars$mpg)
```

![](mtcars-boxplot.pdf){width=25%}
![](mtcars-hist.pdf){width=25%}
![](mtcars-plot.pdf){width=25%}
\begin{figure}[!ht]
\caption{Visualizations of the Motor Trend Cars (\texttt{mtcars}) data set using the R language. *TODO* split into separate subsections.}
\end{figure}

## Linear and logarithmic scales {#sec:scales}

Scientists use the term *order of magnitude* to compare values only by the power of $10$.
One would say $a = 1.6 \times 10^{3}$ is three orders of magnitude smaller than $b = 8.3 \times 10^{6}$,
which is to say $b/a \approx \num{1000}$.

The *scale* of an axis, such as in bar plot, is the spacing between values.
A *linear scale* might show marks at 10, 20, 30, 40, and so on.
A *logarithmic scale* might show marks at 10, 100, $\num{1000}$, $\num{10000}$, and so on.

![](barplot-linear-scale.pdf){width=50%}
![](barplot-log-scale.pdf){width=50%}
\begin{figure}
\caption{These two bar plots show the same data using different scales. The left plot uses a linear scale, where successive marks have a constant \textit{additive} distance. The right plot uses a logarithmic scale, where succesive marks have a constant \textit{multiplicative} difference. A logarithmic scale is useful when values differ by orders of magnitude, as the large values obscure differences among the smaller values. Observe that the third and fourth values appear nearly the same on a linear scale, but are clearly different on a logarithmic scale.}
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

Return to https://webr.r-wasm.org/latest/ and plot this data with linear and logarithmic scales:

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

## Intuition

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

Changing the scale on a plot can be a simple but powerful method to develop intuition
for the shape of the data. However, one should be cautious of over-generalization.

Consider the *sigmoid* function,

$$
\sigma \left( x \right) = \frac{e^x}{1+e^x}.
$$

The sigmoid curve can be used to model a system characterized by competing exponential
growth and decay. An example is the spread of a contagion among a population.
Initially, very few individuals have the disease, but the rate at which the
disease spreads quickly increases as the number of infected members compounds.
At the same time, however, the probability that another individual is already
infected or can resist the contagion also increases, slowing the spread as we
reach some *inflection point*.

<!-- https://tex.stackexchange.com/a/563446/311890 --> 
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

**TODO**: Sigmoid 

## Discussion prompts

1. Like a barplot, a pie chart shows the relative sizes of categorical values.
What are some advantages and disadvantages of using pie charts?

2. What are some plot practices, such as inconsistent scales, that would be
misleading to the reader? 

## Practical exercises

1. Given a dataset, plot the data and explain why this plot technique is appropriate. 

2. Be creative and construct intentionally misleading plots, then try to "spot the flaw" in one another's work.  
