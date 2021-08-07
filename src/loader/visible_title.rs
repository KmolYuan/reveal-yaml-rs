use super::*;

pub(crate) fn visible_title<'a, 'b: 'a>(n: &'a Node, v: &'b Anchors) -> Option<&'a Node> {
    if let Ok(n) = n.get("title") {
        Some(n.as_anchor(v))
    } else if let Ok(n) = n.get("-title") {
        Some(n.as_anchor(v))
    } else {
        None
    }
}
