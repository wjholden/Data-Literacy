all: dl.pdf

01-introduction.md:

02-data.md:

03-centrality.md:

04-lm.md:

05-testing.md:

06-supervised.md:

07-unsupervised.md:

08-graph.md:

99-references.md:

metadata.txt:

references.bib:

# https://stackoverflow.com/questions/39603422/makefile-for-dotfiles-graphviz
%.dot.pdf: %.dot
	dot -Tpdf $< -o $@ -Efontname=JuliaMono -Nfontname=JuliaMono

dl.pdf: $(patsubst %.dot,%.dot.pdf,$(wildcard *.dot)) *.md metadata.txt references.bib
	pandoc -o dl.pdf metadata.txt 00-preface.md \
	01-introduction.md 02-data.md 03-centrality.md \
	08-graph.md 99-references.md \
	--citeproc --pdf-engine=xelatex --toc --number-sections

clean:
	rm dl.pdf
	rm *.dot.pdf