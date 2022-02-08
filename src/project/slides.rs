use super::{content_block, md2html, visible_title, Error, ImgBackground, WrapString};
use yaml_peg::{repr::RcRepr, serialize::Optional, Anchors, Node, Seq};

pub(crate) fn slides(
    slides: Seq<RcRepr>,
    v: &Anchors,
    bg: ImgBackground,
    outline: bool,
) -> Result<(String, String), Error> {
    let mut doc = String::new();
    let mut title = String::new();
    for (i, slide) in slides.iter().enumerate() {
        let slide = slide.as_anchor(v);
        doc += "<section>";
        doc += &slide_block(slide, v, &bg)?;
        for slide in slide.get_default("sub", vec![], Node::as_seq)? {
            doc += &slide_block(slide.as_anchor(v), v, &bg)?;
        }
        if i == 0 {
            if let Some(n) = visible_title(slide, v) {
                title += n.as_str()?;
            }
            if !outline || slides.len() < 2 {
                continue;
            }
            doc += "<section";
            if bg.is_valid() {
                doc += &bg.attr();
            }
            doc += "><h2>Outline</h2><hr/>";
            let mut outline = String::new();
            for (i, slide) in slides.iter().enumerate() {
                if i == 0 {
                    continue;
                }
                if let Some(n) = visible_title(slide, v) {
                    outline += &format!("+ [{}](#/{})\n", n.as_str()?, i);
                } else {
                    continue;
                }
                for (j, slide) in slide
                    .get_default("sub", vec![], Node::as_seq)?
                    .iter()
                    .enumerate()
                {
                    if let Some(n) = visible_title(slide, v) {
                        outline += &format!("  + [{}](#/{}/{})\n", n.as_str()?, i, j + 1);
                    }
                }
            }
            doc += &md2html(&outline);
            doc += "</section>";
        }
        doc += "</section>";
    }
    Ok((doc, title))
}

fn slide_block(slide: &Node, v: &Anchors, bg: &ImgBackground) -> Result<String, Error> {
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
        let local_bg = ImgBackground::new(slide)?;
        doc += &if local_bg.is_valid() { &local_bg } else { bg }.attr();
    }
    doc += ">";
    for (i, &title) in ["title", "-title", "$title"].iter().enumerate() {
        if let Ok(n) = slide.get(title) {
            if i != 1 {
                doc += &md2html(&format!("# {}", n.as_anchor(v).as_str()?));
                doc += "<hr/>";
            }
            break;
        }
    }
    doc += &content_block(slide, v, &mut 0)?;
    if let Ok(n) = slide.get("note") {
        doc += &md2html(n.as_anchor(v).as_str()?).wrap("<aside class=\"notes\">", "</aside>");
    }
    doc += "</section>";
    Ok(doc)
}

/// Slides data.
///
/// Slides are a list of multiple slide blocks, they are totally YAML Maps.
///
/// ```yaml
/// - title: Title 1
///   doc: Document 1
/// - title: Title 2
///   doc: Document 2
///   sub:
///     - title: Title 2-1
///       doc: Document 2-1
/// ```
///
/// Only chapter (horizontal) slides has "sub" attribute,
/// which can append section slides vertically.
#[derive(Default, serde::Deserialize)]
#[serde(default)]
pub struct ChapterSlide {
    /// Chapter slides have all attributes of other slides. (*flatten*)
    #[serde(flatten)]
    pub slide: Slide,
    /// Here is the other section slides under this chapter slide.
    pub sub: Vec<Slide>,
}

/// All slides has following attributes.
#[derive(Default, serde::Deserialize)]
#[serde(default)]
pub struct Slide {
    /// Markdown level 1 title without `#` notation.
    pub title: String,
    /// Visible title but will be excluded in TOC.
    #[serde(rename = "title-hidden")]
    pub title_hidden: String,
    /// Invisible title, doesn't show but will be included in TOC.
    #[serde(rename = "title-invisible")]
    pub title_invisible: String,
    /// Slides have all attributes of "content"s. (*flatten*)
    ///
    /// `Content` type can be placed with different layouts.
    #[serde(flatten)]
    pub content: super::Content,
    /// Note in Speaker's view, Markdown syntax.
    pub note: String,
    /// [Background color](https://revealjs.com/backgrounds/#color-backgrounds).
    #[serde(rename = "bg-color")]
    pub bg_color: String,
    /// Background setting, as same as global.
    ///
    /// + Local background option can be boolean `false` to disable global background.
    pub background: Optional<super::Background>,
    /// [Transition](https://revealjs.com/transitions/) option.
    pub trans: String,
    /// [Background transition](https://revealjs.com/transitions/#background-transitions) option.
    #[serde(rename = "bg-trans")]
    pub bg_trans: String,
}
