use super::Error;
use yaml_peg::Node;

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
        doc += &lower_camelcase(k.as_str()?);
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
