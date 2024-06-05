use pandoc_ast::{Inline, MutVisitor};
use pandoc_ast::MetaValue;
use std::collections::BTreeMap;
use std::io::{self, Read, Write};

use lipics_filter::knowledges::{KnowledgeResolver, span_to_knowledge, parse_knowledge_base,
knowledge_to_latex, knowledge_to_fast_latex, knowledge_to_pandoc };
use lipics_filter::utils;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputMode {
    Latex,
    FastLatex,
    Pandoc,
}

struct MyVisitor {
    kdb: KnowledgeResolver,
    mode: OutputMode,
}

impl MutVisitor for MyVisitor {
    fn visit_inline(&mut self, inline: &mut Inline) {
        if self.mode == OutputMode::Pandoc {
            if let Some(knowledge) = span_to_knowledge(inline) {
                *inline = knowledge_to_pandoc(&mut self.kdb, knowledge);
            }
        } 
        self.walk_inline(inline);
    }

    fn visit_vec_inline(&mut self, inlines: &mut Vec<Inline>) {
        if self.mode != OutputMode::Pandoc {
            let mut new_inlines = vec![];
            for inline in inlines.iter_mut() {
                if let Some(knowledge) = span_to_knowledge(inline) {
                    if self.mode == OutputMode::Latex {
                        new_inlines.extend(knowledge_to_latex(knowledge));
                    } else if self.mode == OutputMode::FastLatex {
                        new_inlines.extend(knowledge_to_fast_latex(&mut self.kdb, knowledge));
                    }
                } else {
                    new_inlines.push(inline.clone());
                }
            }
            *inlines = new_inlines;
        } 
        self.walk_vec_inline(inlines);
    }
}

/* 
impl MutVisitor for MyVisitor {
    fn visit_vec_block(&mut self, blocks: &mut Vec<Block>) {
        let mut new_blocks = vec![];
        for block in blocks.iter_mut() {
            if let Some(thm) = block_to_theorem(&mut self.ctx, block.clone()) {
                new_blocks.extend(theorem_to_latex(thm));
            } else {
                new_blocks.push(block.clone());
            }
        }
        *blocks = new_blocks;
        self.walk_vec_block(blocks);
    }

    fn visit_vec_inline(&mut self, inlines: &mut Vec<Inline>) {
        let mut new_inlines = vec![];
        for inline in inlines.iter_mut() {
            if let Some(knowledge) = span_to_knowledge(&mut self.ctx, inline.clone()) {
                new_inlines.extend(knowledge_to_latex(knowledge));
            } else {
                new_inlines.push(inline.clone());
            }
        }
        *inlines = new_inlines;
        self.walk_vec_inline(inlines);
    }
}
*/

#[derive(Debug)]
struct PandocLipics {
    mode:  Option<OutputMode>,
    debug: bool,
}

#[derive(Debug)]
struct Cli {
    format: Option<String>,
}

fn parse_pandoc_lipics(meta: &BTreeMap<String, MetaValue>) -> PandocLipics {
    let mode = {
        let mode_m : Option<String> = utils::meta_deep_get(meta, "lipics.mode").and_then(|x| utils::meta_to_string(&x));
        eprintln!("{:?}", mode_m);
        if let Some(s) = mode_m {
            if s == "latex" {
                Some(OutputMode::Latex)
            } else if s == "fast-latex" {
                Some(OutputMode::FastLatex)
            } else if s == "pandoc" {
                Some(OutputMode::Pandoc)
            } else {
                None
            }
        } else {
            None
        }
    }; 

    let debug = utils::meta_deep_get(meta, "lipics.debug").map(|_| true).unwrap_or(false);

    PandocLipics {
        mode, debug
    }
}


fn main() {
    let mut s = String::new();
    let format = std::env::args().nth(1);

    io::stdin().read_to_string(&mut s).unwrap();
    let s = pandoc_ast::filter(s, |mut pandoc| {
        let db  = parse_knowledge_base(&pandoc.meta);
        let kdb = KnowledgeResolver::new(db);
        let pandoc_lipics = parse_pandoc_lipics(&pandoc.meta);

        // Sane defaults
        // -> if the person did not ask for knowledge explicitly
        // we do not use it.
        eprintln!("{:?}", pandoc_lipics);
        eprintln!("{:?}", format);
        let mode = if let Some(f) = format.as_deref() {
            match f {
                "latex" => pandoc_lipics.mode.unwrap_or(OutputMode::Pandoc),
                _ => OutputMode::Pandoc,
            }
        } else {
            OutputMode::Pandoc
        };

        eprintln!("Mode: {:?}", mode);
        
        let mut visitor = MyVisitor {  kdb, mode };
        visitor.walk_pandoc(&mut pandoc);
        eprintln!("{:?}", visitor.kdb);
        pandoc
    });
    io::stdout().write(s.as_bytes()).unwrap();
}
