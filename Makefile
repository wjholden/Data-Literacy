all: dl.pdf

01-introduction.md:

02-data.md:

03-centrality.md:

04-dimensionality.md:

05-lm.md:

06-testing.md:

07-graph.md:

08-supervised.md:

09-unsupervised.md:

10-sat.md:

99-references.md:

metadata.txt:

references.bib:

# https://stackoverflow.com/questions/39603422/makefile-for-dotfiles-graphviz
%.dot.pdf: %.dot
	dot -Tpdf $< -o $@ -Efontname=JuliaMono -Nfontname=JuliaMono

dl.pdf: $(patsubst %.dot,%.dot.pdf,$(wildcard *.dot)) *.md metadata.txt references.bib
	pandoc -o dl.pdf metadata.txt 00-preface.md \
	01-introduction.md 02-data.md 03-centrality.md 04-dimensionality.md \
	07-graph.md 99-references.md \
	--citeproc --pdf-engine=xelatex --toc --number-sections --fail-if-warnings=true

clean:
	rm dl.pdf
	rm *.dot.pdf