use pandoc_ast::{Block, Inline, MutVisitor};
use std::collections::{HashMap, HashSet};
use std::io::{self, Read, Write};

/// Ultimately, perform all the computations in this preprocessor, even for LaTeX output,
/// so that we have a "one pass compilation" of the document for LaTeX, to speed up the
/// view time. Note that for tikz pictures, this is irrelevant because we would have
/// to parse them to get proper cross-references.
///
/// Better Figures.
///
/// - parse ![caption](url){.figure} to add it to the figures list
/// - allow to use SVG or TIKZ code directly in the document
///     -> for tikz, in latex, this is just plain latex code
///     -> in other formats, collect all the latex codes, and compile
///     a standalone latex document with the tikz code to get a pdf
///     of the image, that is then rasterized into a low-res png.
///
/// Better Tables?
///
/// - TODO.
///
/// Better Algorithms.
///
/// - parse ```{=name .algorithm}``` to add it to the list of algorithms,
///   and create a nicely formatted algorithm environment in LaTeX.
///
/// Better Macros.
///
/// -> for the moment, we will also use the "standalone" compilation of the math components
/// to get png images of the macros. This is way simpler.
///
/// Checking.
///
/// - check that every theorem has a proof (or proof sketch): provide the list of theorems without proofs
/// - check that knowledges are introduced before they are used.
/// - check for consistency in the references.
/// - provide an estimated number of pages.
///
use lipics_filter::polyreg::split_vec;

/// A proof kind in the lipics format.
/// Proof is a direct proof, that should be shown
/// Sketch is a proof sketch
#[derive(Debug)]
enum ProofKind {
    Proof,
    Sketch,
}

/// Proof status in the lipics format.
/// Either it is important (main body)
/// or should be hidden (appendix / details)
#[derive(Debug)]
enum ProofStatus {
    Important,
    Hidden,
}

/// A proof in the lipics format
/// proof kind = "proof" | "proofof" | "sketch"
/// proof body = block+
#[derive(Debug)]
struct Proof {
    title: Option<Vec<Inline>>,
    status: ProofStatus,
    kind: ProofKind,
    label: Option<String>,
    body: Vec<Block>,
    classes: HashSet<String>,
    keyvals: HashMap<String, String>,
}

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

fn main() {
    let mut s = String::new();
    let mut visitor = MyVisitor {
        ctx: Context::new(),
    };
    io::stdin().read_to_string(&mut s).unwrap();
    let s = pandoc_ast::filter(s, |mut pandoc| {
        build_resolver(&mut visitor.ctx, &pandoc.meta);
        visitor.walk_pandoc(&mut pandoc);
        pandoc
    });
    io::stdout().write(s.as_bytes()).unwrap();
}
