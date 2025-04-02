# Introduction

## Data Literacy

The *Wisdom Hierarchy* [@doi:10.1177/0165551506070706] begins with raw *data*.
In context, data form *information*. When aggregated and interpreted through
subjective values, information forms *knowledge*. When applied in novel
circumstances, knowledge supports *wisdom*. Another model for Data, Information,
Knowledge, and Wisdom (DKIW) is that they answer measurements, what, how, and why.
The DKIW model is often visualized as the pyramid shown in figure \ref{fig:dikw}.

\begin{figure}
\centering
\includegraphics{dikw.tikz}
\caption{The DIKW model shows data, information, knowledge, and wisdom in a hierachy.
Lower layers of abstraction support higher levels of understanding.}
\label{fig:dikw}
\end{figure}

This book is intended to develop *data literacy*, one's ability interpret data
into useful information that will support knowledge and wisdom. Studying data
requires an understanding of measurements, basic mathematics, statistics,
computation, computer programming, databases, and graphs.

The weakest form of knowledge is *anecdata*, based on personal experience and
intuition and not scientific rigor. Acecdata can be difficult to refute.
Imagine the smoker who insists that *they* have not (yet) experienced any
harmful side-effects from smoking.

We use the symbol $n$ to represent the size of a *study*. Studies can be
*interventional* (where the researcher actively makes a change to measure the 
result) or *observational* (where the researcher passively measures results
without directly influencing the experiment). Both interventional and
observational studies support *empirical knowledge* gathered through experiments.

An $n=1$ study is effectively anecdata. Small studies risk incorrect conclusions
due to *lurking variables* (variables not known or measured by the researcher)
or *confounding variables* (variables that interfere with one another). The
smoker might observe that lung cancer patients are elderly, claiming that age,
not smoking, is the proximate cause.

Larger studies seek to *control* for these problems by capturing many observations
among many subjects. A large study should also account for *noise* due to 
sampling. The human population is *approximately* 50% male
and 50% female, but in an individual classroom we might have, for example, 11
girls and only 6 boys. This imbalance can be easily explained by random noise.
If, on the other hand, a large school has 1110 girls and only 600 boys, then one
should be more surprised by this result, assuming our *first principle* that the
proportion of men and women should be nearly equal.

*Data mining* is an effort to distill useful information when one lacks those
first principles. For example, could a climate scientist discover the ideal gas
law, $P V \propto T$ (pressure and volume are proportionate to temperature) using 
only weather data? Once the data scientist suspects a relationship among data, a 
traditional scientist could structure an experiment to *verify* or *falsify* the 
hypothesis.

Verification and falsification are powerful tools. Both are useful to refute
absolute statements. If one says, "all swans are white," then the discovery of
only a single black swan falsifies the statement. Likewise, qualified statements
with the quantifier "some" or "at least one" are also supported by verification.
For example, the statement "some people are seven feet tall" is verifiably true,
although not easily as such people are very rare. 

A joke goes that an astronomer, a physicist, and a mathematician are on a train
to Edinburgh and see a cow. The astronomer says, "all cows are white." The
physicist says, "some cows in Scotland are white." The mathematician says, 
"there exists *at least* one cow in Scotland such that *one side* is white."
The level of precision reflects the specificity of one's conclusions in each
field.

*Pure mathematics*, unlike most sciences, primarily uses *deductive* reasoning.
Mathematical reasoning begins with *axioms* (also known as *postulates* in 
geometry) that are considered obvious and acceptable without proof. From axioms,
we prove *theorems* through several methods.

Mathematical methods include proof by construction (also known as direct proof)^[
The symbol $\implies$ is pronounced "implies" and is called *conditional
implication*. $a \implies b$ when $b$ is always true when $a$ is true.
One can alternatively read $a \implies b$ as "if $a$, then $b$."
The *statement* $S = a \implies b$ is false if $b$ is false when $a$ is true.
$S$ is still considered true when $a$ is false, regardless of the value of $b$.
An example is the statement, "if it is raining, then I wear a jacket,"
$r \implies j$. The statement is false if it rains but the speaker does not
wear a jacket. The statement remains true if the speaker wears a jacket
in the snow or cold without rain.]


<!-- <https://www.mathcentre.ac.uk/resources/uploaded/mathcentre-direct.pdf> -->
$$
p \implies q,
$$

<!-- <https://math.libretexts.org/Courses/Monroe_Community_College/MTH_220_Discrete_Math/3%3A_Proof_Techniques/3.4%3A_Indirect_Proofs> --> 

proof by contrapositive^[The symbol $\neg$ is pronounced "not" and indicates
negation. $\neg T = F$ and $\neg F = T$.]
^[The symbols $\land$ and $\lor$ form
the logical "and" and "or", also known as *conjunction* and *disjunction*.
The statement $a \lor b$ is an *inclusive or*, meaning the statement is true if
$a$ is true, $b$ is true, or both $a$ and $b$ are true. The symbol for an
*exclusive or* is $a \oplus b$ or, less commonly, $a \veebar b$.]

$$
\neg p \land \left( \neg q \implies \neg p \right) \implies \left( p \implies q \right),
$$

<!-- <https://en.wikipedia.org/wiki/Proof_by_contradiction> -->
proof by contradiction

$$
\neg \neg p \implies p,
$$

and proof by mathematical induction^[We will see examples of inductive proofs
in sections \ref{sec:choose2} and \ref{sec:bfs}.]

$$
p(i) \land \left( p(k) \implies p(k+1) \right) \implies p(n).
$$

Most analysis lies somewhere between the extremes of data mining and pure 
mathematics. These *applied mathematics* endeavors use a combination of data and
reasoning to construct *models* to *predict* the behavior of the world.

A *binary classifier* is an example of a model. Let $C$ be such a classifier
returns either true ($T$) or false ($F$). The *accuracy* of the model is the
proportion of true positives (TP) and true negatives (TN) of its predictions,
which include false positives (FP) and false negatives (FN).

$$
\text{Accuracy} = \frac{\text{TP}+\text{TN}}{\text{TP}+\text{FP}+\text{TN}+\text{FN}}
$$

There are many paradigms for implementing our models on a computing machine.
*Imperative* programming, visualized in figure \ref{fig:imperative}, allows us
to represent an *algorithm* (a procedure to compute a result) directly as code.
These programs construct knowledge from data in a bottom-up structure.

<!-- original work -->
\begin{figure}
\centering
\includegraphics{imperative.tikz}
\caption{Imperative languages are useful to transform input into output. The
programmer provides explicit algorithms as instructions to the computing machine.}
\label{fig:imperative}
\end{figure}

*Declarative* computing environments allow the analyst to form a *query*,
where a high-level language automatically answers the desired information from
the top-down using rules or databases, as illustrated in figure \ref{fig:declarative}.
These categories are broad generalities. The command `ls *.txt` on a UNIX-like
system provides a list of files ending with the `txt` filename extension. The
interface presented to the user is declarative, but the algorithm to filter
filenames that match the specification is ultimately a set of instructions.

<!-- original work -->
\begin{figure}
\centering
\includegraphics{declarative.tikz}
\caption{Declarative languages, such as Structured Query Language (SQL),
regular expressions, and constraint solvers, search for solutions that satisfy
the desired query. The user of a declarative system should not need to
understand the internal workings necessary to answer queries.}
\label{fig:declarative}
\end{figure}

Artificial Intelligence (AI) methods, such as Machine Learning (ML), seek to
provide knowledge directly by modeling from data. AI is notoriously difficult
to define [@aima, p. 1-4]. In the past, spellcheck programs were considered AIs
[@church1991probability]. Today, the public may conflate the terms AI with ML,
Generative AI, and Large-Language Models (LLM). The point is that AI is a
fundamentally different approach to using a computer from traditional programming.
In any AI method, we seek to let the computer program itself by providing data
and general rules. The resulting model may or may not satisfy our needs.

<!-- original work -->
\begin{figure}
\centering
\includegraphics{ai.tikz}
\caption{AI systems seek to model implicit algorithms by learning from data.
A successful AI model is then used to form predictions on novel inputs not seen
in the training and testing data. AI models can be difficult or impossible to
interpret.}
\label{fig:ai}
\end{figure}

**TODO**: social impacts of science (consensus, literature review, peer review,
reproducibility...)

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

## Number Representation {#sec:numbers}

Modern computing machines are *digital* systems which represent numbers as
*strings* (lists) of *bits*. A bit has only two possible values: 0 or 1.
Today, *bytes* are eight bits long, although there were computers in the past
which did not follow this convention.

An unsigned, sixteen-bit integer has $2^{16} = \num{65536}$ possible values.
The bit string $0011000000111001$ represents \num{12345}.

$$
2^0+2^3+2^4+2^5+2^{12}+2^{13}=1+8+16+32+\num{4096}+\num{8192}=\num{12345}
$$

Computer engineers have developed several techniques for representing signed
(possibly negative) numbers. Some computers use a dedicated sign bit. Others use
*one's complement* or *two's complementent* representations.

Decimals are still more complex. *Fixed-point* decimals provide some constant
amount of digits for the whole and fractional parts of the number. *Floating-point*
decimals provide dynamic whole and fractional digits, enabling the computer to
represent a large number of decimal digits when possible. Floating-point values
are common, but have several well-known pitfalls. In any modern web browser,
such as Firefox, press F12 to open the developer console and enter `0.1+0.2`.
The result is not `0.3`:

```javascript
>> 0.1+0.2 
<- 0.30000000000000004
```

This happens for the same reason that $2 \div 3$ produces a repeating fraction
in base 10. The number $2/3$ cannot be exactly represented in a string of decimal
digits (a sum of powers of ten). Likewise, the number $0.3=3/10$ cannot be
exactly represented in a string of binary digits.

Computing machines do not process numbers. Rather, they process bit strings
which represent numbers.

## Levels of measurement {#sec:levels}

There are four distinct *levels of measurement* that a value may fit [@stevens1946theory].
*Nominal* data is simply names or categories, with no concept of order or distance.
A movie might be animated or live-action, a dichotomy without order.
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

## Discretization {#sec:discretize}

Measurements with arbitrarily many decimal digits of precision are *continuous*,
whereas measurements with finite steps in between (including categories) are *discrete*.
For example, when driving along a road, the house numbers (150 Main Street, 152 Main Street, 154 Main Street...) are discrete;
there is no intermediate value between $\num{150}$ and $\num{151}$.
On the other hand, the grid coordinates associated with each address are continuous;
one could (theoretically) specify grid position to the square millimeter, picometer, nanometer, and beyond.

Spoken English has some vocabulary for distinguishing continuous and discrete
quantities, although these conventions are not strictly necessary in daily
communication.

 - How *many* glasses do we have?
 - How *much* water do you want?
 - I walked *fewer than* $\qty{10}{\km}$.
 - She weighs $\qty{5}{\kg}$ *less than* her sister.

It can be useful to combine continuous measurements into discrete categories.
An example might be one's birth date and birth year.
No one knows their birth *instant* with subsecond precision.
Rather, the year, year and month, or year, month, and day are almost almost always enough information.
We even combine years into groups when discussing generations and peer groups.
Combining a range of birth years into generational categories is an example of *discretization*.

## Missing values {#sec:nan}

In practice, *data sets* often have missing values.
Different programming languages have substantially different syntax and semantics for representing and handling missing values.

As a small exercise, open Microsoft Excel and enter the values 1, 2, 3, and 5 into cells A1, A2, A3, and A5.
Leave cell A4 blank.
In cell A6, enter the formula `=PRODUCT(A1:A5)`.
The result is $30 = 1 \times 2 \times 3 \times 5$ (the 4 should be missing).
Excel did *not* treat the empty cell as an implicit zero and silently ignores the missing value.

Now change cell A4 to `=NA()`.
`NA` means "value not available", an explicit indication that a value is not given.
The product in cell A6 should update to `#N/A`, which explicitly tells us that there is a problem in the calculation.

Now change cell A4 to `=1/0`.
Both cells A4 and A6 should both say `#DIV/0!`, a fault telling us that a division by zero has made further calculation impossible.

Error values propagate from source data through intermediate calculations to final results.
If we enter a formula into A7 referencing A6, such as `=SQRT(A6)`, then we will find the same faults in A7 that we see in A6.

*Structured Query Language* (SQL) databases use the symbol `NULL` to denote missing values.
One might build the database *schema* (the structure of the database) to explicitly forbid `NULL` values.
For example,

```sql
CREATE TABLE Run (
  Name TEXT NOT NULL,
  Time INTEGER NOT NULL,
  Distance REAL NOT NULL)
```

defines a table *schema* where each of the three columns must be specified.
Many programming languages (including C, Java, and JavaScript) also use the term `null` for variables that do not reference any specific value.

Many programming languages support a `NaN` ("not a number") value in error conditions.
One might encounter `NaN` when dividing by zero, subtracting infinities, and parsing non-numeric words as numbers.
Comparisons with `NaN` can be confusing, such as `NaN == NaN` returning *false*.

Some programming languages will automatically *initialize* variables with some zero value.
Other languages give some `Undefined` value to uninitialized variables.
Still other languages raise an error if no explicit value is assigned to a variable.

## Optional and Sentinel Values

Computer scientist Tony Hoare famously called null references a "billion dollar mistake,"
explaining that programming languages with *nullable* values contain flaws that might
have been prevented in languages that require value initialization [@BillionDollarMistake].

Rust is one of many young languages that provides an *optional* type to express
a value which may or may not contain useful information. The form `Some(value)`
indicates a usable value. If the programmer wishes to express the absence of a
value, they use `None`.

```r
use rand::Rng;

fn g() -> Option<f32> {
    let mut rng = rand::rng();
    let x = rng.random_range(0.0..=1.0);
    if x > 0.5 {
        Some(x)
    } else {
        None
    }
}

fn main() {
    match g() {
        Some(x) => println!("g() returned some value, {x}."),
        None => println!("g() returned none.")
    }
}
```

Repeatedly run this program at the Rust Playground^[<https://play.rust-lang.org/?&gist=df4c6636ab6ff336dbae5994b7508adc>]
and observe that the `g()` function returns `Some(x)` values where $0.5 \le x \le 1.0$ and `None`.

The use of the language's type system to express optional values allows Rust to
eschew *sentinel values*, which are special values used to control a program.
An example of a sentinel value is in the `read()` function of the C programming
language. `read()` is used to read data from files, and ordinarily returns the
number of bytes read, but in many cases returns 0 or $-1$, signaling status to
the caller.

## Comma-Separated Values (CSV) {#sec:csv}

*Comma-separated values* (CSV) are a well-known situation where sentinel values
are especially problematic. A CSV file is a simple method of structuring data in
plain-text files. For example, a table such as

| $x$   | $y$                 | $z$                |
|-------|---------------------|--------------------|
| Rob   | 0.74019382956651820 | 0.3508759018489489 |
| John  | 0.41331428270607506 | 0.2936926427452584 |
| David | 0.37671743737357277 | 0.5676190157838865 |
| Frank | 0.50270122376380740 | 0.7939268929144455 |

can be entered into a CSV file as

```
x,y,z
Rob,0.74019382956651820,0.3508759018489489
John,0.41331428270607506,0.2936926427452584
David,0.37671743737357277,0.5676190157838865
Frank,0.50270122376380740,0.7939268929144455
```

In section \ref{sec:json}, we will see this same data represented in a more
modern format called JSON.

What happens if we add a column where values themselves contain commas?
RFC 4180, a specification for CSV, recommends enclosing values containing commas
with quotation marks [@rfc4180]. If the value enclosed with quotation marks also
contains a quotation mark, then a doubled quotation mark (`"this field contains
""escaped"" quotation marks"`) *escapes* the inner value for unambiguous parsing.

Unicode and its predecessor, the American Standard Code for Information Interchange (ASCII),
provide *control codes* `001C`, `001D`, `001E`, and `001F` for file, group,
record, and unit separators, respectively, but these codes are not commonly used.
If these codes had been more convenient to type, our world of data might have
avoided some of the common pitfalls of CSV and other formats containing sentinel
values.

## Strong/weak and static/dynamic typing {#sec:strong-weak-types}

Values come in many forms: categorical and numerical, ordered and unordered, discrete and continuous, defined and missing.
*Types* can be used to constrain variables to allowable values and applicable operations.

For example, suppose a database indicates how many cars a person owns.
It makes no sense to own a fractional or negative car, so we might find an existing type
(in this case, whole numbers) or define some new type to model the domain.

Some programming languages offer *dynamic* types that implicitly change the type (*cast*) of values to operate correctly.
Go to <https://jsfiddle.net> or press F12 to open the developer console in most modern browsers.
Enter the following into the JavaScript console:

```javascript
>> "5" * 5
<- 25
```

Characters inside quotation marks (`"5"`) are called *strings* and are ordinarily used for text, but JavaScript automatically parses `"5" * 5` as the product of two numerical values and returns `25`.

JavaScript is notoriously inconsistent.

```javascript
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

<!-- <https://drops.dagstuhl.de/storage/01oasics/oasics-vol076-plateau2019/OASIcs.PLATEAU.2019.6/OASIcs.PLATEAU.2019.6.pdf> -->

*Tables* of data are structured in *columns* and *rows*, where the rows represent the *individuals* or *observations* in the data set and the columns represent the *features*.
For example, a table of employee names might have two columns (the given and surnames) and ten rows, where each row represents one of the ten employees.

In computer science, the terms *list* and *array* both refer to single-column tables, but with different internal memory representation.
The distinction is usually unimportant to data analysts.

Scientific languages, such as Julia and R, often use the term *data frame* (or *dataframe*) as their method for representing tables of data.
Data frames often provide rich syntax for row-wise and column-wise operations.
By contrast, in an object-oriented language, such as Java and JavaScript, the idiomatic representation of a table is likely an array of objects.
We will discuss object-oriented programming in more detail in section
\ref{sec:oop}.

<!-- ## Forms and input validation -->



## Vectors and matrices {#sec:vector}

\begin{figure}
\centering
\includegraphics{vector-sum.tikz}
\caption{Vectors are geometric entities. This plot shows that $(4,3)+(3,5)=(3,5)+(4,3)=(7,8)$.}
\label{fig:vector-sum}
\end{figure}

We now quickly mention the terms *vector* and *matrix* here to disambiguate them from other terms already defined.

Arrays, lists, and columns containing numeric data may sometimes be represented with *vectors*.
Likewise, tables and data frames might be represented with *matrices*.

A vector is a quantity with both magnitude and direction, generally consisting
of two or more elements. A plot demonstrating vector summation is shown in
figure \ref{fig:vector-sum}.

$$
\mathbf{x} = \left( x_1 , x_2 , x_3 , \ldots , x_n \right)
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
Some programming languages use the terms "vector" and "array" interchangably,
or to indicate an array has dynamic vice fixed size.
Many programming languages support *tuples* as an alternative representation
of a quantity with multiple values.

One must take care to verify an arithmetic operator performs as expected with
"vectors" and tuples in different languages. Compare and contrast the semantics
of arrays and tuples in Python

```python
In [1]: 1 == [1]
Out[1]: False

In [2]: 1 == (1,)
Out[2]: False

In [3]: [1, 2] + [3, 4]
Out[3]: [1, 2, 3, 4]

In [4]: (1, 2) + (3, 4)
Out[4]: (1, 2, 3, 4)
```

and R

```r
> 1 == c(1)
[1] TRUE
> 1 == list(1)
[1] TRUE
> c(1, 2) + c(3, 4)
[1] 4 6
> list(1, 2) + list(3, 4)
Error in list(1, 2) + list(3, 4) :
  non-numeric argument to binary operator
```

Python's behavior is typical of general-purpose programming languages,
while R's behavior (see section \ref{sec:array-programming}) is common among
scientific languages.

## Complex Numbers

\begin{figure}
\centering
\includegraphics{argand.tikz}
\caption{Complex numbers are two-dimensional quantities. In this Argand diagram, the $x$ axis is $\pm 1$ and the $y$ axis is $\pm i$. Multiplying a value by $i$ rotates the value among the real and imaginary axes.}
\label{fig:argand}
\end{figure}

*Complex numbers*, which have "real" and "imaginary" components, are also
multidimensional values, but with the property  $i = \sqrt{-1}$ and therefore 
$i^2 = -1$. This means that their multiplication forms a cycle, as shown in 
figure \ref{fig:argand}.

$$
\begin{aligned}
1 \times i &= i \\
i \times i &= -1 \\
-1 \times i &= -i \\
-i \times i &= 1 \\
\end{aligned}
$$

Many languages provide complex arithmetic for
situations that require it. An R example is shown below.

```r
> c(1, 1i, -1, -1i) * 1i
[1]  0+1i -1+0i  0-1i  1+0i
```

While complex arithmetic is common in physics and signal progressing,
many scientific disciplines have no use cases for complex numbers.
If one has a two-dimensional quantity and no application for the multiplication
rules shown in figure \ref{fig:argand}, then one should avoid complex types
and instead favor arrays.

As an example, suppose one wanted to represent the systolic and diastolic
values in blood pressure samples. One *could* use a complex value instead of
an array or tuple, but now the values have a concept of multiplication and
direction that is not appropriate to the problem domain. Just as we learned
to consider which arithmetic operations apply to our data in section
\ref{sec:levels}, likewise we must consider the operations applicable to
composite values.

## Sets, relations, functions, and algorithms {#sec:discrete-math}

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
\textrm{Translate} \left( \textrm{Friday}, \textrm{English}, \textrm{German} \right) &= \textit{Freitag} \\
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

## Abstraction and Reification

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

Abstractions are *leaky* when one must understand the internal details to use the
abstraction effectively. For example, surprises such as `0.1 + 0.2 == 0.30000000000000004`
in floating-point arithmetic lead to programmers understanding more implementation
details than intended; floating-point arithemetic is a leaky abstraction towards
representing real numbers (see section \ref{sec:numbers}). The object-oriented
paradigm (see section \ref{sec:oop}) emphasizes *encapsulation* of both data
and code to allow users to use the *interface* exposed by an object without
consideration for its implementation.

*Reification* is the opposite of abstraction: we something specific from something general.
For example, suppose we have a language translation function

$$
T(x, l_1, l_2)
$$

where $x$ is the message to be translated, $l_1$ is the input language, and $l_2$ is the output language.

$$
T(\text{hello}, \text{English}, \text{French}) = \textit{bonjour}
$$

From this general function, we can enclose parameters $l_1$ and $l_2$ into a specific function.

$$
\begin{aligned}
T'(x) &= T(x, \text{English}, \text{French}) \\
T'(\text{goodbye}) &= \textit{au revoir}
\end{aligned}
$$

Function $T'$ reifys $T$ into a less general form.
Such functions might be called *convenience functions* that are provided as a "quality of life" improvement for the user.
An example of a convenience function might be `LOG10(number)` in Excel.
Excel also provides `LOG(number,[base])` (where `base` defaults to 10 if omitted), but some users may prefer the explicit syntax `LOG10` to improve clarity.

## Discussion prompts

1. If `ls *.txt` is a declarative program, then is `rm *.txt` also a declarative program?

2. Who owns knowledge management? 

3. What are good and bad uses for spreadsheets? 

4. What is reproducibility? Why would this be important for scientific inquiry? 

5. A manager sends an Excel spreadsheet to their employees, telling them to each
enter information and send it back. What are some challenges the manager might
experience while merging these spreadsheets?

6. The set of natural numbers, $\mathbb{N} = \left\{ 0, 1, 2, 3, \ldots \right\}$,
can be constructed using a *successor function*, $S(n)=n+1$. If we begin with $0=0$,
then $1=S(0)$, $2=S(1)=S(S(0))$, $3=S(2)=S(S(1))=S(S(S(0)))$, and so on.
Can we define the reals, $\mathbb{R}$, in such a way? Could we construct a
successor function for floating-point approximations of real numbers?

7. CSV makes *appending* data easy: simply write new rows to the end of the file.
This structure also makes it easy to *stream* the data record-by-record, but it
makes the schema of the data inflexible. One must define the columns in the header
of the CSV and continue to use this structure thereafter. Do Excel spreadsheets
and SQL databases share this problem? How can organizations store their data
when their requirements may change?

## Practical exercises

1. Create a small survey using Microsoft Forms (part of Office 365) or Google Forms (part of Google Docs).
Compare this experience to the hypothetical manager who gathered information by
manually merging spreadsheets.

2. Given a noisy and poorly structured dataset, propose a method of restructuring the data. 

3. Discretize the values of a dataset and explain the reasoning. 

4. The following Rust program, which one can run at the
Rust Playground^[<https://play.rust-lang.org/?gist=82eb9505ef18cf3af0faa2a373c11901>],
doubles a value until we exceed the largest representable value. However, the
program *appears* to make an arithmetic error at $\num{134217730}$.
$\num{67108864} \times 2 = \num{134217728}$, not $\num{134217730}$ (no power of
two could ever end in zero in decimal). Use <https://www.h-schmidt.net/FloatConverter/IEEE754.html>
to investigate the error.

```rust
fn main() {
    let mut x: f32 = 1.0;
    while x != f32::INFINITY {
        println!("{x}");
        x *= 2.0;
    }
}
```

5. What is the output of the following Java program? Use the 
Java Playground^[<https://dev.java/playground/>] to experiment.

```java
static void count() {
    float x = 0.0f;
    while (x != x + 1.0f) {
        x += 1.0f;
    }
    System.out.println(x);
}

count();
```

<!-- ° ALT+0176 on Windows -->
6. An *azimuth* on a magnetic compass conventionally reads 0° when pointed north, 
90° for east, 180° for south, and 270° for west. In trigonometry, the angle
0° corresponds to $(x,y)$ position $(1,0)$ on the unit circle, 90° to $(0,1)$,
180° to $(-1,0)$, and 270° to $(0,-1)$. Implement a function $A$ to convert
azimuths to angles, another function $A^{-1}$ to convert angles to azimuths,
and create test cases to verify that $A^{-1}(A(\theta))=\theta$.