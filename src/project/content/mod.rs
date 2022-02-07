pub(super) use self::marked::md2html;
pub use self::{frag_map::*, lay_img::*, media::*};
use super::*;
use std::fs::read_to_string;
use yaml_peg::{serialize::InlineList, Anchors, Node};

mod frag_map;
mod lay_img;
mod marked;
mod media;

pub(crate) fn sized_block(node: &Node) -> Result<(String, String), Error> {
    let src = node.get_default("src", "", Node::as_str)?;
    if src.is_empty() {
        return Err(Error("empty source", node.pos()));
    }
    let size = sized("width", node)? + &sized("height", node)?;
    Ok((format!(" src=\"{}\"", src), size))
}

fn sized(attr: &'static str, node: &Node) -> Result<String, Error> {
    let value = node.get_default(attr, "", Node::as_value)?;
    if value.is_empty() {
        Ok("".to_owned())
    } else {
        Ok(format!(" {}=\"{}\"", attr, value))
    }
}

pub(crate) fn content_block(
    slide: &Node,
    v: &Anchors,
    frag_count: &mut usize,
) -> Result<String, Error> {
    let mut doc = String::new();
    let frag = FragMapOld::new(slide, v, frag_count)?;
    for n in slide.get_default("fit", vec![], Node::as_seq)? {
        let t = n.as_anchor(v).as_str()?;
        if t == "---" {
            doc += &frag.fragment("fit", "<hr/>");
        } else {
            doc += "\n<h2 class=\"r-fit-text\">";
            doc += &frag.fragment("fit", t);
            doc += "</h2>";
        }
    }
    doc += &frag.fragment("doc", &md2html(slide.with(v, "doc", "", Node::as_str)?));
    if let Ok(n) = slide.get("include") {
        let t = n.as_str()?;
        if !t.is_empty() {
            let include = read_to_string(t).map_err(|_| ("read file error", n.pos()))?;
            doc += &frag.fragment(
                "include",
                &if n.tag() == "html" {
                    include
                } else {
                    md2html(&include)
                },
            );
        }
    }
    doc += &frag.fragment(
        "math",
        &slide.with(v, "math", "", Node::as_str)?.wrap("\\[", "\\]"),
    );
    doc += &media(slide, v, &frag)?;
    if let Ok(n) = slide.get("lay-img") {
        doc += &lay_img(n.as_anchor(v), v)?;
    }
    for (i, &title) in ["hstack", "$hstack", "vstack", "$vstack"]
        .iter()
        .enumerate()
    {
        let stack = slide.get_default(title, vec![], Node::as_seq)?;
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
            doc += &content_block(slide.as_anchor(v), v, frag_count)?;
            doc += "</div>";
        }
        doc += "</div>";
    }
    Ok(doc)
}

/// A content block, which visualize all contents in the layout.
///
/// The attributes will placed in the following order.
#[derive(Default, serde::Deserialize)]
#[serde(default)]
pub struct Content {
    /// [Fit texts](https://revealjs.com/layout/#fit-text).
    ///
    /// + Longer text will be smaller.
    /// + Special symbol `---` represents horizontal line `<hr/>`.
    pub fit: Vec<String>,
    /// Multiline Markdown text, accept HTML.
    pub doc: String,
    /// Include a Markdown file from path, append after `doc`.
    pub include: String,
    /// If you want to include an HTML file without conversion, enable this option.
    #[serde(rename = "include-html")]
    pub include_html: bool,
    /// Latex math without `$$` / `\[\]` brackets.
    pub math: String,
    /// Embed images.
    pub img: InlineList<Img>,
    /// Embed videos.
    pub video: InlineList<Video>,
    /// Embed `<iframe>` structures, such as YouTube videos.
    pub iframe: InlineList<IFrame>,
    /// [Layout stack](https://revealjs.com/layout/#stack) for images.
    #[serde(rename = "lay-img")]
    pub lay_img: InlineList<LayImg>,
    /// [Fragment](https://revealjs.com/fragments/) option.
    pub fragment: FragMap,
    /// Horizontal stack.
    #[serde(rename = "h-stack")]
    pub h_stack: Vec<Self>,
    /// Vertical stack.
    #[serde(rename = "v-stack")]
    pub v_stack: Vec<Self>,
    /// Horizontal stack with border.
    #[serde(rename = "h-stack-border")]
    pub h_stack_border: Vec<Self>,
    /// Vertical stack with border.
    #[serde(rename = "v-stack-border")]
    pub v_stack_border: Vec<Self>,
}

/// Sized item option.
#[derive(Default, serde::Deserialize)]
#[serde(default)]
pub struct Sized {
    /// Source link.
    pub src: String,
    /// Item width.
    pub width: String,
    /// Item height.
    pub height: String,
}
