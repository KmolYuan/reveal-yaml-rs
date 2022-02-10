use super::{md2html, Ctx, StringWrap};
use yaml_peg::serialize::Optional;

fn slide_title(slide: &Slide) -> &str {
    if !slide.title.is_empty() {
        &slide.title
    } else {
        &slide.title_hidden
    }
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
pub struct Slides {
    /// The slides.
    pub slides: Vec<ChapterSlide>,
}

impl super::ToHtml for Slides {
    fn to_html(self, ctx: &Ctx) -> String {
        let Self { mut slides } = self;
        if !ctx.outline.is_empty() {
            let doc = slides
                .iter()
                .enumerate()
                .map(|(i, chapter)| {
                    let title = slide_title(&chapter.slide);
                    if title.is_empty() {
                        String::new()
                    } else {
                        format!("+ [{}](#/{})\n", title, i)
                            + &chapter
                                .sub
                                .iter()
                                .enumerate()
                                .map(|(j, slide)| {
                                    let title = slide_title(slide);
                                    if !title.is_empty() {
                                        format!("  + [{}](#/{}/{})\n", title, i, j + 1)
                                    } else {
                                        String::new()
                                    }
                                })
                                .collect::<String>()
                    }
                })
                .collect::<String>();
            if let Some(cover) = slides.first_mut() {
                cover.sub.push(Slide {
                    title: ctx.outline.clone(),
                    content: super::Content {
                        doc,
                        ..Default::default()
                    },
                    background: Optional::Bool(true),
                    ..Default::default()
                });
            }
        }
        slides.to_html(ctx)
    }
}

/// The chapter slide.
///
/// Please see [`Slides`] for more information.
#[derive(Default, serde::Deserialize)]
#[serde(default)]
pub struct ChapterSlide {
    /// Chapter slides have all attributes of other slides. (*flatten*)
    #[serde(flatten)]
    pub slide: Slide,
    /// Here is the other section slides under this chapter slide.
    pub sub: Vec<Slide>,
}

impl super::ToHtml for ChapterSlide {
    fn to_html(self, ctx: &Ctx) -> String {
        let Self { slide, sub } = self;
        let slide = slide.to_html(ctx) + &sub.to_html(ctx);
        format!("<section>\n{}</section>", slide)
    }
}

/// All slides has following attributes.
///
/// Please see [`Slides`] for more information.
#[derive(Default, serde::Deserialize)]
#[serde(default)]
pub struct Slide {
    /// Markdown level 1 title without `#` notation.
    pub title: String,
    /// Visible title but will be excluded in TOC.
    #[serde(rename = "title-hidden")]
    pub title_hidden: String,
    /// Invisible title, doesn't show but will be included in TOC.
    #[serde(rename = "title-only")]
    pub title_only: String,
    /// Slides have all attributes of "content"s. (*flatten*)
    ///
    /// `Content` type can be placed with different layouts.
    #[serde(flatten)]
    pub content: super::Content,
    /// Note in Speaker's view, Markdown syntax.
    pub note: String,
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

impl super::ToHtml for Slide {
    fn to_html(self, ctx: &Ctx) -> String {
        let Self {
            title,
            title_hidden: _,
            title_only,
            content,
            note,
            background,
            trans,
            bg_trans,
        } = self;
        let background = match background {
            Optional::Bool(false) => String::new(),
            Optional::Bool(true) => ctx.background.clone(),
            Optional::Some(bg) => bg.to_html(ctx),
        };
        let data = background
            + &trans.wrap(" data-transition=\"", "\"")
            + &bg_trans.wrap(" data-background-transition=\"", "\"");
        let content = md2html(&title.wrap("# ", ""))
            + &md2html(&title_only.wrap("# ", ""))
            + &content.to_html(ctx)
            + &md2html(&note).wrap("<aside class=\"notes\">", "</aside>\n");
        ctx.frag.set(0);
        format!("<section{}>\n{}</section>", data, content)
    }
}
