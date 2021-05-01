use heck::MixedCase;
use pulldown_cmark::{html::push_html, CodeBlockKind, Event, Options, Parser, Tag};
use std::{collections::HashMap, fs::read_to_string, io::Result, slice::Iter};
use yaml_rust::{yaml::Hash, Yaml};

const MARKED: Options = Options::from_bits_truncate(
    Options::ENABLE_TABLES.bits()
        | Options::ENABLE_SMART_PUNCTUATION.bits()
        | Options::ENABLE_TASKLISTS.bits()
        | Options::ENABLE_STRIKETHROUGH.bits(),
);
pub(crate) type Pos = (usize, usize);

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
                head += &format!(
                    "<span class=\"fragment {}\" data-fragment-index=\"{}\">",
                    frag, index
                );
                end += "</span>";
            }
        }
        head + inner + &end
    }
}

pub(crate) trait Unpack {
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
                    doc += &format!("{}: {}, ", k, v);
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

pub(crate) trait ToContainer
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
                head += "<pre><code>"
            } else {
                let lang = info.split('[').next().unwrap();
                let line = info
                    .replace(lang, "")
                    .replace(|s| (s == '[') | (s == ']'), "");
                head += &format!("<pre><code class=\"language-{}\"", lang);
                if !line.is_empty() {
                    head += &format!(" data-line-numbers=\"{}\"", line);
                }
                head += ">";
            }
            Event::Html(head.into())
        }
        _ => e,
    }
}

pub(crate) fn parse(text: &str) -> String {
    let mut out = String::new();
    push_html(&mut out, Parser::new_ext(text, MARKED).map(marked));
    out
}

pub(crate) fn sized_block(img: &Hash, pos: Pos) -> Result<String> {
    let src = img.get_string("src", "", pos)?;
    if src.is_empty() {
        return err!(format!("No image source: ({}:{})", pos.0, pos.1));
    }
    let mut doc = format!(" src=\"{}\"", src);
    for attr in &["width", "height"] {
        let value = img.get_value(attr, "", pos)?;
        if !value.is_empty() {
            doc += &format!(" {}=\"{}\"", attr, value);
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
        doc += &format!("<figcaption>{}</figcaption>", label);
    }
    doc += "</figure></div>";
    Ok(doc)
}

pub(crate) fn content_block(slide: &Hash, pos: Pos, frag_count: &mut usize) -> Result<String> {
    let mut doc = String::new();
    let mut frag = FragMap::new(slide, pos, frag_count)?;
    let mut t = slide.get_string("doc", "", pos)?;
    if !t.is_empty() {
        doc += &frag.fragment("doc", &parse(&t));
    }
    t = slide.get_string("include", "", pos)?;
    if !t.is_empty() {
        doc += &frag.fragment("include", &parse(&read_to_string(&t)?));
    }
    match slide.get(yaml_str!["img"]).unwrap_or(yaml_vec![]) {
        Yaml::Array(imgs) => {
            if !imgs.is_empty() {
                doc += "<div class=\"img-row\">";
                for img in imgs {
                    doc += &frag.fragment("img", &img_block(&img.as_hash().unwrap(), pos)?);
                }
                doc += "</div>";
            }
        }
        Yaml::Hash(img) => {
            doc += &frag.fragment("img", &img_block(img, pos)?);
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
        doc += &frag.fragment("math", &format!("\\[{}\\]", t));
    }
    let (stack, stack_len) = slide.get_vec("stack", pos)?;
    if stack_len > 0 {
        doc += "<div style=\"display: flex\">";
        let width = 100. / stack_len as f32;
        for slide in stack {
            let slide = slide.assert_hash("unpack stack failed")?;
            doc += &format!(
                "<div style=\"width: {}%;text-align: center\">\n{}</div>",
                width,
                content_block(slide, pos, frag_count)?
            );
        }
        doc += "</div>";
    }
    Ok(doc)
}
