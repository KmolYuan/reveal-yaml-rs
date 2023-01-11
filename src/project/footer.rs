use super::*;

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
    pub size: Sized,
}

impl ToHtml for Footer {
    fn to_html(self, _ctx: &Ctx) -> String {
        let Self { label, link, size } = self;
        let (src, size) = size.size();
        if src.is_empty() && label.is_empty() {
            return String::new();
        }
        let link = link.wrap("<a href=\"", "\">\n");
        let link_end = if link.is_empty() { "" } else { "</a>\n" };
        let img = src.wrap("<img", &format!("{size}/>"));
        "<div class=\"footer\">\n".to_string()
            + &link
            + &img
            + &label.wrap("<span>&nbsp;", "</span>")
            + link_end
            + "</div>"
    }
}
