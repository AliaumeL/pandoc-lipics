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
    review-mode: False
    arxiv-mode: True
    anonymous: True
    debug: true
    mode: latex
related-version:
    - type: "Preprint"
      url: "https://arxiv.org/abs/XXX"
supplement:
    - type: "Data"
      url: "http://dx.doi.org/10.4230/LIPIcs.xxx.xxx.xxx"
acknowledgement: "We would like to thank our colleagues for their feedback."
bibliography: lipics-v2021-sample-article.bib
knowledges:
    - synonyms:
        - this
        - that
        - name: those
          scope: testing-scope
abstract: |
    Abstract of the paper
...


# Introduction

## Theorems and proofs

::: theorem :::
### Optional Theorem title

Theorem statement

# Proof

proof statement 


:::

::: {.claim restate="firstclaim" title="My Title" #firstclaimlabel}

Let $x$ be a variable, we can do this and this and that.

Then in particular:

i. $x$ is a variable.
ii. $x$ is a variable.

All of the above are equivalent to $x$ being a variable.

# Proof Sketch

:::

## Knowledges


We can introduce knowledges with [this]{.intro}
and later on refer to those using [this]{.ref}.
If for some strange reason we want to introduce them twice,
we can use [that]{.reintro}.

If we want to use a scoped knowledge, we can like this [those]{.ref
scope=testing-scope}.

## Citations 

We start by citing a paper [@DBLP:journals/cacm/Knuth74].

We can also cite them like this @DBLP:journals/cacm/Knuth74. Which becomes a
tiny bit more impressive using a lot of names such as
@DBLP:conf/focs/HopcroftPV75.

Note that we have the full power of the pandoc citation syntax. In particular
we can [see @DBLP:journals/cacm/Knuth74; because @DBLP:conf/focs/HopcroftPV75
has Theorem 1.6].

# Main part 

Imagine some text followed by a theorem

<div class="theorem">
<h3><span class="number" data-number="1">Theorem 1</span> (good omens).</h3>
<p>There are good omens.</p>
<ul>
<li>Good omen 1.</li>
<li>Good omen 2.</li>
</ul>
<details>
<summary>Proof</summary>
<p>Proof of Theorem 1.</p>
</details>
</div>
