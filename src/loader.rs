use crate::content::*;
use std::io::Result;
use yaml_rust::{yaml::Hash, Yaml, YamlLoader};

const TEMPLATE: &str = include_str!("assets/template.html");

struct Background {
    src: String,
    size: String,
    position: String,
    repeat: String,
    opacity: String,
}

impl Background {
    fn new(meta: &Hash) -> Result<Self> {
        let h = Hash::new();
        let bg = match meta.get(yaml_str!["background"]).unwrap_or(yaml_bad![]) {
            Yaml::Hash(h) => h,
            Yaml::BadValue => &h,
            _ => return err!("background must be hash"),
        };
        Ok(Self {
            src: bg.get_string("src", "", (0, 0))?,
            size: bg.get_value("size", "", (0, 0))?,
            position: bg.get_string("position", "", (0, 0))?,
            repeat: bg.get_string("repeat", "", (0, 0))?,
            opacity: bg.get_value("opacity", "", (0, 0))?,
        })
    }
    fn is_valid(&self) -> bool {
        !self.src.is_empty()
    }
    fn attr(&self) -> String {
        let mut doc = String::new();
        for (attr, member) in &[
            ("", self.src.clone()),
            ("-size", self.size.clone()),
            ("-position", self.position.clone()),
            ("-repeat", self.repeat.clone()),
            ("-opacity", self.opacity.clone()),
        ] {
            if !member.is_empty() {
                doc += &format!(" data-background{}=\"{}\"", attr, member);
            }
        }
        doc
    }
}

fn slide_block(slide: &Hash, bg: &Background, pos: Pos) -> Result<String> {
    if slide.is_empty() {
        return err!(format!("empty slide block, ({}:{})", pos.0, pos.1));
    }
    let mut doc = String::from("<section");
    let mut t = slide.get_string("bg-color", "", pos)?;
    if !t.is_empty() {
        doc += &format!(" data-background-color=\"{}\"", t);
    }
    t = slide.get_string("trans", "", pos)?;
    if !t.is_empty() {
        doc += &format!(" data-transition=\"{}\"", t);
    }
    t = slide.get_string("bg-trans", "", pos)?;
    if !t.is_empty() {
        doc += &format!(" data-background-transition=\"{}\"", t);
    }
    if bg.is_valid() && slide.is_enabled("background") {
        let local_bg = Background::new(slide)?;
        doc += &if local_bg.is_valid() { &local_bg } else { bg }.attr();
    }
    doc += ">";
    for title in &["title", "no-title"] {
        t = slide.get_string(title, "", pos)?;
        if !t.is_empty() {
            doc += &format!("<h2>{}</h2><hr/>", t);
        }
    }
    doc += &content_block(slide, pos, &mut 0)?;
    t = slide.get_string("note", "", pos)?;
    if !t.is_empty() {
        doc += &format!("<aside class=\"notes\">{}</aside>", parse(&t));
    }
    doc += "</section>";
    Ok(doc)
}

fn footer_block(meta: &Hash) -> Result<String> {
    let h = Hash::new();
    let footer = match meta.get(yaml_str!["footer"]).unwrap_or(yaml_bad![]) {
        Yaml::Hash(h) => h,
        Yaml::BadValue => &h,
        _ => return err!("invalid footer"),
    };
    let src = footer.get_string("src", "", (0, 0))?;
    let label = footer.get_string("label", "", (0, 0))?;
    if src.is_empty() && label.is_empty() {
        return Ok("".into());
    }
    let mut doc = String::from(
        "<div id=\"hidden\" style=\"display: none\"><div id=\"footer\"><div id=\"footer-left\">\n",
    );
    let link = footer.get_string("link", "", (0, 0))?;
    if !link.is_empty() {
        doc += &format!("<a href=\"{}\">", link);
    }
    doc += &format!("<img{}/>", sized_block(footer, (0, 0))?);
    let label = footer.get_string("label", "", (0, 0))?;
    if !label.is_empty() {
        doc += &format!("<span>&nbsp;{}</span>", label);
    }
    if !link.is_empty() {
        doc += "</a>";
    }
    doc += "\n</div></div></div>";
    Ok(doc)
}

/// Load YAML string as HTML.
pub fn loader(yaml_str: &str, mount: &str) -> Result<String> {
    let yaml = match YamlLoader::load_from_str(yaml_str) {
        Ok(v) => v,
        Err(e) => return err!(e.to_string()),
    };
    if yaml.len() < 2 {
        return err!("Missing metadata or slides");
    }
    let mut title = String::new();
    let meta = yaml[0].assert_hash("meta must be key values")?;
    let slides = yaml[1].assert_vec("slides must be array")?;
    let bg = Background::new(meta)?;
    let mut doc = String::new();
    for (i, s) in slides.iter().enumerate() {
        doc += "<section>";
        let slide = s.assert_hash(&format!("unpack slide failed: ({}:0)", i))?;
        doc += &slide_block(slide, &bg, (i, 0))?;
        let (sub, _) = slide.get_vec("sub", (i, 0))?;
        for (j, s) in sub.enumerate() {
            let slide = s.assert_hash(&format!("unpack slide failed: ({}:0)", i))?;
            doc += &slide_block(slide, &bg, (i, j))?;
        }
        if i == 0 {
            title = slide.get_string("title", "", (i, 0))?;
            if !meta.get_bool("outline", true, (i, 0))? {
                continue;
            }
            doc += "<section";
            if bg.is_valid() {
                doc += &bg.attr();
            }
            doc += "><h2>Outline</h2><hr/><ul>";
            for (i, s) in slides.iter().enumerate() {
                let s = s.assert_hash("unpack slide failed")?;
                let t = s.get_string("title", "", (i, 0))?;
                if t.is_empty() {
                    continue;
                }
                doc += &format!("<li><a href=\"#/{}\">{}</a></li>", i, t);
                let (sub, sub_len) = s.get_vec("sub", (i, 0))?;
                if sub_len == 0 {
                    continue;
                }
                doc += "<ul>";
                for (j, s) in sub.enumerate() {
                    let s = s.assert_hash("unpack slide failed")?;
                    let t = s.get_string("title", "", (i, j))?;
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
    let mut reveal = String::from(TEMPLATE).replace("{%mount}", mount);
    for (key, default) in &[
        ("icon", "img/icon.png"),
        ("lang", "en"),
        ("title", &title),
        ("description", ""),
        ("author", ""),
        ("theme", "serif"),
        ("code-theme", "zenburn"),
    ] {
        reveal = reveal.replace(
            &format!("{{%{}}}", key),
            &meta.get_string(key, default, (0, 0))?,
        );
    }
    reveal = reveal.replace("/* {%option} */", &meta.get_custom_pairs("option", (0, 0))?);
    reveal = reveal.replace("/* {%style} */", &meta.get_string("style", "", (0, 0))?);
    reveal = reveal.replace("{%footer}", &footer_block(meta)?);
    reveal = reveal.replace("{%slides}", &doc);
    Ok(reveal)
}
