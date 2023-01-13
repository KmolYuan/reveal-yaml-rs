use super::*;
use std::collections::HashMap;

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

/// Other Reveal.js [options](https://revealjs.com/config/).
///
/// + Use any cased string to indicate the option; this function will translate
/// into lower camelcase. Those formats are allowed:
///
///   + `slide-number`
///   + `slide number`
///   + `slide_number`
///   + `slideNumber`
///
/// + This place is actually what `Reveal.initialize` input. So plugin options
/// should be placed here.
///
/// + Use `!!markdown` type on the string type, let us help you convert from
/// Markdown to HTML simply!
#[derive(Default, serde::Deserialize)]
#[serde(default)]
pub struct JsOption {
    /// Inner data structure. (*flatten*)
    #[serde(flatten)]
    pub inner: HashMap<String, JsType>,
}

impl ToHtml for JsOption {
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

impl ToHtml for JsType {
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
