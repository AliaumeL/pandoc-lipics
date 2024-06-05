/// Handle knowledges like the knowledge package in LaTeX.
/// Better Knowledges.
/// 
/// - convert knowledge file to yaml
/// - parse [knowledge]{.ref scope=xxx kl=yyy} as \kl[xxx](yyy){knowledge}
/// - parse [knowledge]{.intro} as \intro{knowledge}
///
///
/// TODO:
/// - [ ] Add debug informations
/// - [ ] Test
/// - [ ] Add information back to the metadata (introduced, unknown, backrefs)

use std::hash::Hash;
use pandoc_ast::{Inline, MetaValue};
use std::collections::{HashMap, BTreeMap};

use crate::utils;

/// Create a newtype for "knowledges-ids"
/// to avoid confusion with actual integers
#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
pub struct KnowledgeId(u16);


/// The knowledge commands that can be issued
/// in the document.
#[derive(Debug,Clone)]
pub enum KnowledgeCommandKind {
    /// Introduces a knowledge (creates an anchor)
    Intro, 
    /// Re-introduce a knowledge (does not create an anchor)
    Reintro, 
    /// References a knowledge
    Ref, 
}


/// An internal representation of the knowledge command
/// to be issued. This is language agnostic.
#[derive(Debug,Clone)]
pub struct KnowledgeCommand {
    /// the identifier of the command (for back references)
    ident: String,
    /// the kind of command
    kind: KnowledgeCommandKind,
    /// The textual content of the command 
    content: Vec<Inline>,
    /// An optional specified name for the knowledge
    name: Option<String>,
    /// An optional specified scope for the knowledge
    scope: Option<String>,
}


#[derive(Debug,Clone)]
pub enum KnowledgeSynonym {
    Global(Vec<Inline>),
    Scoped(Vec<Inline>, String)
}


impl KnowledgeSynonym {
    pub fn to_string(&self) -> String {
        match self {
            KnowledgeSynonym::Global(i) => {
                utils::stringify_inlines(&i)
            }
            KnowledgeSynonym::Scoped(i,s) => {
                format!("{}@{}", utils::stringify_inlines(&i), s)
            }
        }
    }
}


impl PartialEq for KnowledgeSynonym {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}

impl Eq for KnowledgeSynonym {}

impl Hash for KnowledgeSynonym {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            KnowledgeSynonym::Global(i) => {
                format!("{:?}", i).hash(state);
            }
            KnowledgeSynonym::Scoped(i,s) => {
                format!("{:?}@{:?}", i, s).hash(state);
            }
        }
    }
}

#[derive(Debug,Clone)]
pub struct KnowledgeEntry {
    synonyms: Vec<KnowledgeSynonym>,
}

#[derive(Debug,Clone)]
pub struct KnowledgeBase {
    /// Resolve a label to a knowledge entry
    forward:  Vec<KnowledgeEntry>,
    /// Resolve a synonym to a canonical label
    canonize: HashMap<KnowledgeSynonym, KnowledgeId>,
}

#[derive(Debug,Clone)]
pub struct KnowledgeResolver {
    /// The knowledge base to resolve
    knowledge: KnowledgeBase,
    /// The identifiers of spans that referenced the knowledge
    backrefs: Vec<(KnowledgeId,KnowledgeCommand)>,
    /// The knowledges that have been introduced 
    /// and their location
    introduced: Vec<(KnowledgeId,KnowledgeCommand)>,
    /// Unknown knowledges
    unknown: Vec<KnowledgeCommand>,
}

impl KnowledgeResolver {
    pub fn new(db: KnowledgeBase) -> KnowledgeResolver {
        KnowledgeResolver { knowledge: db, backrefs: vec![], introduced: vec![], unknown: vec![] }
    }
}

/// Resolve a label to a knowledge if possible
fn resolve_knowledge<'a>(kdb : &'a KnowledgeBase, l : &KnowledgeSynonym) -> Option<(KnowledgeId, &'a KnowledgeEntry)> {
    match kdb.canonize.get(l) {
        Some(id) => match kdb.forward.get(id.0 as usize) {
            Some(entry) => Some((*id, entry)),
            None => None,
        }
        None => None,
    }
}

fn classes_to_knowledge_kind(classes: &[String]) -> Option<KnowledgeCommandKind> {
    if classes.contains(&"intro".to_string()) {
        Some(KnowledgeCommandKind::Intro)
    } else if classes.contains(&"reintro".to_string()) {
        Some(KnowledgeCommandKind::Reintro)
    } else if classes.contains(&"ref".to_string()) {
        Some(KnowledgeCommandKind::Ref)
    } else {
        None
    }
}

fn keyvals_to_knowledge_command(keyvals: &[(String,String)]) -> (Option<String>, Option<String>) {
    let mut name  = None;
    let mut scope = None;
    for (k,v) in keyvals {
        if k == "kl" {
            name = Some(v.clone());
        } else if k == "scope" {
            scope = Some(v.clone());
        }
    }
    (name, scope)
}

/// Parses a span into a knowledge command if possible
/// - [knowledge]{.intro} -> KnowledgeCommand { kind: Intro, content: [knowledge] }
/// - [knowledge]{.reintro} -> KnowledgeCommand { kind: Reintro, content: [knowledge] }
/// - [knowledge]{.ref scope=xxx kl=yyy} -> KnowledgeCommand { kind: Ref, content: [knowledge], name: Some("yyy"), scope: Some("xxx") } 
pub fn span_to_knowledge(span : &Inline) -> Option<KnowledgeCommand> {
    match span {
        Inline::Span((ident, classes, keyvals), inlines) => {
            let (name, scope) = keyvals_to_knowledge_command(keyvals);
            let kind = classes_to_knowledge_kind(classes)?;
            Some(KnowledgeCommand { ident: ident.clone(), kind, content: inlines.clone(), name, scope })
        }
        _ => None,
    }
}

/// Transforms a knowledge command into the corresponding
/// LaTeX code.
///
/// This function delegates the actual resolution to the
/// `knowledge` package in LaTeX, and thus requires a
/// specific preamble in the document.
pub fn knowledge_to_latex(knowledge: KnowledgeCommand) -> Vec<Inline> {
    let format = pandoc_ast::Format("latex".to_string());
    let mut inlines = vec![];
    let params = match (knowledge.scope, knowledge.name) {
        (Some(scope),Some(name)) => format!("({})[{}]", scope, name),
        (Some(scope),None) => format!("({})", scope),
        (None,Some(name)) => format!("[{}]", name),
        (None,None) => "".to_string(),
    };
    match knowledge.kind {
        KnowledgeCommandKind::Intro => {
            inlines.push(Inline::RawInline(format.clone(), format!("\\intro{}{{", params)));
            inlines.extend(knowledge.content);
            inlines.push(Inline::RawInline(format.clone(), "}".to_string()));
        }
        KnowledgeCommandKind::Reintro => {
            inlines.push(Inline::RawInline(format.clone(), format!("\\reintro{}{{", params)));
            inlines.extend(knowledge.content);
            inlines.push(Inline::RawInline(format.clone(), "}".to_string()));
        }
        KnowledgeCommandKind::Ref => {
            inlines.push(Inline::RawInline(format.clone(), format!("\\kl{}{{", params)));
            inlines.extend(knowledge.content);
            inlines.push(Inline::RawInline(format.clone(), "}".to_string()));
        }
    }
    inlines 
}

/// Transforms a knowledge command into the corresponding
/// LaTeX code. This *avoids* using the knowledge package,
/// but still emits low-level LaTeX code that allows to
/// compile the document *in a single pass*.
///
/// the resolution of the knowledges is done at compile time
/// and the code outputs the following macros,
/// that can be suitably modified in the preamble of the document:
///
/// \akldef{unique-id}{content}
/// \aklref{unique-id}{content}
/// \aklredef{unique-id}{content}
/// \akldeferror{content}
/// \aklreferror{content}
/// \aklredeferror{content}
///
/// TODO: add the possibility to compute the backreferences
/// and list introduced, duplicated, and unknown knowledges.
///
pub fn knowledge_to_fast_latex(db : &mut KnowledgeResolver, kl: KnowledgeCommand) -> Vec<Inline> {
    let format = pandoc_ast::Format("latex".to_string());
    let synonym = match (&kl.scope, &kl.name) { 
        (Some(scope), Some(name)) => KnowledgeSynonym::Scoped(vec![Inline::Str(name.clone())], scope.clone()),
        (Some(scope), None)       => KnowledgeSynonym::Scoped(kl.content.clone(), scope.clone()),
        (None       , Some(name)) => KnowledgeSynonym::Global(vec![Inline::Str(name.clone())]),
        (None       , None)       => KnowledgeSynonym::Global(kl.content.clone()),
    };
    let mut inlines = vec![];
    match resolve_knowledge(&db.knowledge, &synonym) {
        None => {
            // We do not have a knowledge entry: this is problematic
            // and we should issue a warning.
            db.unknown.push(kl.clone());
            match kl.kind {
                KnowledgeCommandKind::Intro => {
                    inlines.push(Inline::RawInline(format.clone(), "\\akldeferror{".to_string()));
                }
                KnowledgeCommandKind::Reintro => {
                    inlines.push(Inline::RawInline(format.clone(), "\\aklredeferror{".to_string()));
                }
                KnowledgeCommandKind::Ref => {
                    inlines.push(Inline::RawInline(format.clone(), "\\aklreferror{".to_string()));
                }
            }
        }
        Some((kid, _)) => {
            let kl_unique_id = format!("kl-{}", kid.0);
            match kl.kind {
                KnowledgeCommandKind::Intro => {
                    db.introduced.push((kid, kl.clone()));
                    inlines.push(Inline::RawInline(format.clone(), format!("\\akldef{{{}}}{{", kl_unique_id)));
                }
                KnowledgeCommandKind::Reintro => {
                    inlines.push(Inline::RawInline(format.clone(), format!("\\aklredef{{{}}}{{", kl_unique_id)));
                }
                KnowledgeCommandKind::Ref => {
                    inlines.push(Inline::RawInline(format.clone(), format!("\\aklref{{{}}}{{", kl_unique_id)));
                    db.backrefs.push((kid, kl.clone()));
                }
            }
        }
    }
    inlines.extend(kl.content);
    inlines.push(Inline::RawInline(format.clone(), "}".to_string()));
    inlines
}

///
/// This functions resolves the knowledge commands at compile time,
/// and thus can be used for any kind of output format (in particular, LaTeX without
/// knowledge installed)
///
pub fn knowledge_to_pandoc(db: &mut KnowledgeResolver, mut kl: KnowledgeCommand) -> Inline {
    let synonym = match (&kl.scope, &kl.name) { 
        (Some(scope), Some(name)) => KnowledgeSynonym::Scoped(vec![Inline::Str(name.clone())], scope.clone()),
        (Some(scope), None)       => KnowledgeSynonym::Scoped(kl.content.clone(), scope.clone()),
        (None       , Some(name)) => KnowledgeSynonym::Global(vec![Inline::Str(name.clone())]),
        (None       , None)       => KnowledgeSynonym::Global(kl.content.clone()),
    };
    match resolve_knowledge(&db.knowledge, &synonym) {
        None => {
            // We do not have a knowledge entry: this is problematic
            // and we should issue a warning.
            db.unknown.push(kl.clone());
            match kl.kind {
                KnowledgeCommandKind::Intro => {
                        Inline::Span((kl.ident, vec!["kl-intro".to_string(), "kl-undefined".to_string()], vec![]),
                        vec![Inline::Emph(kl.content)])
                }
                KnowledgeCommandKind::Reintro => {
                        Inline::Span((kl.ident, vec!["kl-reintro".to_string(), "kl-undefined".to_string()], vec![]), 
                        vec![Inline::Emph(kl.content)])
                }
                KnowledgeCommandKind::Ref => {
                        Inline::Span((kl.ident, vec!["kl-ref".to_string(), "kl-undefined".to_string()], vec![]), kl.content)
                }
            }
        }
        Some((kid, entry)) => {
            let kl_unique_id = format!("kl-{}", kid.0);
            match kl.kind {
                KnowledgeCommandKind::Intro => {
                    let attrs = (kl_unique_id,
                                vec!["kl-intro".to_string(), "kl-defined".to_string()],
                                vec![("kl".into(), kid.0.to_string())]);
                    db.introduced.push((kid, kl.clone()));
                    Inline::Span(attrs, vec![Inline::Emph(kl.content)])
                }
                KnowledgeCommandKind::Reintro => {
                    let attrs = ("".into(),
                                 vec!["kl-reintro".to_string(), "kl-defined".to_string()],
                                 vec![("kl".into(), kid.0.to_string())]);
                    Inline::Span(attrs, vec![Inline::Emph(kl.content)])
                }
                KnowledgeCommandKind::Ref => {
                    if kl.ident == "" {
                        kl.ident = format!("kref-{}", db.backrefs.len());
                    }
                    let attrs = (kl.ident.clone(), vec!["kl-ref".to_string(), "kl-defined".to_string()], vec![]);
                    let title = format!("Reference to {}", entry.synonyms[0].to_string());
                    let target : (String, String) = (format!("#kl-{}", kid.0), title);
                    db.backrefs.push((kid, kl.clone()));
                    Inline::Link(attrs, kl.content, target)
                }
            }
        }
    }
}

///
/// This function parses a knowledge entry from the metadata
/// of a pandoc document. 
///
/// It is either of the form 
/// string                          -> global knowledge
/// { name: string, scope: string } -> scoped knowledge
///
fn parse_knowledge_synonym(meta : &MetaValue) -> Option<KnowledgeSynonym> {
    match meta {
        MetaValue::MetaString(s) => {
            let str_to_inline = Inline::Str(s.clone());
            Some(KnowledgeSynonym::Global(vec![str_to_inline]))
        }
        MetaValue::MetaInlines(i) => {
            Some(KnowledgeSynonym::Global(i.clone()))
        }
        MetaValue::MetaMap(m) => {
            let name = utils::meta_to_inline(m.get("name")?)?;
            let scope = utils::meta_to_string(m.get("scope")?)?;
            Some(KnowledgeSynonym::Scoped(name, scope))
        }
        _ => None
    }
}


///
/// Parses a knowledge entry inside the metadata of a pandoc document.
/// For now, a knowledge entry is simply a list of synonyms. But in the future,
/// we may add more information (url, description, bibentry, etc.)
///
fn parse_knowledge_entry(meta : &MetaValue) -> Option<KnowledgeEntry> {
    if let MetaValue::MetaMap(m) = meta {
        let synonyms = m.get("synonyms")?;
        match &**synonyms {
            MetaValue::MetaList(l) => {
                let synonyms = l.into_iter().filter_map(|e| parse_knowledge_synonym(&e) ).collect();
                Some(KnowledgeEntry { synonyms })
            }
            _ => None
        }
    } else {
        None
    }
}

///
/// Parses a list of knowledge entries from the metadata of a pandoc document.
///
fn parse_knowledge_entries(meta : &MetaValue) -> Option<Vec<KnowledgeEntry>> {
    if let MetaValue::MetaList(l) = meta {
        Some(l.into_iter().filter_map(|e| parse_knowledge_entry(&e)).collect())
    } else {
        None
    }
}

/// Parses a knowledge base from 
/// the metadata of a pandoc document.
pub fn parse_knowledge_base(meta : &BTreeMap<String, MetaValue>) -> KnowledgeBase {
    let entries = meta.get("knowledges").and_then(|x| parse_knowledge_entries(&*x));
    if let Some(forward) = entries {
        let mut canonize = HashMap::new();
        for (i,entry) in forward.iter().enumerate() {
            let id = KnowledgeId(i as u16);
            for syn in &entry.synonyms {
                canonize.insert(syn.clone(), id);
            }
        }
        KnowledgeBase { forward, canonize }
    } else {
        KnowledgeBase { forward: vec![], canonize: HashMap::new() }
    }
}
