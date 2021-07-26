use yaml_peg::Node;

#[derive(Default)]
pub(crate) struct Background {
    src: String,
    size: String,
    position: String,
    repeat: String,
    opacity: String,
}

impl Background {
    pub(crate) fn new(meta: &Node) -> Result<Self, u64> {
        if let Ok(n) = meta.get("background") {
            Ok(Self {
                src: n.get_default("src", "", Node::as_str)?.to_string(),
                size: n.get_default("size", "", Node::as_value)?.to_string(),
                position: n.get_default("position", "", Node::as_value)?.to_string(),
                repeat: n.get_default("repeat", "", Node::as_value)?.to_string(),
                opacity: n.get_default("opacity", "", Node::as_value)?.to_string(),
            })
        } else {
            Ok(Self::default())
        }
    }

    pub(crate) fn is_valid(&self) -> bool {
        !self.src.is_empty()
    }

    pub(crate) fn attr(&self) -> String {
        let mut doc = String::new();
        for (attr, member) in [
            ("", &self.src),
            ("-size", &self.size),
            ("-position", &self.position),
            ("-repeat", &self.repeat),
            ("-opacity", &self.opacity),
        ] {
            if !member.is_empty() {
                doc += &format!(" data-background{}=\"{}\"", attr, member);
            }
        }
        doc
    }
}
