use super::*;

pub(crate) fn visible_title(n: &Node, v: &Anchors) -> Option<Node> {
    if let Ok(n) = n.get("title") {
        Some(n.as_anchor(v))
    } else if let Ok(n) = n.get("-title") {
        Some(n.as_anchor(v))
    } else {
        None
    }
}
