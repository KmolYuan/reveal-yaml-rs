use super::{sized_block, Ctx, Error, WrapString};
use yaml_peg::Node;

pub(crate) fn footer(meta: &Node) -> Result<String, Error> {
    let footer = match meta.get("footer") {
        Ok(n) => n,
        Err(_) => return Ok(String::new()),
    };
    let src = footer.get_default("src", "", Node::as_str)?;
    let label = footer.get_default("label", "", Node::as_str)?;
    if src.is_empty() && label.is_empty() {
        return Ok(String::new());
    }
    let mut doc =
        "<div id=\"hidden\" style=\"display: none\"><div id=\"footer\"><div id=\"footer-left\">\n"
            .to_string();
    let link = footer.get_default("link", "", Node::as_str)?;
    if !link.is_empty() {
        doc += &format!("<a href=\"{}\">", link);
    }
    let (src, size) = sized_block(footer)?;
    doc += &format!("<img{}{}/>", src, size);
    let label = footer.get_default("label", "", Node::as_str)?;
    if !label.is_empty() {
        doc += &format!("<span>&nbsp;{}</span>", label);
    }
    if !link.is_empty() {
        doc += "</a>";
    }
    doc += "\n</div></div></div>";
    Ok(doc)
}

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
