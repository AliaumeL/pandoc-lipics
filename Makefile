PAPER=lipics-v2021-sample-article

# Static files for the latex compilation
TEX_STATIC=lipics-v2021.cls \
		   orcid.pdf \
		   lipics-logo-bw.pdf \
		   cc-by.pdf
# Source files are markdown documents
# in the parts directory
SRC=$(wildcard parts/*.md)

# Template file is in the template directory
TEX_TEMPLATE =template/lipics.template.tex
HTML_TEMPLATE=template/lipics.template.html

.PHONY: watch clean

%.tex: %.md $(SRC) $(TEX_TEMPLATE) $(TEX_STATIC)
	pandoc -s -o $@ \
		   $< \
		   $(SRC) \
		   --template=$(TEX_TEMPLATE) \
		   --filter lipics-filter/target/debug/lipics-filter \
		   --citeproc \
		   --metadata=suppress-bibliography:true \
		   --lua-filter=template/lipics-citation.lua \
		   --metadata=git-revision:`git rev-parse HEAD` \
		   --metadata=git-repositiory:`git remote get-url origin` \
		   -t latex

# Creating a self contained arxiv file
# with all references included
%.arxiv.tex: %.tex
	latexpand -o $@ \
			  --empty-comments \
			  --expand-bbl $(PAPER).bbl \
		      $<

# Creating an archive ready to be distributed
arxiv.tar.gz: %.arxiv.tex $(TEX_STATIC)
	mkdir -p arxiv
	mv $(PAPER).arxiv.tex arxiv/$(PAPER).tex
	cp $(TEX_STATIC) arxiv/
	tar -czf arxiv.tar.gz arxiv/*

# Building pdf files in general
# using pdflatex because
# xelatex is not compatible with lipics
# due to the use of \input{glyphtounicode}
# in the class file
%.pdf: %.tex
	latexmk -pdf -pdflatex $<

%.html: %.md $(SRC) $(HTML_TEMPLATE)
	pandoc -s -o $@ \
		   $< \
		   $(SRC) \
		   --filter lipics-filter/target/debug/lipics-filter \
		   --number-sections \
		   --template=$(HTML_TEMPLATE) \
		   --mathjax \
		   --citeproc \
		   --metadata=live-reload:$(PANDOC_LIVE_RELOAD) \
		   --metadata=git-revision:`git rev-parse HEAD` \
		   --metadata=git-repositiory:`git remote get-url origin` \
		   -t html5

watch: export PANDOC_LIVE_RELOAD=1
watch:
	find . -name "*.md" | entr -s "make $(PAPER).html && echo reload" | websocat -s 8080


clean:
	rm -f $(PAPER).{pdf,html,arxiv.tex}
	rm -f $(PAPER).{aux,bbl,blg,log,out,run.xml,toc}
	rm -f arxiv.tar.gz
	rm -rf arxiv

