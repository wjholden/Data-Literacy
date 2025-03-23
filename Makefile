all: test dl.pdf

00-preface.md:

01-introduction.md:

02-visualization.md:

03-data.md:

04-centrality.md:

05-dimensionality.md:

06-lm.md:

07-testing.md:

08-graph.md:

09-supervised.md:

10-unsupervised.md:

11-sat.md:

99-references.md:

metadata.txt:

references.bib:

# https://stackoverflow.com/questions/39603422/makefile-for-dotfiles-graphviz
%.dot.pdf: %.dot
	dot -Tpdf $< -o $@ -Efontname=JuliaMono -Nfontname=JuliaMono

pareto.pdf: pareto.jl lifts.csv
	julia pareto.jl

dl.pdf: $(patsubst %.dot,%.dot.pdf,$(wildcard *.dot)) *.md metadata.txt references.bib pareto.pdf
	pandoc -o dl.pdf metadata.txt 00-preface.md \
	01-introduction.md 02-visualization.md 03-data.md 04-centrality.md 05-dimensionality.md \
	08-graph.md 99-references.md \
	--citeproc --pdf-engine=lualatex --toc --number-sections --fail-if-warnings=true

dl.html: dl.pdf
	pandoc -s -o dl.html metadata.txt 00-preface.md \
	01-introduction.md 02-visualization.md 03-data.md 04-centrality.md 05-dimensionality.md \
	08-graph.md 99-references.md \
	--citeproc --toc --number-sections

.PHONY: clean
clean:
	rm dl.pdf
	rm *.dot.pdf
	rm pareto.pdf

# Error on common misspellings. It's supposed to be "first principles" and
# "principal component analysis."
#
# We also don't want any more "bare" URLs. As a matter of style, let's always
# wrap URLs in <angle brackets>. Also, avoid \url{} LaTeX syntax.
#
# Don't use pretentious non-English letters in English words:
### ä 0228
### ë 0235
### ï 0239
### ö 0246
### ü 0252
#
# TikZ pictures should all be in separate *.tikz files instead of embedded directly
# into the Markdown source code.
#
# Eventually, we will need to also search for the word "todo". We aren't ready for this yet.
.PHONY: test
test:
	@! rg -tmd --ignore-case "first principal"
	@! rg -tmd --ignore-case "principle component"
	@! rg -tmd " http"
	@! rg -tmd --fixed-strings "^[http"
	@! rg -tmd --fixed-strings "\url{http"
	@! rg -tmd "[äëïöü]"
	@! rg -tmd tikzpicture

# Errors should be in C:\Users\wjhol\AppData\Local\MiKTeX\miktex\log\lualatex.log
