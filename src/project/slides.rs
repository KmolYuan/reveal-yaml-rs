use super::*;
use yaml_peg::serde::Optional;

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

impl ToHtml for Slides {
    fn to_html(self, ctx: &Ctx) -> String {
        let Self { mut slides } = self;
        if !ctx.outline.is_empty() {
            let doc = slides
                .iter()
                .enumerate()
                .skip(1)
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
                .collect();
            if let Some(cover) = slides.first_mut() {
                cover.sub.push(Slide {
                    title: ctx.outline.clone(),
                    content: Content {
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

impl ToHtml for ChapterSlide {
    fn to_html(self, ctx: &Ctx) -> String {
        let Self { slide, sub } = self;
        let title = slide_title(&slide).to_string();
        if let Some(header) = &ctx.chapter_header {
            header.replace(String::new());
        }
        let slide = slide.to_html(ctx);
        if let Some(header) = &ctx.chapter_header {
            header.replace(title);
        }
        format!("<section>\n{}</section>", slide + &sub.to_html(ctx))
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
    pub content: Content,
    /// Note in Speaker's view, Markdown syntax.
    pub note: String,
    /// Background setting, as same as global.
    ///
    /// + Local background option can be boolean `false` to disable global background.
    pub background: Optional<Background>,
    /// HTML "class" attribute for this section.
    ///
    /// For example, "my-class" will become `<section class="my-class">`.
    pub class: String,
    /// HTML "id" attribute for this section.
    ///
    /// For example, "my-id" will become `<section id="my-id">`.
    pub id: String,
    /// HTML tag attributes, separated by space.
    ///
    /// For example, "attr1 attr2 ..." will become `<section attr1 attr2 ...>`.
    pub attr: String,
    /// [Auto-Animate](https://revealjs.com/auto-animate/) function.
    #[serde(rename = "auto-animate")]
    pub auto_animate: bool,
    /// [Transition](https://revealjs.com/transitions/) option.
    pub trans: String,
    /// [Background transition](https://revealjs.com/transitions/#background-transitions) option.
    #[serde(rename = "bg-trans")]
    pub bg_trans: String,
}

impl ToHtml for Slide {
    fn to_html(self, ctx: &Ctx) -> String {
        let Self {
            title,
            title_hidden: _,
            title_only,
            content,
            note,
            background,
            class,
            id,
            attr,
            auto_animate,
            trans,
            bg_trans,
        } = self;
        let background = match background {
            Optional::Bool(false) => String::new(),
            Optional::Bool(true) => ctx.background.clone(),
            Optional::Some(bg) => bg.to_html(ctx),
        };
        let auto_animate = if ctx.auto_animate || auto_animate {
            " data-auto-animate"
        } else {
            ""
        };
        let data = background
            + &class.wrap(" class=\"", "\"")
            + &id.wrap(" id=\"", "\"")
            + &trans.wrap(" data-transition=\"", "\"")
            + &bg_trans.wrap(" data-background-transition=\"", "\"")
            + auto_animate
            + &attr.wrap(" ", "");
        let content = md2html(&title.wrap("# ", ""))
            + &md2html(&title_only.wrap("# ", ""))
            + &content.to_html(ctx)
            + &md2html(&note).wrap("<aside class=\"notes\">", "</aside>\n");
        let header = if let Some(header) = &ctx.chapter_header {
            header
                .borrow()
                .wrap("<div class=\"chapter-header\">", "</div>")
        } else {
            String::new()
        };
        ctx.frag.set(0);
        format!("<section{}>\n{}{}</section>", data, content, header)
    }
}
