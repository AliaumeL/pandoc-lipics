---
lipics:
    mode: pandoc
knowledges:
    - synonyms:
        - salut c’est moi
        - coucou
    - synonyms:
        - name: hello
          scope: global
        - name: hihi
          scope: local
---

Let me introduce [coucou]{.intro} which is a nice knowledge. This knowledge is
also known as [salut c’est moi]{.reintro}. 

It means that in particular, we can reference them via either [coucou]{.ref} or
[salut c’est moi]{.ref}.

Now, let us discuss scopes: [hello]{.ref scope=global} and [hihi]{.ref
scope=local}. both should refer to the same knowledge, [hello]{.intro
scope=global}.

Now, imagine we want to refer to a 
[yet undefined knowledge]{.ref kl="salut c’est moi"} .
