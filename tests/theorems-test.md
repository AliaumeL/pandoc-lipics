---
title: "Sample Article for LIPIcs"
title-running: "Sample LIPIcs Running"
author:
    - name: "John Q. Public"
      affiliation: "Dummy University Computing Laboratory"
      email: "dummy-email"
      orcid: "dummy-orcid"
      funding: "some funding"
bibliography: lipics-v2021-sample-article.bib
abstract: |
    Abstract of the paper
refs: |
    ::: {#refs}
    :::
...


# Introduction

## Theorems and proofs

::: theorem :::
### Optional Theorem title

Theorem statement

# Proof {of=theorem}

proof statement 
:::

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
[Knowledge]{.intro}



