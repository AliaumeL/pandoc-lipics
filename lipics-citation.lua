-- Aliaume LOPEZ
-- 2024
--
-- This file is a Lua script that correctly converts
-- pandoc citations into classical "bibfile" citations
-- for the LIPIcs conference proceedings series. 
--
-- The script must be run *after* pandoc-citeproc.
-- 
-- It turns citations of the form [@citekey] into
-- \cite{citekey} and citations of the form [see @citekey]
-- and @citekey into \hyperlink{cite.citekey}{Authors}
-- respectively.
--
-- TODO: add possibility to create hyperlinks with page numbers
--

local function Cite(citations)
  -- we assert that there is at least one citation
  if #citations.citations == 0 then
    error("Only one citation is allowed per Cite element")
  end

  -- we fetch the first citation from the list to know whether
  -- we should be AuthorInText or not
  local cite = citations.citations[1]
  local intext = cite.mode == "AuthorInText"

  if intext and #citations.citations > 1 then
      return "#ERROR: author-in-text mode with multiple citations is undefined"
  -- otherwise if intext and there is only one citation
  elseif intext then
    -- we simply wrap the content of the citations object
    -- into a hyperlink to the corresponding bibliographic entry
    -- but we remove the date in parentheses at the end of the content
    -- because it is not needed in the LIPIcs style
    -- 
    -- To that end, we remove the two last elements of the citations.content
    -- because I don’t know any lua, this is the terrible way to do it.
    local content = {}
    for i = 1, #citations.content - 2 do
      table.insert(content, citations.content[i])
    end
    -- Note the \nocite{} command that is used to ensure that the
    -- citation is included in the bibliography in the end!
    return (
      { pandoc.RawInline("latex", "\\nocite{" .. cite.id .."}\\hyperlink{cite." .. cite.id .. "}{") }
      .. pandoc.Inlines(content) ..
      { pandoc.RawInline("latex", "}") } )
  else 
    local content = pandoc.Inlines({})
    content = content .. { pandoc.Str("[") }
    local number = #citations.citations
    for i, citation in ipairs(citations.citations) do
      -- if there is a prefix that has at least one element
      if citation.prefix and #citation.prefix > 0 then
         content = content .. citation.prefix .. { pandoc.Str(" ")}
      end
      content = content .. { pandoc.RawInline("latex", "\\cite{" .. citation.id .. "}") }
      if (i < number) or (citation.suffix and #citation.suffix > 0) then
          content = content .. { pandoc.Str(" ") } .. citation.suffix
      end
    end
    content = content .. { pandoc.Str("]") }

    return content
    -- local ids = {}
    -- for _, citation in ipairs(citations.citations) do
    --   table.insert(ids, citation.id)
    -- end
    -- local bibref = table.concat(ids, ",")
    -- return pandoc.RawInline("latex", "\\cite[{" ..  .. "}]{" .. bibref .. "}")
  end
end

return {
    { Cite = Cite }
}
