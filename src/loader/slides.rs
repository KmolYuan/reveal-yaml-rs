use super::{content_block, md2html, visible_title, Background, Error, WrapString};
use yaml_peg::{repr::RcRepr, Anchors, Array, Node};

pub(crate) fn slides(
    slides: Array<RcRepr>,
    v: &Anchors,
    bg: Background,
    outline: bool,
) -> Result<(String, String), Error> {
    let mut doc = String::new();
    let mut title = String::new();
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
            if !outline {
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
    Ok((doc, title))
}

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
