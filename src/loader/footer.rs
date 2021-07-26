use super::{sized_block, Error};
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
    let (src, size) = sized_block(&footer)?;
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
