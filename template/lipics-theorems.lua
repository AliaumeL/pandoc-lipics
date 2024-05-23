-- Aliaume LOPEZ 2024
--
-- This file is a Lua script that correctly converts pandoc div syntax
-- into correct LIPIcs theorem environments.
--
-- Typical syntax is
--
-- ::: theorem 
-- # optional title
--
-- content
--
-- # optional proof first
--
-- content 
--
-- # optional proof second
--
-- content
--
-- :::
--
--
-- The output is 
--
-- \begin{theorem}[optional title]
-- content
-- \end{theorem}
-- \begin{proof}
-- content
-- \end{proof}
-- \begin{proof}
-- content
-- \end{proof}
--
-- Furthermore, proofs can be selectively sent to appendix
-- by adding an "appendix" attribute.
--
--
-- In HTML mode, the script will output the proofs in expandable
-- environments.
--
--
-- TODO.

-- First we list all the class names that we have to
-- interact with:
local classes = {
  "theorem",
  "lemma",
  "conjecture",
  "claim",
  "corollary",
  "proposition",
  "definition",
  "example",
  "remark",
  "proof"
}

-- For the HTML output, we create a theorem counter
-- <div class="theorem" id="theorem-name">
-- <h3><span>34</span>(theorem name).</h3>
-- theorem content
-- <details>
-- <summary>Proof a</summary>
-- </details>
-- <details>
-- <summary>Proof b</summary>
-- </details>
-- </div>

-- initialise a theorem counter to 0
local theorem_counter = 0

-- Now we create a function that takes a Div element
-- as input, and returns 
local function Div(el)
  -- check if the class of the Div element is in the list
  if el.classes[1] in classes then
    theorem_counter = theorem_counter + 1
  end
end

{
    { Div = Div }
}
