use super::*;
use yaml_peg::{Anchors, Node, Yaml};

type F = fn(&Node) -> Result<String, Error>;

pub(crate) fn media(n: &Node, v: &Anchors, frag: &mut FragMap) -> Result<String, Error> {
    let mut doc = String::new();
    for (tag, f) in [
        ("img", img_block as F),
        ("video", video_block as F),
        ("iframe", iframe_block as F),
    ] {
        if let Ok(m) = n.get(tag) {
            match m.yaml() {
                Yaml::Array(ms) => {
                    if !ms.is_empty() {
                        doc += "<div class=\"hstack\">\n";
                        for m in ms {
                            doc += &frag.fragment(tag, &f(m.as_anchor(v))?);
                            doc += "\n";
                        }
                        doc += "</div>";
                    }
                }
                Yaml::Map(_) => {
                    doc += &frag.fragment(tag, &f(&m)?);
                }
                _ => return Err(Error("invalid blocks", m.pos())),
            }
        }
    }
    Ok(doc)
}

fn img_block(img: &Node) -> Result<String, Error> {
    let (src, size) = sized_block(img)?;
    let src = format!("<img{}{}/>", src, size);
    let label = img.get_default("label", "", Node::as_str)?;
    Ok(if label.is_empty() {
        src
    } else {
        format!("<figure>{}<figcaption>{}</figcaption></figure>", src, label)
    })
}

fn video_block(video: &Node) -> Result<String, Error> {
    let (src, size) = sized_block(video)?;
    let mut doc = format!("<video{}", size);
    if video.get_default("controls", true, Node::as_bool)? {
        doc += " controls";
    }
    if video.get_default("autoplay", false, Node::as_bool)? {
        doc += " autoplay";
    }
    let ty = video.get_default("type", "video/mp4", Node::as_str)?;
    doc += &format!("><source{} type=\"{}\"></video>", src, ty);
    Ok(doc)
}

fn iframe_block(iframe: &Node) -> Result<String, Error> {
    let (src, size) = sized_block(iframe)?;
    Ok(format!("<iframe{}{}></iframe>", src, size))
}
