use self::frag_map::FragMap;
pub(super) use self::marked::md2html;
use super::*;
use std::fs::read_to_string;
use yaml_peg::{Anchors, Node, Yaml};

mod frag_map;
mod marked;

pub(crate) fn sized_block(img: &Node) -> Result<(String, String), Error> {
    let src = img.get_default("src", "", Node::as_str)?;
    if src.is_empty() {
        return Err(Error("empty source", img.pos()));
    }
    let mut doc = String::new();
    for attr in ["width", "height"] {
        let value = img.get_default(attr, "", Node::as_value)?;
        if !value.is_empty() {
            doc += &format!(" {}=\"{}\"", attr, value);
        }
    }
    Ok((format!(" src=\"{}\"", src), doc))
}

fn img_block(img: &Node) -> Result<String, Error> {
    let (src, size) = sized_block(img)?;
    let mut doc = format!("<figure><img{}{}/>", src, size);
    let label = img.get_default("label", "", Node::as_str)?;
    if !label.is_empty() {
        doc += &format!("<figcaption>{}</figcaption>", label);
    }
    doc += "</figure>";
    Ok(doc)
}

fn video_block(video: &Node) -> Result<String, Error> {
    let (src, size) = sized_block(video)?;
    let mut doc = format!("<video{}", size);
    if video.get_default("controls", true, Node::as_bool)? {
        doc += " controls";
    }
    if video.get_default("autoplay", false, Node::as_bool)? {
        doc += " autoplay";
    }
    let ty = video.get_default("type", "video/mp4", Node::as_str)?;
    doc += &format!("><source{} type=\"{}\"></video>", src, ty);
    Ok(doc)
}

pub(crate) fn content_block(
    slide: &Node,
    v: &Anchors,
    frag_count: &mut usize,
) -> Result<String, Error> {
    let mut doc = String::new();
    let mut frag = FragMap::new(slide, frag_count)?;
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
    let media: [(_, fn(&Node) -> Result<String, Error>); 2] =
        [("img", img_block), ("video", video_block)];
    for (tag, f) in media {
        if let Ok(m) = slide.as_anchor(v).get(tag) {
            match m.yaml() {
                Yaml::Array(ms) => {
                    if !ms.is_empty() {
                        doc += "<div style=\"display:flex;flex-direction:row;justify-content:center;align-items:center\">";
                        for m in ms {
                            doc += &frag.fragment(tag, &f(m)?);
                        }
                        doc += "</div>";
                    }
                }
                Yaml::Map(_) => {
                    doc += &frag.fragment(tag, &f(&m)?);
                }
                _ => return Err(Error("invalid blocks", m.pos())),
            }
        }
    }
    for (i, &title) in ["hstack", "$hstack", "vstack", "$vstack"]
        .iter()
        .enumerate()
    {
        let stack = slide.get_default(title, vec![], Node::as_array)?;
        if stack.is_empty() {
            continue;
        }
        let head = if i < 2 {
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
