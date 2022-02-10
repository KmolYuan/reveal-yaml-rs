use super::{md2html, Error, WrapString};
use crate::project::Ctx;
use std::collections::HashMap;
use yaml_peg::{Node, Yaml};

fn lower_camelcase(doc: &str) -> String {
    let mut s = String::new();
    let mut is_word = false;
    for c in doc.chars() {
        if " -_".contains(c) {
            is_word = true;
            continue;
        }
        s.push(if is_word {
            is_word = false;
            c.to_ascii_uppercase()
        } else {
            c
        });
    }
    s
}

pub(crate) fn js_option(meta: &Node) -> Result<String, Error> {
    let meta = match meta.get("option") {
        Ok(n) => n,
        Err(_) => return Ok(String::new()),
    };
    let mut doc = String::new();
    for (k, v) in meta.as_map()? {
        doc += "\n";
        doc += &" ".repeat(8);
        doc += &lower_camelcase(k.as_str()?);
        doc += ": ";
        doc += &as_json(&v)?;
        doc += ",";
    }
    Ok(doc)
}

fn as_json(n: &Node) -> Result<String, Error> {
    match n.yaml() {
        Yaml::Str(s) => Ok(if n.tag() == "markdown" {
            format!("\"{}\"", md2html(s).escape())
        } else {
            format!("\"{}\"", s.escape())
        }),
        Yaml::Int(s) | Yaml::Float(s) => Ok(s.clone()),
        Yaml::Bool(true) => Ok("true".to_string()),
        Yaml::Bool(false) => Ok("false".to_string()),
        Yaml::Seq(a) => {
            let mut s = "[".to_string();
            for n in a {
                s += &as_json(n)?;
                s += ", ";
            }
            Ok(s + "]")
        }
        Yaml::Map(m) => {
            let mut s = "{".to_string();
            for (k, v) in m {
                s += &lower_camelcase(k.as_str()?);
                s += ": ";
                s += &as_json(v)?;
                s += ", ";
            }
            Ok(s + "}")
        }
        Yaml::Null => Ok("null".to_string()),
        Yaml::Anchor(_) => Err(Error("option is not support using anchor", n.pos())),
    }
}

/// Other Reveal.js [options](https://revealjs.com/config/).
///
/// + Use any case string to indicate the option, this function will translate into lower camelcase, for example, YAML `slide number: c/t` will be JavaScript `slideNumber: "c/t"`.
/// + This place is actually what `Reveal.initialize` input. So plugin options should be placed here.
/// + Use `!!markdown` type on the string type, let us help you convert from Markdown to HTML simply!
#[derive(Default, serde::Deserialize)]
#[serde(default)]
pub struct JsOption {
    /// Inner data structure. (*flatten*)
    #[serde(flatten)]
    pub inner: HashMap<String, JsType>,
}

impl super::ToHtml for JsOption {
    fn to_html(self, _ctx: &Ctx) -> String {
        self.inner
            .into_iter()
            .map(|(k, j)| {
                "\n".to_string()
                    + &" ".repeat(8)
                    + &lower_camelcase(&k)
                    + ": "
                    + &j.to_html(_ctx)
                    + ","
            })
            .collect()
    }
}

/// The union type of the options.
#[derive(serde::Deserialize)]
#[serde(untagged)]
pub enum JsType {
    /// Boolean values.
    Bool(bool),
    /// Integer values.
    Int(u32),
    /// Float values.
    Float(f32),
    /// String values.
    String(String),
    /// Sequence values.
    Seq(Vec<Self>),
    /// A subsequence of options, map-like.
    Map(HashMap<String, Self>),
}

impl Default for JsType {
    fn default() -> Self {
        Self::String(String::new())
    }
}

impl super::ToHtml for JsType {
    fn to_html(self, _ctx: &Ctx) -> String {
        match self {
            JsType::Bool(b) => if b { "true" } else { "false" }.to_string(),
            JsType::Int(n) => n.to_string(),
            JsType::Float(n) => n.to_string(),
            JsType::String(s) => format!("\"{}\"", s.escape()),
            JsType::Seq(seq) => {
                "[".to_string()
                    + &seq
                        .into_iter()
                        .map(|j| j.to_html(_ctx))
                        .collect::<Vec<_>>()
                        .join(", ")
                    + "]"
            }
            JsType::Map(map) => {
                "{".to_string()
                    + &map
                        .into_iter()
                        .map(|(k, v)| format!("{}: {}", lower_camelcase(&k), v.to_html(_ctx)))
                        .collect::<Vec<_>>()
                        .join(",\n")
                    + "}"
            }
        }
    }
}
