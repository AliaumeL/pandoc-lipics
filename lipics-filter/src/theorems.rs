/// This module deals with "theorems". However,
/// it is not meant to only contain theorems/lemmas/definitions,
/// but also refer to proofs and other "latex-like" environment
/// definitions.
/// Better Proofs.
///
/// - do not treat proofs and theorems as separate entities
/// - use the "anchor kind" to determine the type of the environment
/// - allow to have "links" between environments (proof-of)
/// - allow to have "references" to environments (cleveref)
///     parse @reference   -> \cref{reference} (if exists)
/// - allow to restate environments (thm-restate)
use pandoc_ast::{Block, Inline};
use std::collections::{HashMap, HashSet};

/// Theorem type in the lipics format.
/// We provide a few standard types
/// plus a custom type that can be used
/// for any other type of theorem that is not
/// covered by the standard ones.
#[derive(Debug)]
enum TheoremKind {
    Theorem,
    Lemma,
    Corollary,
    Proposition,
    Conjecture,
    Claim,
    Custom(String),
}

impl TryFrom<&str> for TheoremKind {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, ()> {
        match s {
            "theorem" => Ok(TheoremKind::Theorem),
            "lemma" => Ok(TheoremKind::Lemma),
            "corollary" => Ok(TheoremKind::Corollary),
            "proposition" => Ok(TheoremKind::Proposition),
            "conjecture" => Ok(TheoremKind::Conjecture),
            "claim" => Ok(TheoremKind::Claim),
            _ if s.starts_with("custom:") => Ok(TheoremKind::Custom(
                s.strip_prefix("custom:").unwrap().to_string(),
            )),
            _ => Err(()),
        }
    }
}

impl From<TheoremKind> for String {
    fn from(tt: TheoremKind) -> String {
        match tt {
            TheoremKind::Theorem => "theorem".to_string(),
            TheoremKind::Lemma => "lemma".to_string(),
            TheoremKind::Corollary => "corollary".to_string(),
            TheoremKind::Proposition => "proposition".to_string(),
            TheoremKind::Conjecture => "conjecture".to_string(),
            TheoremKind::Claim => "claim".to_string(),
            TheoremKind::Custom(s) => format!("{}", s),
        }
    }
}

/// Checks whether a list of classes contains a theorem type.
/// It can be because it is a standard type, or a custom type
/// in which case it is written "custom:<name>"
fn to_theorem_type(classes: &[String]) -> Option<TheoremKind> {
    for class in classes {
        if let Ok(tt) = TheoremKind::try_from(class.as_str()) {
            return Some(tt);
        }
    }
    return None;
}

/// A theorem in the lipics format.
/// it has an optional title
/// an optional label
/// an optional restatable command
/// potential proof elements
/// and contains a list of blocks for
/// the statement of the theorem.
#[derive(Debug)]
struct Theorem {
    title: Option<Vec<Inline>>,
    kind: TheoremKind,
    label: Option<String>,
    restatable: Option<String>,
    proofs: Vec<Proof>,
    statement: Vec<Block>,
    classes: HashSet<String>,
    keyvals: HashMap<String, String>,
}

fn theorem_to_latex(thm: Theorem) -> Vec<Block> {
    let thmtype = String::from(thm.kind);

    let mut blocks = vec![];
    let format = pandoc_ast::Format("latex".to_string());

    let start_block = format!("\\begin{{{}}}", thmtype);
    let end_block = format!("\\end{{{}}}", thmtype);

    // We create the following output
    //
    // \begin{thm_kind}[label={label}, restatable={restatable}, title={title}]
    // statement
    // \end{thm_kind}
    //
    // the first line is a Block::Plain block
    // containing a vector of
    // Inline::RawInline(format, "\\begin{{{thm_kind}}}"),
    // Inline::RawInline(format, "label={label}, restatable={restatable}, title={"),
    // title
    // Inline::RawInline(format, "}]")

    match thm.title {
        Some(mut inlines) => {
            inlines.insert(
                0,
                Inline::RawInline(format.clone(), format!("\\begin{{{}}}[", thmtype)),
            );
            let opts: Vec<String> = vec![
                thm.label.map(|l| format!("label={}", l)),
                thm.restatable.map(|r| format!("restatable={}", r)),
                Some("title={".to_string()),
            ]
            .into_iter()
            .filter_map(|x| x)
            .collect();

            let stropts = opts.join(", ");
            inlines.insert(1, Inline::RawInline(format.clone(), stropts));
            inlines.push(Inline::RawInline(format.clone(), "}]".to_string()));
            blocks.push(Block::Plain(inlines));
        }
        None => {
            let mut inlines = vec![Inline::RawInline(
                format.clone(),
                format!("\\begin{{{}}}", thmtype),
            )];
            let opts: Vec<String> = vec![
                thm.label.map(|l| format!("label={}", l)),
                thm.restatable.map(|r| format!("restatable={}", r)),
            ]
            .into_iter()
            .filter_map(|x| x)
            .collect();
            let stropts = opts.join(", ");
            if !stropts.is_empty() {
                inlines.push(Inline::RawInline(format.clone(), format!("[{}]", stropts)));
            }
            blocks.push(Block::Plain(inlines));
        }
    }

    blocks.extend(thm.statement);

    blocks.push(Block::Plain(vec![Inline::RawInline(
        format.clone(),
        end_block,
    )]));

    for proof in thm.proofs {
        let start_proof = format!("\\begin{{proof}}");
        let end_proof = format!("\\end{{proof}}");
        let start = Inline::RawInline(format.clone(), start_proof);
        let end = Inline::RawInline(format.clone(), end_proof);

        blocks.push(Block::Plain(vec![start]));

        blocks.extend(proof.body);

        blocks.push(Block::Plain(vec![end]));
    }

    blocks
}

/// Kind of possible anchors

/// A context for the conversion.
#[derive(Debug)]
struct Context {
    theorem_counter: u32,
    label_counter: u32,
    // knowledge-forward  : hash -> knowledge
    // knowledge-backward : hash -> hash  (hash of a plain text to the hash of the knowledge)
    knowledge_forward: HashMap<String, KnowledgeEntry>,
    knowledge_backward: HashMap<String, String>,
    //
    // references
    theorems: HashMap<String, Anchor>,
    // forward references (a label points to somewhere)
    // label -> Anchor
    // backward references (a label is pointed by some things)
    // label -> Vec<Anchor>
}

impl Context {
    fn new() -> Context {
        Context {
            theorem_counter: 0,
            label_counter: 0,
            theorems: HashMap::new(),
            knowledge_forward: HashMap::new(),
            knowledge_backward: HashMap::new(),
        }
    }

    fn next_theorem(&mut self) -> u32 {
        let current = self.theorem_counter;
        self.theorem_counter += 1;
        current
    }
}

/// Block to theorem
/// Converts a block to a theorem if possible
/// otherwise returns None.
fn block_to_theorem(ctx: &mut Context, block: Block) -> Option<Theorem> {
    match block {
        Block::Div((ident, classes, keyvals), blocks) => {
            let theorem_type = to_theorem_type(&classes)?;
            let thm_num = ctx.next_theorem();

            let mut title: Option<Vec<Inline>> = None;
            let mut label: Option<String> = None;
            let mut restatable: Option<String> = None;
            let mut proofs: Vec<Proof> = vec![];
            let mut statement: Vec<Block> = vec![];
            let mut classes: HashSet<String> = classes.into_iter().collect();
            let mut keyvals: HashMap<String, String> = keyvals.into_iter().collect();

            let decorated = blocks
                .into_iter()
                .map(|b| match b {
                    Block::Header(lvl, ident, inlines) => Ok((lvl, (ident, inlines))),
                    _ => Err(b),
                })
                .collect();

            let (before, mut after) = split_vec(decorated);

            if !before.is_empty() {
                statement = before;
            } else {
                let (block_title, block_statement) = after.remove(0);
                statement = block_statement;
                let (_, ((id, cls, kvl), inlines)) = block_title;
                title = Some(inlines);
                // update the identifier
                if !id.is_empty() {
                    label = Some(id)
                }
                // add new classes
                classes.extend(cls);
                // add new keyvals
                keyvals.extend(kvl);
            }

            // now we collect the "proof blocks" that are in the after
            // array
            let proofs = after
                .into_iter()
                .map(|(block_title, block_proof)| {
                    let (_, ((id, classes, keyvals), inlines)) = block_title;
                    let cls: HashSet<String> = classes.into_iter().collect();
                    let kvl: HashMap<String, String> = keyvals.into_iter().collect();
                    let kind = match cls.contains("sketch") {
                        true => ProofKind::Sketch,
                        false => ProofKind::Proof,
                    };
                    let status = match cls.contains("appendix") {
                        true => ProofStatus::Hidden,
                        false => ProofStatus::Important,
                    };
                    let title = if !inlines.is_empty() {
                        Some(inlines)
                    } else {
                        None
                    };
                    let label = if !id.is_empty() { Some(id) } else { None };
                    Proof {
                        title,
                        label,
                        status,
                        kind,
                        body: block_proof,
                        classes: cls,
                        keyvals: kvl,
                    }
                })
                .collect();

            Some(Theorem {
                title,
                kind: theorem_type,
                label,
                restatable,
                proofs,
                statement,
                classes,
                keyvals,
            })
        }
        _ => None,
    }
}
