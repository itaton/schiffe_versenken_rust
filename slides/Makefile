MAIN = Battleships
LATEX = pdflatex -shell-escape
VIEWER = zathura

view: pdf
	$(VIEWER) $(MAIN).pdf

pdf:
	$(LATEX) $(MAIN).tex

clean:
	rm -f *.aux *.log *.nav *.snm *synctex.gz *.toc *.out

tidy: clean
	rm -f *.dvi *.ps *.pdf

.PHONY : pdf view clean tidy
