# start from the pandoc/latex:latest image
# which is an alpine linux image with pandoc and texlive installed
FROM pandoc/latex:latest

# install `make` for build scripts
# install `git` for version control
# install `inkscape` for svg manipulations
# install `entr` for live reload
# install `websocat` for live reload
RUN apk add --no-cache make
RUN apk add --no-cache git
RUN apk add --no-cache inkscape
RUN apk add --no-cache entr
RUN apk add --no-cache websocat
