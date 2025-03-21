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
.PHONY: test
test:
	@! rg -i "first principal" -g *.md
	@! rg -i "principle component" -g *.md
