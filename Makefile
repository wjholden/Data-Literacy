all: dl.pdf

01-introduction.md:

02-data.md:

03-centrality.md:

04-lm.md:

05-testing.md:

06-supervised.md:

07-unsupervised.md:

08-graph.md:

metadata.txt:

references.bib:

dl.pdf: *.md metadata.txt references.bib
	pandoc -o dl.pdf metadata.txt 01-introduction.md 02-data.md \
	03-centrality.md 04-lm.md 05-testing.md 06-supervised.md 07-unsupervised.md \
	08-graph.md --citeproc --pdf-engine=xelatex --toc

clean:
	rm dl.pdf