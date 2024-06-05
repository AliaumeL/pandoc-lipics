use pandoc_ast::{MetaValue, Inline};
use std::collections::BTreeMap;

/// Converts a MetaValue to a string if it is a MetaString
pub fn meta_to_string(meta : &MetaValue) -> Option<String> {
    match meta {
        MetaValue::MetaString(s) => Some(s.clone()),
        MetaValue::MetaInlines(l) if l.len() == 1 => {
            if let Some(Inline::Str(s)) = l.first() {
                Some(s.clone())
            } else {
                None
            }
        }
        _ => None
    }
}

/// Converts a MetaValue to a vector of inlines if it is a MetaString or MetaInlines
pub fn meta_to_inline(meta : &MetaValue) -> Option<Vec<Inline>> {
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

pub fn stringify_inlines(i : &[Inline]) -> String {
    i.iter().map(|x| stringify(x)).collect::<Vec<String>>().join("")
}

pub fn stringify(i : &Inline) -> String {
    match i {
        Inline::Str(s) => s.clone(),
        Inline::Note(i) => "".into(),
        Inline::Emph(i) => stringify_inlines(i),
        Inline::Strong(i) => stringify_inlines(i),
        Inline::Underline(i) => stringify_inlines(i),
        Inline::Strikeout(i) => stringify_inlines(i),
        Inline::Superscript(i) => stringify_inlines(i),
        Inline::Subscript(i) => stringify_inlines(i),
        Inline::SmallCaps(i) => stringify_inlines(i),
        Inline::Quoted(_,i) => stringify_inlines(i),
        Inline::Cite(_,i) => stringify_inlines(i),
        Inline::Code(_,s) => s.clone(),
        Inline::Space => " ".into(),
        Inline::SoftBreak => "\n".into(),
        Inline::LineBreak => "\n".into(),
        Inline::Math(_,s) => s.clone(),
        Inline::RawInline(_,s) => s.clone(),
        Inline::Link(_,i,t) => stringify_inlines(i),
        Inline::Image(_,_,_) => "".into(),
        Inline::Span(_,i) => stringify_inlines(i),
    }
}


#[derive(Debug)]
enum MetaSelect {
    Field(String),
    Index(usize),
}

fn meta_deep_get_internal(meta : &MetaValue, path : &[MetaSelect]) -> Option<MetaValue> {
    let mut current = meta;
    for p in path {
        match p {
            MetaSelect::Field(f) => {
                if let MetaValue::MetaMap(m) = current {
                    if let Some(next) = m.get(f) {
                        current = next;
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            MetaSelect::Index(i) => {
                if let MetaValue::MetaList(l) = current {
                    if let Some(next) = l.get(*i) {
                        current = next;
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
        }
    }
    Some(current.clone())
}

fn string_to_selector(s : &str) -> Vec<MetaSelect> {
    s.split('.').map(|x| {
        if let Ok(i) = x.parse::<usize>() {
            MetaSelect::Index(i)
        } else {
            MetaSelect::Field(x.to_string())
        }
    }).collect()
}



pub fn meta_deep_get(meta : &BTreeMap<String, MetaValue>, path : &str) -> Option<MetaValue> {
    let selector     = string_to_selector(path);
    let (first,rest) = selector.split_at(1);
    if let Some(MetaSelect::Field(f)) = first.iter().nth(0) {
        if let Some(m) = meta.get(f) {
            meta_deep_get_internal(m, rest)
        } else {
            None
        }
    } else {
        None
    }
}

