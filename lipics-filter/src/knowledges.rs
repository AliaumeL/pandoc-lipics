/// Handle knowledges like the knowledge package in LaTeX.
/// Better Knowledges.
/// 
/// - convert knowledge file to yaml
/// - parse [knowledge]{.ref scope=xxx kl=yyy} as \kl[xxx](yyy){knowledge}
/// - parse [knowledge]{.intro} as \intro{knowledge}
use pandoc_ast::{Block,Inline,MutVisitor};
use std::collections::{HashMap,HashSet};


/// Create a newtype for "labels"
/// to avoid confusion with actual strings
#[derive(Debug,Clone,PartialEq,Eq,Hash)]
struct Label(String);

/// Create a newtype for "knowledges-ids"
/// to avoid confusion with actual integers
#[derive(Debug,Clone,PartialEq,Eq,Hash)]
struct KnowledgeId(u16);

#[derive(Debug,Clone)]
enum KnowledgeCommandKind {
    Intro, Reintro, Ref, 
}

#[derive(Debug,Clone)]
struct KnowledgeCommand {
    ident: String,
    kind: KnowledgeCommandKind,
    content: Vec<Inline>,
    name: Option<String>,
    scope: Option<String>,
}

#[derive(Debug,Clone)]
enum KnowledgeSynonym {
    Global(Vec<Inline>),
    Scoped(Vec<Inline>, String)
}

#[derive(Debug,Clone)]
struct KnowledgeEntry {
    label: Label,
    synonyms: Vec<KnowledgeSynonym>,
}

#[derive(Debug,Clone)]
struct KnowledgeBase {
    /// Resolve a label to a knowledge entry
    forward:  Vec<KnowledgeEntry>,
    /// Resolve a synonym to a canonical label
    canonize: HashMap<Label, KnowledgeId>,
}

#[derive(Debug,Clone)]
struct KnowledgeResolver {
    /// The knowledge base to resolve
    knowledge: KnowledgeBase,
    /// The identifiers of spans that referenced the knowledge
    backrefs: Vec<(KnowledgeId, Label)>,
    /// The knowledges that have been introduced 
    /// and their location
    introduced: Vec<(KnowledgeId, Label)>,
}

/// Resolve a label to a knowledge if possible
fn resolve_knowledge<'a>(kdb : &'a KnowledgeBase, l : &Label) -> Option<&'a KnowledgeEntry> {
    match kdb.canonize.get(l) {
        Some(id) => kdb.forward.get(id.0 as usize),
        None => None,
    }
}

/// Converts Inlines to a Label
/// -> dumbest conversion possible
fn inlines_to_label(inlines: &[Inline]) -> Label {
    Label(format!("{:?}", inlines))
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
    let mut name = None;
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
fn span_to_knowledge(span : &Inline) -> Option<KnowledgeCommand> {
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
fn knowledge_to_latex(knowledge: KnowledgeCommand) -> Vec<Inline> {
    eprintln!("Knowledge: {:?}", knowledge);
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

///
/// This functions resolves the knowledge commands at compile time,
/// and thus can be used for any kind of output format (in particular, LaTeX without
/// knowledge installed)
///
fn knowledge_to_pandoc(db: &mut KnowledgeResolver, kl: KnowledgeCommand) -> Inline {
    let knowledge_label = inlines_to_label(&kl.content);
    let kid = resolve_knowledge(&db.knowledge, &knowledge_label);

    // What we should produce
    //
    // 1. if the knowledge is not in the database, we produce a string with the content
    //    and specific classes
    // 2. otherwise, we produce a link to the knowledge, or an anchor to the knowledge.
    //
    match kid {
        None => {
            match kl.kind {
                KnowledgeCommandKind::Intro => {
                        Inline::Span((kl.ident, vec!["kl-intro".to_string(), "kl-undefined".to_string()], vec![]), kl.content)
                }
                KnowledgeCommandKind::Reintro => {
                        Inline::Span((kl.ident, vec!["kl-reintro".to_string(), "kl-undefined".to_string()], vec![]), kl.content)
                }
                KnowledgeCommandKind::Ref => {
                        Inline::Span((kl.ident, vec!["kl-ref".to_string(), "kl-undefined".to_string()], vec![]), kl.content)
                }
            }
        }
        Some(entry) => {
            match kl.kind {
                KnowledgeCommandKind::Intro => {
                        Inline::Span((kl.ident, vec!["kl-intro".to_string(), "kl-defined".to_string()], vec![]), kl.content)
                }
                KnowledgeCommandKind::Reintro => {
                        Inline::Span((kl.ident, vec!["kl-reintro".to_string(), "kl-defined".to_string()], vec![]), kl.content)
                }
                KnowledgeCommandKind::Ref => {
                    let attr = (kl.ident, vec!["kl-ref".to_string(), "kl-defined".to_string()], vec![]);
                    let target = ("".into(),"".into());
                    Inline::Link(attr, kl.content, target)
                }
            }
        }
    }
}

struct MyVisitor {
    ctx: Context,
}

fn meta_to_string(meta : &pandoc_ast::MetaValue) -> Option<String> {
    match meta {
        pandoc_ast::MetaValue::MetaString(s) => Some(s.clone()),
        _ => None
    }
}

fn meta_to_inline(meta : &pandoc_ast::MetaValue) -> Option<Vec<Inline>> {
    match meta {
        pandoc_ast::MetaValue::MetaString(s) => {
            Some(vec![Inline::Str(s.clone())])
        }
        pandoc_ast::MetaValue::MetaInlines(i) => {
            Some(i.clone())
        }
        _ => None
    }
}


fn parse_knowledge_synonym(meta : &pandoc_ast::MetaValue) -> Option<KnowledgeSynonym> {
    use pandoc_ast::MetaValue;
    match meta {
        MetaValue::MetaString(s) => {
            let str_to_inline = Inline::Str(s.clone());
            Some(KnowledgeSynonym::Global(vec![str_to_inline]))
        }
        MetaValue::MetaInlines(i) => {
            Some(KnowledgeSynonym::Global(i.clone()))
        }
        MetaValue::MetaMap(m) => {
            let name = meta_to_inline(m.get("name")?)?;
            let value = meta_to_string(m.get("value")?)?;
            Some(KnowledgeSynonym::Scoped(name, value))
        }
        _ => None
    }
}

fn parse_knowledge_entry(ctx : &mut Context, meta : &pandoc_ast::MetaValue) -> Option<KnowledgeEntry> {
    use pandoc_ast::MetaValue;
    match meta {
        MetaValue::MetaMap(m) => {
            let synonyms = m.get("synonyms")?;
            match synonyms {
                MetaValue::MetaList(l) => {
                    let parsed_synonyms : Vec<KnowledgeSynonym> = l.into_iter().filter_map(parse_knowledge_synonym).collect();
                    if parsed_synonyms.
                }
            }
        }
    }
}

fn build_resolver(ctx: &mut Context, meta: &pandoc_ast::Map<String, pandoc_ast::MetaValue>) { 
    let knowledges = match meta.get("knowledge") {
        Some(pandoc_ast::MetaValue::MetaList(values)) => values,
        _ => return,
    };

    for item in knowledges {
        // create a unique label for the knowledge
        let label = format!("knowledge-{}", ctx.next_theorem());
        let synonyms = match item {
            pandoc_ast::MetaValue::MetaMap(map) => {
                match map.get("synonyms") {
                    Some(value) => {
                        match **value {
                            pandoc_ast::MetaValue::MetaList(values) => values,
                            _ => continue,
                        }
                    }
                    _ => continue,
                }
            }
            _ => {continue}
        };
        let mut knowledge_synonyms = vec![];
        for synonym in synonyms {
            // a synonym is either a string or a map 
            // with name: and scope: fields
            match synonym {
                pandoc_ast::MetaValue::MetaString(name) => {
                    knowledge_synonyms.push((vec![Inline::Str(name.clone())], "".into()));
                }
                pandoc_ast::MetaValue::MetaMap(map) => {
                    let name = match map.get("name") {
                        Some(s) => {
                            match **s {
                                pandoc_ast::MetaValue::MetaString(name) => name,
                                _ => {continue}
                            }
                        }
                        _ => {continue}
                    };
                    let scope = match map.get("scope") {
                        Some(s) => {
                            match **s {
                                pandoc_ast::MetaValue::MetaString(scope) => scope,
                                _ => {continue}
                        }
                        _ => {continue}
                    };
                    knowledge_synonyms.push((vec![Inline::Str(name.clone())], scope.unwrap_or("".to_string())));
                    }
                }
                _ => {continue}
            }
        }
        ctx.knowledge_forward.insert(label.clone(), KnowledgeEntry { label, synonyms: knowledge_synonyms });
        for (synonym, scope) in knowledge_synonyms {
            ctx.knowledge_backward.insert(format!("{:?}::scope::{}", synonym, scope), label.clone());
        }
    }
}
