use super::{md2html, Error};
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
        Yaml::Str(s) => Ok({
            let s = if n.ty() == "markdown" {
                md2html(s)
            } else {
                s.clone()
            };
            format!("\"{}\"", s.replace('\n', "\\n").replace('"', "\\\""))
        }),
        Yaml::Int(s) | Yaml::Float(s) => Ok(s.clone()),
        Yaml::Bool(true) => Ok("true".to_string()),
        Yaml::Bool(false) => Ok("false".to_string()),
        Yaml::Array(a) => {
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
