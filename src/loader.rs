use heck::MixedCase;
use pulldown_cmark::{html::push_html, CodeBlockKind, Event, Options, Parser, Tag};
use std::{collections::HashMap, fs::read_to_string, io::Result, slice::Iter};
use yaml_rust::{yaml::Hash, Yaml, YamlLoader};

const TEMPLATE: &str = include_str!("assets/template.html");
const MARKED: Options = Options::from_bits_truncate(
    Options::ENABLE_TABLES.bits()
        | Options::ENABLE_SMART_PUNCTUATION.bits()
        | Options::ENABLE_TASKLISTS.bits()
        | Options::ENABLE_STRIKETHROUGH.bits(),
);
type Pos = (usize, usize);

struct FragMap(HashMap<String, HashMap<usize, String>>);

impl FragMap {
    fn new(slide: &Hash, pos: Pos, count: &mut usize) -> Result<Self> {
        let (vec, _) = slide.get_vec("fragment", pos)?;
        let mut frag_map = HashMap::new();
        for h in vec {
            for (k, v) in h
                .assert_hash("invalid fragment setting")?
                .custom_pairs(true, pos)?
            {
                if !frag_map.contains_key(&k) {
                    frag_map.insert(k.clone(), HashMap::new());
                }
                frag_map.get_mut(&k).unwrap().insert(*count, v);
            }
            *count += 1;
        }
        Ok(Self(frag_map))
    }
    fn fragment(&mut self, tag: &str, inner: &str) -> String {
        let tag = String::from(tag);
        let mut head = String::new();
        let mut end = String::new();
        if let Some(frag) = self.0.get(&tag) {
            for (index, frag) in frag {
                head.push_str(&format!(
                    "<span class=\"fragment {}\" data-fragment-index=\"{}\">",
                    frag, index
                ));
                end.push_str("</span>");
            }
        }
        head + inner + &end
    }
}

macro_rules! yaml_bad {
    [] => { &Yaml::BadValue };
}

macro_rules! yaml_bool {
    [$b:expr] => { &Yaml::Boolean($b) };
}

macro_rules! yaml_str {
    [] => { yaml_str![""] };
    [$t:expr] => { &Yaml::String(String::from($t)) };
}

macro_rules! yaml_vec {
    [$($v:tt)?] => { &Yaml::Array(vec![$($v)?]) };
}

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
                doc.push_str(&format!(" data-background{}=\"{}\"", attr, member))
            }
        }
        doc
    }
}

trait Unpack {
    fn get_bool(&self, key: &str, default: bool, pos: Pos) -> Result<bool>;
    fn get_string(&self, key: &str, default: &str, pos: Pos) -> Result<String>;
    fn get_value(&self, key: &str, default: &str, pos: Pos) -> Result<String>;
    fn get_vec(&self, key: &str, pos: Pos) -> Result<(Iter<Yaml>, usize)>;
    fn get_custom_pairs(&self, key: &str, pos: Pos) -> Result<String>;
    fn custom_pairs(&self, string_only: bool, pos: Pos) -> Result<Vec<(String, String)>>;
    fn is_enabled(&self, key: &str) -> bool;
}

impl Unpack for Hash {
    fn get_bool(&self, key: &str, default: bool, (i, j): Pos) -> Result<bool> {
        match self.get(yaml_str![key]).unwrap_or(yaml_bool![default]) {
            Yaml::Boolean(b) => Ok(*b),
            _ => err!(format!("wrong {}: must be boolean ({}:{})", key, i, j)),
        }
    }
    fn get_string(&self, key: &str, default: &str, (i, j): Pos) -> Result<String> {
        match self.get(yaml_str![key]).unwrap_or(yaml_str![default]) {
            Yaml::String(s) => Ok(s.clone()),
            _ => err!(format!("wrong {}: must be string ({}:{})", key, i, j)),
        }
    }
    fn get_value(&self, key: &str, default: &str, (i, j): Pos) -> Result<String> {
        match self.get(yaml_str![key]).unwrap_or(yaml_str![default]) {
            Yaml::Real(s) | Yaml::String(s) => Ok(s.clone()),
            Yaml::Integer(v) => Ok(v.to_string()),
            _ => err!(format!(
                "wrong {}: must be integer, float or string ({}:{})",
                key, i, j
            )),
        }
    }
    fn get_vec(&self, key: &str, (i, j): Pos) -> Result<(Iter<Yaml>, usize)> {
        match self.get(yaml_str![key]).unwrap_or(yaml_bad![]) {
            Yaml::Array(a) => Ok((a.iter(), a.len())),
            Yaml::BadValue => Ok(([].iter(), 0)),
            _ => err!(format!("wrong {}: must be array ({}:{})", key, i, j)),
        }
    }
    fn get_custom_pairs(&self, key: &str, (i, j): Pos) -> Result<String> {
        match self.get(yaml_str![key]).unwrap_or(yaml_bad![]) {
            Yaml::Hash(h) => {
                let mut doc = String::new();
                for (k, v) in h.custom_pairs(false, (i, j))? {
                    doc.push_str(&format!("{}: {}, ", k, v));
                }
                Ok(doc)
            }
            Yaml::BadValue => Ok("".into()),
            _ => err!(format!("wrong {}: must be map ({}:{})", key, i, j)),
        }
    }
    fn custom_pairs(&self, string_only: bool, (i, j): Pos) -> Result<Vec<(String, String)>> {
        let mut doc = Vec::new();
        for (k, v) in self {
            let k = match k.as_str() {
                Some(v) => v,
                None => return err!(format!("invalid key {:?}: ({}:{})", k, i, j)),
            }
            .to_mixed_case();
            let v = if string_only {
                match v {
                    Yaml::String(s) => s.clone(),
                    _ => return err!(format!("invalid string {}: ({}:{})", k, i, j)),
                }
            } else {
                match v {
                    Yaml::Real(s) => s.clone(),
                    Yaml::String(s) => {
                        if s.is_empty() {
                            "".into()
                        } else {
                            format!("\"{}\"", s)
                        }
                    }
                    Yaml::Integer(v) => v.to_string(),
                    Yaml::Boolean(b) => b.to_string(),
                    _ => return err!(format!("invalid value {}: ({}:{})", k, i, j)),
                }
            };
            doc.push((k, v));
        }
        Ok(doc)
    }
    fn is_enabled(&self, key: &str) -> bool {
        match self.get(yaml_str![key]).unwrap_or(yaml_bool![true]) {
            Yaml::Boolean(false) => false,
            _ => true,
        }
    }
}

trait ToContainer
where
    Self: Sized,
{
    fn assert_hash(&self, msg: &str) -> Result<&Hash>;
    fn assert_vec(&self, msg: &str) -> Result<&Vec<Self>>;
}

impl ToContainer for Yaml {
    fn assert_hash(&self, msg: &str) -> Result<&Hash> {
        match self {
            Yaml::Hash(h) => Ok(h),
            _ => err!(msg),
        }
    }
    fn assert_vec(&self, msg: &str) -> Result<&Vec<Self>> {
        match self {
            Yaml::Array(a) => Ok(a),
            _ => err!(msg),
        }
    }
}

fn marked(e: Event) -> Event {
    match e {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))) => {
            let info = info.replace(' ', "");
            let mut head = String::new();
            if info.is_empty() {
                head.push_str("<pre><code>")
            } else {
                let lang = info.split('[').next().unwrap();
                let line = info
                    .replace(lang, "")
                    .replace(|s| (s == '[') | (s == ']'), "");
                head.push_str(&format!("<pre><code class=\"language-{}\"", lang));
                if !line.is_empty() {
                    head.push_str(&format!(" data-line-numbers=\"{}\"", line));
                }
                head.push_str(">");
            }
            Event::Html(head.into())
        }
        _ => e,
    }
}

fn parse(text: &str) -> String {
    let mut out = String::new();
    push_html(&mut out, Parser::new_ext(text, MARKED).map(marked));
    out
}

fn sized_block(img: &Hash, pos: Pos) -> Result<String> {
    let src = img.get_string("src", "", pos)?;
    if src.is_empty() {
        return err!(format!("No image source: ({}:{})", pos.0, pos.1));
    }
    let mut doc = format!(" src=\"{}\"", src);
    for attr in &["width", "height"] {
        let value = img.get_value(attr, "", pos)?;
        if !value.is_empty() {
            doc.push_str(&format!(" {}=\"{}\"", attr, value));
        }
    }
    Ok(doc)
}

fn img_block(img: &Hash, pos: Pos) -> Result<String> {
    let mut doc = format!(
        "<div class=\"img-column\"><figure><img{}/>",
        sized_block(img, pos)?
    );
    let label = img.get_string("label", "", pos)?;
    if !label.is_empty() {
        doc.push_str(&format!("<figcaption>{}</figcaption>", label));
    }
    doc.push_str("</figure></div>");
    Ok(doc)
}

fn content_block(slide: &Hash, pos: Pos, frag_count: &mut usize) -> Result<String> {
    let mut doc = String::new();
    let mut frag = FragMap::new(slide, pos, frag_count)?;
    let mut t = slide.get_string("doc", "", pos)?;
    if !t.is_empty() {
        doc.push_str(&frag.fragment("doc", &parse(&t)));
    }
    t = slide.get_string("include", "", pos)?;
    if !t.is_empty() {
        doc.push_str(&frag.fragment("include", &parse(&read_to_string(&t)?)));
    }
    match slide.get(yaml_str!["img"]).unwrap_or(yaml_vec![]) {
        Yaml::Array(imgs) => {
            if !imgs.is_empty() {
                doc.push_str("<div class=\"img-row\">");
                for img in imgs {
                    doc.push_str(&frag.fragment("img", &img_block(&img.as_hash().unwrap(), pos)?));
                }
                doc.push_str("</div>");
            }
        }
        Yaml::Hash(img) => {
            doc.push_str(&frag.fragment("img", &img_block(img, pos)?));
        }
        _ => {
            return err!(format!(
                "wrong img: must be map or array {}:{}",
                pos.0, pos.1
            ))
        }
    }
    t = slide.get_string("math", "", pos)?;
    if !t.is_empty() {
        doc.push_str(&frag.fragment("math", &format!("\\[{}\\]", t)));
    }
    let (stack, stack_len) = slide.get_vec("stack", pos)?;
    if stack_len > 0 {
        doc.push_str("<div style=\"display: flex\">");
        let width = 100. / stack_len as f32;
        for slide in stack {
            let slide = slide.assert_hash("unpack stack failed")?;
            doc.push_str(&format!(
                "<div style=\"width: {}%;text-align: center\">\n{}</div>",
                width,
                content_block(slide, pos, frag_count)?
            ));
        }
        doc.push_str("</div>");
    }
    Ok(doc)
}

fn slide_block(slide: &Hash, bg: &Background, pos: Pos) -> Result<String> {
    if slide.is_empty() {
        return err!(format!("empty slide block, ({}:{})", pos.0, pos.1));
    }
    let mut doc = String::from("<section");
    let mut t = slide.get_string("bg-color", "", pos)?;
    if !t.is_empty() {
        doc.push_str(&format!(" data-background-color=\"{}\"", t));
    }
    t = slide.get_string("trans", "", pos)?;
    if !t.is_empty() {
        doc.push_str(&format!(" data-transition=\"{}\"", t));
    }
    t = slide.get_string("bg-trans", "", pos)?;
    if !t.is_empty() {
        doc.push_str(&format!(" data-background-transition=\"{}\"", t));
    }
    if bg.is_valid() && slide.is_enabled("background") {
        let local_bg = Background::new(slide)?;
        doc.push_str(&if local_bg.is_valid() { &local_bg } else { bg }.attr());
    }
    doc.push_str(">");
    for title in &["title", "no-title"] {
        t = slide.get_string(title, "", pos)?;
        if !t.is_empty() {
            doc.push_str(&format!("<h2>{}</h2><hr/>", t));
        }
    }
    doc.push_str(&content_block(slide, pos, &mut 0)?);
    t = slide.get_string("note", "", pos)?;
    if !t.is_empty() {
        doc.push_str(&format!("<aside class=\"notes\">{}</aside>", parse(&t)));
    }
    doc.push_str("</section>");
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
        doc.push_str(&format!("<a href=\"{}\">", link));
    }
    doc.push_str(&format!("<img{}/>", sized_block(footer, (0, 0))?));
    let label = footer.get_string("label", "", (0, 0))?;
    if !label.is_empty() {
        doc.push_str(&format!("<span>&nbsp;{}</span>", label));
    }
    if !link.is_empty() {
        doc.push_str("</a>");
    }
    doc.push_str("\n</div></div></div>");
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
        doc.push_str("<section>");
        let slide = s.assert_hash(&format!("unpack slide failed: ({}:0)", i))?;
        doc.push_str(&slide_block(slide, &bg, (i, 0))?);
        let (sub, _) = slide.get_vec("sub", (i, 0))?;
        for (j, s) in sub.enumerate() {
            let slide = s.assert_hash(&format!("unpack slide failed: ({}:0)", i))?;
            doc.push_str(&slide_block(slide, &bg, (i, j))?);
        }
        if i == 0 {
            title = slide.get_string("title", "", (i, 0))?;
            if !meta.get_bool("outline", true, (i, 0))? {
                continue;
            }
            doc.push_str("<section");
            if bg.is_valid() {
                doc.push_str(&bg.attr());
            }
            doc.push_str("><h2>Outline</h2><hr/><ul>");
            for (i, s) in slides.iter().enumerate() {
                let s = s.assert_hash("unpack slide failed")?;
                let t = s.get_string("title", "", (i, 0))?;
                if t.is_empty() {
                    continue;
                }
                doc.push_str(&format!("<li><a href=\"#/{}\">{}</a></li>", i, t));
                let (sub, sub_len) = s.get_vec("sub", (i, 0))?;
                if sub_len == 0 {
                    continue;
                }
                doc.push_str("<ul>");
                for (j, s) in sub.enumerate() {
                    let s = s.assert_hash("unpack slide failed")?;
                    let t = s.get_string("title", "", (i, j))?;
                    if t.is_empty() {
                        continue;
                    }
                    doc.push_str(&format!("<li><a href=\"#/{}/{}\">{}</a></li>", i, j + 1, t));
                }
                doc.push_str("</ul>");
            }
            doc.push_str("</ul></section>");
        }
        doc.push_str("</section>");
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
