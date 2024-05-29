# Pandoc LIPIcs Template

The goal of this repository is to allow the usage of Pandoc to write papers in
the LIPIcs template. This is a work in progress, but the features that are
planned are the following:

- [x] Create a pandoc template file with options adapted to the LIPIcs
  template.
- [x] Create an html template that mimics the LIPIcs style.
- [ ] Create a docker image with pandoc and the rust toolchain
- [ ] Create a CI/CD environment
- [ ] Create a filter to allow usage of the theorem environments in the pandoc
  markdown.
- [ ] Find a way to make bibliographies work with the LIPIcs style, that only
  allows plain `bibtex` commands.
- [ ] Create a filter to have the appendix be part of the main document
- [ ] Create a filter to automatically generate an appendix 
- [ ] Create a filter to ease the use of the knowledge package
- [ ] Create a filter to allow the use of "custom macros"
- [ ] Provide a Makefile to automatically generate the `review`, `final` and
  `arxiv` versions of the paper.
