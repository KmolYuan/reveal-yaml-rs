pub use self::{frag_map::*, lay_img::*, marked::*, media::*};
use super::*;
use yaml_peg::serialize::{InlineList, Stringify};

mod frag_map;
mod lay_img;
mod marked;
mod media;

/// Sized item option.
#[derive(Default, serde::Deserialize)]
#[serde(default)]
pub struct Sized {
    /// Source link.
    pub src: String,
    /// Item width.
    pub width: Stringify,
    /// Item height.
    pub height: Stringify,
}

impl Sized {
    /// Return size information.
    pub fn size(&self) -> (String, String) {
        let Self { src, width, height } = self;
        let src = src.wrap(" src=\"", "\"");
        let size = width.to_string().wrap(" width=\"", "\"")
            + &height.to_string().wrap(" height=\"", "\"");
        (src, size)
    }
}

impl std::fmt::Display for Sized {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (src, size) = self.size();
        write!(f, "{}{}", src, size)
    }
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
    /// Layout stack for images.
    #[serde(rename = "lay-img")]
    pub lay_img: InlineList<LayImg>,
    /// Fragment option.
    #[serde(flatten)]
    pub frag: FragMap,
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

impl ToHtml for Content {
    fn to_html(self, ctx: &Ctx) -> String {
        let Self {
            fit,
            doc,
            include,
            include_html,
            math,
            img,
            video,
            iframe,
            lay_img,
            mut frag,
            h_stack,
            v_stack,
            h_stack_border,
            v_stack_border,
        } = self;
        frag.with_counter(ctx.frag.clone());
        let mut s = String::new();
        for t in fit {
            if t == "---" {
                s += "<hr/>";
            } else {
                s += &frag
                    .wrap("fit", &t)
                    .wrap("<h2 class=\"r-fit-text\">", "</h2>\n");
            }
        }
        s += &frag.wrap("doc", &md2html(&doc));
        if !include.is_empty() {
            let doc = std::fs::read_to_string(include).unwrap();
            let doc = if include_html { doc } else { md2html(&doc) };
            s += &frag.wrap("include", &doc);
        }
        s += &frag.wrap("math", &math.wrap("\\[", "\\]"));
        for media in [img.to_html(ctx), video.to_html(ctx), iframe.to_html(ctx)] {
            s += &media.wrap("<div class=\"hstack\">\n", "</div>\n");
        }
        s += &lay_img
            .to_html(ctx)
            .wrap("<div class=\"r-stack\">", "</div>\n");
        if !h_stack.is_empty() {
            let width = 100. / h_stack.len() as f32;
            let pre = format!("<div style=\"width:{:.04}%\">", width);
            s += &h_stack
                .into_iter()
                .map(|c| c.to_html(ctx).wrap(&pre, "</div>\n"))
                .collect::<String>()
                .wrap("<div class=\"hstack\">", "</div>\n");
        }
        s += &v_stack
            .into_iter()
            .map(|c| c.to_html(ctx).wrap("<div>", "</div>\n"))
            .collect::<String>()
            .wrap("<div class=\"vstack\">", "</div>\n");
        if !h_stack_border.is_empty() {
            let width = 100. / h_stack_border.len() as f32;
            let pre = format!(
                "<div class=\"hstack-border\" style=\"width:{:.04}%\">",
                width
            );
            s += &h_stack_border
                .into_iter()
                .enumerate()
                .map(|(i, c)| {
                    let text = c.to_html(ctx);
                    if i == 0 {
                        text
                    } else {
                        text.wrap(&pre, "</div>\n")
                    }
                })
                .collect::<String>()
                .wrap("<div class=\"hstack\">", "</div>\n");
        }
        s += &v_stack_border
            .into_iter()
            .enumerate()
            .map(|(i, c)| {
                let text = c.to_html(ctx);
                if i == 0 {
                    text
                } else {
                    text.wrap("<div class=\"vstack-border\">", "</div>\n")
                }
            })
            .collect::<String>()
            .wrap("<div class=\"vstack\">", "</div>\n");
        s
    }
}
