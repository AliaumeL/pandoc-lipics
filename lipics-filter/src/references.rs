/// This module deals with the references
/// in the pandoc markdown file, and tries
/// to "mimic" the behaviour of cleveref.
///
/// For now:
///
/// @my-figure-label    -> Figure 1
/// @my-theorem         -> Theorem 7
/// @undefined-label    -> ???
///
use pandoc_ast::Inline;

/// Possible anchors in the document
#[derive(Debug)]
enum AnchorKind {
    Theorem,
    Lemma,
    Corollary,
    Proposition,
    Conjecture,
    Claim,
    Figure,
    Algorithm,
    Table,
    Definition,
    Remark,
    Example,
    Proof,
    Item,
    Equation,
    Section,
}

/// An anchor in the document
#[derive(Debug)]
struct Anchor {
    label: String,
    title: Option<Vec<Inline>>,
    kind: AnchorKind,
}

/// A reference in the document
#[derive(Debug)]
struct Reference {
    label: String,
    kind: AnchorKind,
}
