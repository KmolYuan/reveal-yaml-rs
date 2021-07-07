use super::*;
use pulldown_cmark::{html::push_html, CodeBlockKind, Event, Options, Parser, Tag};
use std::{collections::HashMap, fs::read_to_string};
use yaml_peg::{Node, Yaml};

const MARKED: Options = Options::from_bits_truncate(
    Options::ENABLE_TABLES.bits()
        | Options::ENABLE_SMART_PUNCTUATION.bits()
        | Options::ENABLE_TASKLISTS.bits()
        | Options::ENABLE_STRIKETHROUGH.bits(),
);

fn marked(e: Event) -> Event {
    match e {
        // Support line number for code block
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))) => {
            let info = info.replace(' ', "");
            let mut head = String::new();
            if info.is_empty() {
                head += "<pre><code>"
            } else {
                let lang = info.split('[').next().unwrap();
                let line = info
                    .replace(lang, "")
                    .replace(|s| (s == '[') | (s == ']'), "");
                head += &format!("<pre><code class=\"language-{}\"", lang);
                if !line.is_empty() {
                    head += &format!(" data-line-numbers=\"{}\"", line);
                }
                head += ">";
            }
            Event::Html(head.into())
        }
        _ => e,
    }
}

pub(crate) fn md2html(text: &str) -> String {
    let mut out = String::new();
    push_html(&mut out, Parser::new_ext(text, MARKED).map(marked));
    out
}

struct FragMap(HashMap<String, HashMap<usize, String>>);

impl FragMap {
    fn new(slide: &Node, count: &mut usize) -> Result<Self, Error> {
        let mut frag_map = HashMap::new();
        for h in slide.get_default(&["fragment"], &vec![], Node::as_array)? {
            for (k, v) in h.as_map()?.iter() {
                let k = k.as_str()?;
                let v = v.as_str()?;
                if !frag_map.contains_key(k) {
                    frag_map.insert(k.to_owned(), HashMap::new());
                }
                frag_map.get_mut(k).unwrap().insert(*count, v.to_owned());
            }
            *count += 1;
        }
        Ok(Self(frag_map))
    }

    fn fragment(&mut self, tag: &str, inner: &str) -> String {
        let tag = tag.to_owned();
        let mut head = String::new();
        let mut end = String::new();
        if let Some(frag) = self.0.get(&tag) {
            for (index, frag) in frag {
                head += &format!(
                    "<span class=\"fragment {}\" data-fragment-index=\"{}\">",
                    frag, index
                );
                end += "</span>";
            }
        }
        head + inner + &end
    }
}

pub(crate) fn sized_block(img: &Node) -> Result<String, Error> {
    let src = img.get_default(&["src"], "", Node::as_str)?;
    if src.is_empty() {
        return Err(Error("empty source", img.pos));
    }
    let mut doc = format!(" src=\"{}\"", src);
    for attr in ["width", "height"] {
        let value = img.get_default(&[attr], "", Node::as_value)?;
        if !value.is_empty() {
            doc += &format!(" {}=\"{}\"", attr, value);
        }
    }
    Ok(doc)
}

fn img_block(img: &Node) -> Result<String, Error> {
    let mut doc = format!("<figure><img{}/>", sized_block(img)?);
    let label = img.get_default(&["label"], "", Node::as_str)?;
    if !label.is_empty() {
        doc += &format!("<figcaption>{}</figcaption>", label);
    }
    doc += "</figure>";
    Ok(doc)
}

pub(crate) fn content_block(slide: &Node, frag_count: &mut usize) -> Result<String, Error> {
    let mut doc = String::new();
    let mut frag = FragMap::new(slide, frag_count)?;
    let mut t = slide.get_default(&["doc"], "", Node::as_str)?;
    if !t.is_empty() {
        doc += &frag.fragment("doc", &md2html(t));
    }
    if let Ok(n) = slide.get(&["include"]) {
        t = n.as_str()?;
        if !t.is_empty() {
            let include = read_to_string(t).map_err(|_| ("read file error", n.pos))?;
            doc += &frag.fragment("include", &md2html(&include));
        }
    }
    t = slide.get_default(&["math"], "", Node::as_str)?;
    if !t.is_empty() {
        doc += &frag.fragment("math", &format!("\\[{}\\]", t));
    }
    if let Ok(img) = slide.get(&["img"]) {
        match &img.yaml {
            Yaml::Array(imgs) => {
                if !imgs.is_empty() {
                    doc += "<div style=\"display:flex;flex-direction:row;justify-content:center;align-items:center\">";
                    for img in imgs {
                        doc += &frag.fragment("img", &img_block(img)?);
                    }
                    doc += "</div>";
                }
            }
            Yaml::Map(_) => {
                doc += &frag.fragment("img", &img_block(img)?);
            }
            _ => return Err(Error("invalid image", img.pos)),
        }
    }
    let empty = vec![];
    for (i, &title) in ["hstack", "$hstack", "vstack", "$vstack"]
        .iter()
        .enumerate()
    {
        let stack = slide.get_default(&[title], &empty, Node::as_array)?;
        if stack.is_empty() {
            continue;
        }
        let head = if i < 2 {
            doc += "<div class=\"hstack\">";
            let width = 100. / stack.len() as f32;
            format!(" style=\"width:{}%\"", width)
        } else {
            doc += "<div class=\"vstack\">";
            "".to_owned()
        };
        for (j, slide) in stack.iter().enumerate() {
            let border = if j > 0 && title.starts_with('$') {
                if i < 2 {
                    " class=\"hstack-border\""
                } else {
                    " class=\"vstack-border\""
                }
            } else {
                ""
            };
            doc += &format!("<div{}{}>", border, head);
            doc += &content_block(slide, frag_count)?;
            doc += "</div>";
        }
        doc += "</div>";
    }
    Ok(doc)
}
