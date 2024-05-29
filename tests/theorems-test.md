---
title: "Sample Article for LIPIcs"
title-running: "Sample LIPIcs Running"
lipics:
    composition-mode: true
author:
    - name: "John Q. Public"
      affiliation: "Dummy University Computing Laboratory"
      email: "dummy-email"
      orcid: "dummy-orcid"
      funding: "some funding"
bibliography: lipics-v2021-sample-article.bib
abstract: |
    Abstract of the paper
knowledges:
  - synonyms:
    - alpha
    - beta 
    - gamma
refs: |
    ::: {#refs}
    :::
...


# Introduction

We introduce [alpha]{.intro} and [beta]{.reintro} following the work of
@DBLP:conf/focs/HopcroftPV75.


## Theorems and proofs

::: theorem :::
### Optional Theorem title

Theorem statement

# Proof {of=theorem}

proof statement 
:::

\begin{equation*}
x = x + 1
\end{equation*}

::: {.claim restate="firstclaim" title="My Title" #firstclaimlabel}

Let $x$ be a variable, we can do this and this and that.

Then in particular:

i. $x$ is a variable.
ii. $x$ is a variable.

All of the above are equivalent to $x$ being a variable.

# Proof Sketch

This is a proof sketch.
:::

```{=tikz caption="A TikZ picture" label="fig:tikz"}
\begin{tikzpicture}
\draw (0,0) -- (1,1);
\end{tikzpicture}
```

# Cleveref

Now, we can reference to @optional-theorem-title and [Create a link](#optional-theorem-title).

# References 

[beta]{.ref}



