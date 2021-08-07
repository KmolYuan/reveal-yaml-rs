use super::*;
use yaml_peg::{Anchors, Node, Yaml};

pub(crate) fn lay_img(m: &Node, v: &Anchors) -> Result<String, Error> {
    match m.yaml() {
        Yaml::Array(ms) => {
            let mut doc = "<div class=\"r-stack\">".to_string();
            for m in ms {
                doc += &img_block(m.as_anchor(v))?;
            }
            doc += "</div>";
            Ok(doc)
        }
        Yaml::Map(_) => Ok(format!("<div class=\"r-stack\">{}</div>", img_block(m)?)),
        _ => Err(Error("invalid lay blocks", m.pos())),
    }
}

fn img_block(m: &Node) -> Result<String, Error> {
    let (src, size) = sized_block(m)?;
    let frag = m.get_default("fragment", "", Node::as_str)?;
    Ok(format!("<img class=\"fragment {}\"{}{}/>", frag, src, size))
}
