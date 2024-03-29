pub use self::{frag_map::*, lay_img::*, marked::*, media::*, sized::*};
use super::*;
use yaml_peg::serde::InlineList;

mod frag_map;
mod lay_img;
mod marked;
mod media;
mod sized;

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
    ///
    /// The star * symbol of inline math needs to escaped.
    /// ```markdown
    /// $x_a^\*$ and $x_b^\*$
    /// ```
    pub doc: String,
    /// Include a Markdown file from path, append after `doc`.
    pub include: String,
    /// If you want to include an HTML file without conversion, enable this
    /// option.
    #[serde(rename = "include-html")]
    pub include_html: bool,
    /// Multiline LaTex math without `$$` / `\[\]` brackets.
    /// ```yaml
    /// math: |
    ///   \begin{cases}
    ///   x_1 &= 10
    ///   \\
    ///   x_2 &= 20
    ///   \end{cases}
    /// ```
    pub math: String,
    /// Embed images.
    ///
    /// ```yaml
    /// # Single
    /// img:
    ///   src: img/image.png
    /// # Multiple
    /// img:
    /// - src: img/image1.png
    /// - src: img/image2.png
    /// ```
    pub img: InlineList<Img>,
    /// Embed videos. Same as `img`.
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
            s += &media.wrap("<div class=\"h-stack\">\n", "</div>\n");
        }
        s += &lay_img
            .to_html(ctx)
            .wrap("<div class=\"r-stack\">", "</div>\n");
        if !h_stack.is_empty() {
            let width = 100. / h_stack.len() as f32;
            let pre = format!("<div style=\"width:{width:.04}%\">");
            s += &h_stack
                .into_iter()
                .map(|c| c.to_html(ctx).wrap(&pre, "</div>\n"))
                .collect::<String>()
                .wrap("<div class=\"h-stack\">", "</div>\n");
        }
        s += &v_stack
            .into_iter()
            .map(|c| c.to_html(ctx).wrap("<div>", "</div>\n"))
            .collect::<String>()
            .wrap("<div class=\"v-stack\">", "</div>\n");
        if !h_stack_border.is_empty() {
            let width = 100. / h_stack_border.len() as f32;
            let pre = format!("<div class=\"h-stack-border\" style=\"width:{width:.04}%\">");
            s += &h_stack_border
                .into_iter()
                .enumerate()
                .map(|(i, c)| {
                    let text = c.to_html(ctx);
                    if i == 0 {
                        text.wrap(&format!("<div style=\"width:{width:.04}%\">"), "</div>")
                    } else {
                        text.wrap(&pre, "</div>")
                    }
                })
                .collect::<String>()
                .wrap("<div class=\"h-stack\">", "</div>");
        }
        s += &v_stack_border
            .into_iter()
            .enumerate()
            .map(|(i, c)| {
                let text = c.to_html(ctx);
                if i == 0 {
                    text.wrap("<div class=\"v-stack\">", "</div>\n")
                } else {
                    text.wrap("<div class=\"v-stack-border\">", "</div>\n")
                }
            })
            .collect::<String>()
            .wrap("<div class=\"v-stack\">", "</div>\n");
        s
    }
}
