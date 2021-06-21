use self::content::*;
use heck::MixedCase;
use std::io::{Error as IoError, ErrorKind};
use yaml_peg::{indicated_msg, parse, Array, Node};

mod content;

const TEMPLATE: &str = include_str!("../assets/template.html");

pub(crate) struct Error(&'static str, u64);

impl From<u64> for Error {
    fn from(pos: u64) -> Self {
        Self("invalid value type", pos)
    }
}

impl From<(&'static str, u64)> for Error {
    fn from((v, pos): (&'static str, u64)) -> Self {
        Self(v, pos)
    }
}

struct Background<'a> {
    src: &'a str,
    size: &'a str,
    position: &'a str,
    repeat: &'a str,
    opacity: &'a str,
}

impl<'a> Background<'a> {
    fn new(meta: &'a Node) -> Result<Self, u64> {
        Ok(Self {
            src: meta.get_default(&["background", "src"], "", Node::as_str)?,
            size: meta.get_default(&["background", "size"], "", Node::as_value)?,
            position: meta.get_default(&["background", "position"], "", Node::as_value)?,
            repeat: meta.get_default(&["background", "repeat"], "", Node::as_value)?,
            opacity: meta.get_default(&["background", "opacity"], "", Node::as_value)?,
        })
    }

    fn is_valid(&self) -> bool {
        !self.src.is_empty()
    }

    fn attr(&self) -> String {
        let mut doc = String::new();
        for (attr, member) in &[
            ("", self.src),
            ("-size", self.size),
            ("-position", self.position),
            ("-repeat", self.repeat),
            ("-opacity", self.opacity),
        ] {
            if !member.is_empty() {
                doc += &format!(" data-background{}=\"{}\"", attr, member);
            }
        }
        doc
    }
}

fn slide_block(slide: &Node, bg: &Background, first_column: bool) -> Result<String, Error> {
    if slide.as_map()?.is_empty() {
        return Err(Error("empty slide", slide.pos));
    }
    let mut doc = "<section".to_owned();
    let mut t = slide.get_default(&["bg-color"], "", Node::as_str)?;
    if !t.is_empty() {
        doc += &format!(" data-background-color=\"{}\"", t);
    }
    t = slide.get_default(&["trans"], "", Node::as_str)?;
    if !t.is_empty() {
        doc += &format!(" data-transition=\"{}\"", t);
    }
    t = slide.get_default(&["bg-trans"], "", Node::as_str)?;
    if !t.is_empty() {
        doc += &format!(" data-background-transition=\"{}\"", t);
    }
    if bg.is_valid()
        && slide
            .get_default(&["background"], true, Node::as_bool)
            .unwrap_or(true)
    {
        let local_bg = Background::new(slide)?;
        doc += &if local_bg.is_valid() { &local_bg } else { bg }.attr();
    }
    for (i, title) in ["title", "none-title"].iter().enumerate() {
        t = slide.get_default(&[*title], "", Node::as_str)?;
        if !t.is_empty() {
            if i == 1 || first_column {
                doc += " data-visibility=\"uncounted\"";
            }
            doc += ">";
            doc += &md2html(&format!("# {}", t));
            doc += "<hr/>";
            break;
        }
        if i == 1 {
            doc += ">";
        }
    }
    doc += &content_block(slide, &mut 0)?;
    t = slide.get_default(&["note"], "", Node::as_str)?;
    if !t.is_empty() {
        doc += &format!("<aside class=\"notes\">{}</aside>", md2html(&t));
    }
    doc += "</section>";
    Ok(doc)
}

fn options(meta: &Node) -> Result<String, Error> {
    let meta = match meta.get(&["option"]) {
        Ok(n) => n,
        Err(_) => return Ok(String::new()),
    };
    let mut doc = String::new();
    for (k, v) in meta.as_map()? {
        doc += &k.as_str()?.to_mixed_case();
        doc += ": ";
        if let Ok(s) = v.as_str() {
            doc += &format!("\"{}\"", s);
        } else {
            doc += v.as_value()?;
        }
        doc += ", ";
    }
    Ok(doc)
}

fn footer_block(meta: &Node) -> Result<String, Error> {
    let footer = match meta.get(&["footer"]) {
        Ok(n) => n,
        Err(_) => return Ok(String::new()),
    };
    let src = footer.get_default(&["src"], "", Node::as_str)?;
    let label = footer.get_default(&["label"], "", Node::as_str)?;
    if src.is_empty() && label.is_empty() {
        return Ok(String::new());
    }
    let mut doc =
        "<div id=\"hidden\" style=\"display: none\"><div id=\"footer\"><div id=\"footer-left\">\n"
            .to_owned();
    let link = footer.get_default(&["link"], "", Node::as_str)?;
    if !link.is_empty() {
        doc += &format!("<a href=\"{}\">", link);
    }
    doc += &format!("<img{}/>", sized_block(footer)?);
    let label = footer.get_default(&["label"], "", Node::as_str)?;
    if !label.is_empty() {
        doc += &format!("<span>&nbsp;{}</span>", label);
    }
    if !link.is_empty() {
        doc += "</a>";
    }
    doc += "\n</div></div></div>";
    Ok(doc)
}

fn load_main(yaml: Array, mount: &str) -> Result<String, Error> {
    let mut title = String::new();
    let meta = &yaml[0];
    let slides = yaml[1].as_array()?;
    let bg = Background::new(meta)?;
    let mut doc = String::new();
    for (i, slide) in slides.iter().enumerate() {
        doc += "<section>";
        doc += &slide_block(slide, &bg, i == 0)?;
        for slide in slide.get_default(&["sub"], &vec![], Node::as_array)? {
            doc += &slide_block(slide, &bg, false)?;
        }
        if i == 0 {
            title += slide.get_default(&["title"], "", Node::as_str)?;
            if !meta.get_default(&["outline"], true, Node::as_bool)? {
                continue;
            }
            doc += "<section data-visibility=\"uncounted\"";
            if bg.is_valid() {
                doc += &bg.attr();
            }
            doc += "><h2>Outline</h2><hr/><ul>";
            for (i, slide) in slides.iter().enumerate() {
                if i == 0 {
                    continue;
                }
                let t = slide.get_default(&["title"], "", Node::as_str)?;
                if t.is_empty() {
                    continue;
                }
                doc += &format!("<li><a href=\"#/{}\">{}</a></li>", i, t);
                let empty = vec![];
                let sub = slide.get_default(&["sub"], &empty, Node::as_array)?;
                if sub.is_empty() {
                    continue;
                }
                doc += "<ul>";
                for (j, slide) in sub.iter().enumerate() {
                    let t = slide.get(&["title"])?.as_str()?;
                    if t.is_empty() {
                        continue;
                    }
                    doc += &format!("<li><a href=\"#/{}/{}\">{}</a></li>", i, j + 1, t);
                }
                doc += "</ul>";
            }
            doc += "</ul></section>";
        }
        doc += "</section>";
    }
    let mut reveal = TEMPLATE.to_owned().replace("{%mount}", mount);
    for (key, default) in &[
        ("icon", "help/icon.png"),
        ("lang", "en"),
        ("title", &title),
        ("description", ""),
        ("author", ""),
        ("theme", "serif"),
        ("code-theme", "zenburn"),
    ] {
        reveal = reveal.replace(
            &format!("{{%{}}}", key),
            meta.get_default(&[*key], *default, Node::as_str)?,
        );
    }
    reveal = reveal.replace("/* {%option} */", &options(meta)?);
    reveal = reveal.replace(
        "/* {%style} */",
        meta.get_default(&["style"], "", Node::as_str)?,
    );
    reveal = reveal.replace("{%footer}", &footer_block(meta)?);
    reveal = reveal.replace("{%slides}", &doc);
    Ok(reveal)
}

/// Load YAML string as HTML.
pub fn loader(yaml_str: &str, mount: &str) -> Result<String, IoError> {
    let yaml = parse(yaml_str).map_err(|s| IoError::new(ErrorKind::InvalidData, s))?;
    if yaml.len() < 2 {
        return Err(IoError::new(
            ErrorKind::InvalidData,
            "Missing metadata or slides".to_owned(),
        ));
    }
    load_main(yaml, mount).map_err(|Error(name, pos)| {
        IoError::new(
            ErrorKind::InvalidData,
            format!("{}:\n{}", name, indicated_msg(yaml_str, pos)),
        )
    })
}
