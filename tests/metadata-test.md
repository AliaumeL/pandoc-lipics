---
title: "Sample Article for LIPIcs"
title-running: "Sample LIPIcs Running"
author:
    - name: "John Q. Public"
      affiliation: "Dummy University Computing Laboratory"
      email: "dummy-email"
      orcid: "dummy-orcid"
    - name: "John Q. Public"
      affiliation: "Dummy University Computing Laboratory"
      email: "dummy-email"
      orcid: "dummy-orcid"
      funding: "some funding"
acm-cc-desc:
    - Theory of computation
    - Automata theory
keyword: ["First", "Second", "Third"]
lipics:
    category: "Invited Paper"
    review-mode: True
    arxiv-mode: False
    anonymous: True
related-version:
    - type: "Preprint"
      url: "https://arxiv.org/abs/XXX"
supplement:
    - type: "Data"
      url: "http://dx.doi.org/10.4230/LIPIcs.xxx.xxx.xxx"
acknowledgement: "We would like to thank our colleagues for their feedback."
bibliography: lipics-v2021-sample-article.bib
abstract: |
    Abstract of the paper
refs: |
    ::: {#refs}
    :::
...


# Introduction

Some Introduction 

## Citations 

We start by citing a paper [@DBLP:journals/cacm/Knuth74].

We can also cite them like this @DBLP:journals/cacm/Knuth74. Which becomes
a tiny bit more impressive using a lot of names such as
@DBLP:conf/focs/HopcroftPV75.

Note that we have the full power of the pandoc citation syntax. In particular
we can [see @DBLP:journals/cacm/Knuth74; because @DBLP:conf/focs/HopcroftPV75
has Theorem 1.6].
