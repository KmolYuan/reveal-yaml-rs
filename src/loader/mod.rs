use self::background::Background;
use self::content::*;
use self::error::Error;
use self::footer::footer;
use self::js_option::js_option;
use self::visible_title::visible_title;
use self::wrap_string::WrapString;
use std::io::{Error as IoError, ErrorKind};
use yaml_peg::{indicated_msg, parse, repr::RcRepr, Anchors, Array, Node};

mod background;
mod content;
mod error;
mod footer;
mod js_option;
mod visible_title;
mod wrap_string;

const TEMPLATE: &str = include_str!("../assets/template.html");

fn slide_block(
    slide: &Node,
    v: &Anchors,
    bg: &Background,
    first_column: bool,
) -> Result<String, Error> {
    if slide.as_map()?.is_empty() {
        return Err(Error("empty slide", slide.pos()));
    }
    let mut doc = "<section".to_string();
    if let Ok(n) = slide.get("bg-color") {
        doc += &format!(" data-background-color=\"{}\"", n.as_str()?);
    }
    if let Ok(n) = slide.get("trans") {
        doc += &format!(" data-transition=\"{}\"", n.as_str()?);
    }
    if let Ok(n) = slide.get("bg-trans") {
        doc += &format!(" data-background-transition=\"{}\"", n.as_str()?);
    }
    if bg.is_valid()
        && slide
            .get_default("background", true, Node::as_bool)
            .unwrap_or(true)
    {
        let local_bg = Background::new(slide)?;
        doc += &if local_bg.is_valid() { &local_bg } else { bg }.attr();
    }
    for (i, &title) in ["title", "-title", "$title"].iter().enumerate() {
        if let Ok(n) = slide.get(title) {
            if first_column || i == 2 {
                doc += " data-visibility=\"uncounted\"";
            }
            doc += ">";
            if i != 1 {
                doc += &md2html(&format!("# {}", n.as_anchor(v).as_str()?));
                doc += "<hr/>";
            }
            break;
        } else if i == 2 {
            doc += ">";
        }
    }
    doc += &content_block(slide, v, &mut 0)?;
    if let Ok(n) = slide.get("note") {
        doc += &md2html(n.as_anchor(v).as_str()?).wrap("<aside class=\"notes\">", "</aside>");
    }
    doc += "</section>";
    Ok(doc)
}

fn load_main(yaml: Array<RcRepr>, v: &Anchors, mount: &str) -> Result<String, Error> {
    let mut title = String::new();
    let meta = &yaml[0];
    let slides = yaml[1].as_array()?;
    let bg = Background::new(meta)?;
    let mut doc = String::new();
    for (i, slide) in slides.iter().enumerate() {
        let slide = slide.as_anchor(v);
        doc += "<section>";
        doc += &slide_block(&slide, v, &bg, i == 0)?;
        for slide in slide.get_default("sub", vec![], Node::as_array)? {
            doc += &slide_block(&slide.as_anchor(v), v, &bg, false)?;
        }
        if i == 0 {
            if let Some(n) = visible_title(&slide, v) {
                title += n.as_str()?;
            }
            if !meta.get_default("outline", true, Node::as_bool)? {
                continue;
            }
            doc += "<section data-visibility=\"uncounted\"";
            if bg.is_valid() {
                doc += &bg.attr();
            }
            doc += "><h2>Outline</h2><hr/><ul>";
            for (i, slide) in slides.iter().enumerate() {
                if i == 0 {
                    continue;
                }
                if let Some(n) = visible_title(slide, v) {
                    doc += &format!("<li><a href=\"#/{}\">", i);
                    doc += n.as_str()?;
                    doc += "</a></li>";
                } else {
                    continue;
                }
                let sub = slide.get_default("sub", vec![], Node::as_array)?;
                if sub.is_empty() {
                    continue;
                }
                doc += "<ul>";
                for (j, slide) in sub.iter().enumerate() {
                    if let Some(n) = visible_title(slide, v) {
                        doc += &format!("<li><a href=\"#/{}/{}\">", i, j + 1);
                        doc += n.as_str()?;
                        doc += "</a></li>";
                    }
                }
                doc += "</ul>";
            }
            doc += "</ul></section>";
        }
        doc += "</section>";
    }
    let mut reveal = TEMPLATE.to_string().replace("{%mount}", mount);
    for (key, default) in [
        ("icon", "help/icon.png"),
        ("lang", "en"),
        ("title", &title),
        ("description", ""),
        ("author", ""),
        ("theme", "serif"),
        ("code-theme", "zenburn"),
    ] {
        reveal = reveal.replace(
            &format!("{{%{}}}", key),
            meta.get_default(key, default, Node::as_str)?,
        );
    }
    reveal = reveal.replace("/* {%option} */", &js_option(meta)?);
    reveal = reveal.replace(
        "/* {%style} */",
        meta.get_default("style", "", Node::as_str)?,
    );
    reveal = reveal.replace("{%footer}", &footer(meta)?);
    reveal = reveal.replace("{%slides}", &doc);
    Ok(reveal)
}

/// Load YAML string as HTML.
pub fn loader(yaml_str: &str, mount: &str) -> Result<String, IoError> {
    let (yaml, anchor) = parse(yaml_str).map_err(|s| IoError::new(ErrorKind::InvalidData, s))?;
    if yaml.len() < 2 {
        return Err(IoError::new(
            ErrorKind::InvalidData,
            "Missing metadata or slides".to_string(),
        ));
    }
    load_main(yaml, &anchor, mount).map_err(|Error(name, pos)| {
        IoError::new(
            ErrorKind::InvalidData,
            format!("{}:\n{}", name, indicated_msg(yaml_str, pos)),
        )
    })
}
