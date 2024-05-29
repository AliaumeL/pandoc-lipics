use nom::{
  IResult,
  bytes::complete::{tag, take_while_m_n},
  combinator::map_res,
  sequence::tuple};

use std::collections::HashMap;

/// A knowledge file has the following format.
///
/// \knowledge{notion}
/// % comment
///   | synonym
/// % comment
///   | synonym @ scope
/// % comment
///
#[derive(Debug)]
pub enum Synonym {
    Global(String),
    Scoped(String, String),
}

#[derive(Debug)]
pub struct Knowledge {
    pub name: String,
    pub synonyms: Vec<Synonym>,
    pub keyval: HashMap<String,String>,
}

#[derive(Debug)]
pub struct KnowledgeBase {
    pub knowledges: Vec<Knowledge>,
    pub source_file: String,
}

/// TODO: implement
pub fn parse_knowledge_base(input : &str) -> IResult<&str, KnowledgeBase> {
    Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Not)))
}

/// A latex macro file
///
/// \NewDocumentCommand{\macroname}{ args }{ definition }
///
///
#[derive(Debug)]
pub enum MacroArg {
    Required,
    Optional,
    OptionalDefault(String)
}

#[derive(Debug)]
pub struct Macro {
    pub name: String,
    pub args: Vec<MacroArg>,
    pub definition: String,
}


/// HEAVILY INSPIRED BY
/// https://hackage.haskell.org/package/HaTeX-3.22.4.1/docs/src/Text.LaTeX.Base.Parser.html#parseLaTeX
///
/// We only parse *mathematical* expressions. And we assume that the input has a nice shape
/// already.
/// 
/// tex ::= \command[]{}{}[]{}
///       | \begin{environment}[]{}[]{} tex \end{environment}
///       | { tex }
///       | % comment
///       | tex tex
///       | string
///
/// example:
///    \sum_{i = 1}^{n} w[i:j[ \times \int{0}{1} f(x) \dx
///
///
///
#[derive(Debug)]
pub enum LaTeX {
    Raw(String),
    Comm(String, Vec<LaTeX>),
    CommS(String),
    Env(String, Vec<LaTeX>, Box<LaTeX>),
    Braces(Box<LaTeX>),
    Comment(String),
    Seq(Box<LaTeX>, Box<LaTeX>),
    Empty,
}
