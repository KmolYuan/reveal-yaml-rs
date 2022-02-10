use super::{Ctx, StringWrap};

/// Global footer option.
#[derive(Default, serde::Deserialize)]
#[serde(default)]
pub struct Footer {
    /// Footer text.
    pub label: String,
    /// Footer link, works on image and text.
    pub link: String,
    /// This item is sized. (*flatten*)
    #[serde(flatten)]
    pub size: super::Sized,
}

impl super::ToHtml for Footer {
    fn to_html(self, _ctx: &Ctx) -> String {
        let Self { label, link, size } = self;
        let (src, size) = size.size();
        if src.is_empty() && label.is_empty() {
            return String::new();
        }
        let link = link.wrap("<a href=\"", "\">\n");
        let link_end = if link.is_empty() { "" } else { "</a>\n" };
        let img = src.wrap("<img", &format!("{}/>", size));
        "<div id=\"hidden\" style=\"display: none\">".to_string()
            + "<div id=\"footer\">"
            + "<div id=\"footer-left\">\n"
            + &link
            + &img
            + link_end
            + "</div></div></div>"
    }
}
