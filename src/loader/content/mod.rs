use self::frag_map::FragMap;
use self::lay_img::lay_img;
pub(super) use self::marked::md2html;
use self::media::media;
use super::*;
use std::fs::read_to_string;
use yaml_peg::{Anchors, Node};

mod frag_map;
mod lay_img;
mod marked;
mod media;

pub(crate) fn sized_block(img: &Node) -> Result<(String, String), Error> {
    let src = img.get_default("src", "", Node::as_str)?;
    if src.is_empty() {
        return Err(Error("empty source", img.pos()));
    }
    let mut size = String::new();
    for attr in ["width", "height"] {
        let value = img.get_default(attr, "", Node::as_value)?;
        if !value.is_empty() {
            size += &format!(" {}=\"{}\"", attr, value);
        }
    }
    Ok((format!(" src=\"{}\"", src), size))
}

pub(crate) fn content_block(
    slide: &Node,
    v: &Anchors,
    frag_count: &mut usize,
) -> Result<String, Error> {
    let mut doc = String::new();
    let mut frag = FragMap::new(slide, frag_count)?;
    for n in slide.get_default("fit", vec![], Node::as_array)? {
        let n = n.as_anchor(v);
        let t = n.as_str()?;
        if t == "---" {
            doc += &frag.fragment("fit", "<hr/>");
        } else {
            doc += "\n<h2 class=\"r-fit-text\">";
            doc += &frag.fragment("fit", t);
            doc += "</h2>";
        }
    }
    if let Ok(n) = slide.get("doc") {
        doc += &frag.fragment("doc", &md2html(n.as_anchor(v).as_str()?));
    }
    if let Ok(n) = slide.get("include") {
        let t = n.as_str()?;
        if !t.is_empty() {
            let include = read_to_string(t).map_err(|_| ("read file error", n.pos()))?;
            doc += &frag.fragment("include", &md2html(&include));
        }
    }
    if let Ok(n) = slide.get("math") {
        doc += &frag.fragment("math", &n.as_anchor(v).as_str()?.wrap("\\[", "\\]"));
    }
    doc += &media(slide, v, &mut frag)?;
    if let Ok(n) = slide.get("lay-img") {
        doc += &lay_img(&n.as_anchor(v), v)?;
    }
    for (i, &title) in ["hstack", "$hstack", "vstack", "$vstack"]
        .iter()
        .enumerate()
    {
        let stack = slide.get_default(title, vec![], Node::as_array)?;
        if stack.is_empty() {
            continue;
        }
        let head = if title.ends_with("hstack") {
            doc += "<div class=\"hstack\">";
            let width = 100. / stack.len() as f32;
            format!(" style=\"width:{}%\"", width)
        } else {
            doc += "<div class=\"vstack\">";
            String::new()
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
            doc += &content_block(&slide.as_anchor(v), v, frag_count)?;
            doc += "</div>";
        }
        doc += "</div>";
    }
    Ok(doc)
}
