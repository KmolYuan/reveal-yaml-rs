use pulldown_cmark::{html::push_html, CodeBlockKind, Event, Options, Parser, Tag};
use std::io::{Error, ErrorKind, Result};
use yaml_rust::{yaml::Hash, Yaml, YamlLoader};

const TEMPLATE: &str = include_str!("assets/template.html");

macro_rules! yaml_str {
    [] => { yaml_str![""] };
    [$text:literal] => { &Yaml::String(String::from($text)) };
}

macro_rules! yaml_vec {
    [] => { &Yaml::Array(vec![]) };
}

macro_rules! unpack {
    ($v:expr$(=>$key:literal = $default:expr)?, $method:ident, $msg:literal, $pos:expr) => {
        match $v$(.get(yaml_str!($key)).unwrap_or($default))?.$method() {
            Some(v) => v,
            None => return Err(Error::new(ErrorKind::InvalidData, format!("{}: {:?}", $msg, $pos))),
        }
    };
    ($v:expr$(=>$key:literal = $default:expr)?, $method:ident) => {
        $v$(.get(yaml_str!($key)).unwrap_or($default))?.$method().unwrap()
    }
}

fn parse(text: &str) -> String {
    let parser = Parser::new_ext(
        text,
        Options::ENABLE_TABLES
            | Options::ENABLE_SMART_PUNCTUATION
            | Options::ENABLE_TASKLISTS
            | Options::ENABLE_STRIKETHROUGH,
    )
    .map(|e| match e {
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
    });
    let mut out = String::new();
    push_html(&mut out, parser);
    out
}

fn slide_block(slide: &Hash, i: usize, j: usize) -> Result<String> {
    if slide.is_empty() {
        Err(Error::new(
            ErrorKind::InvalidData,
            format!("empty slide block, {:?}", (i, j)),
        ))
    } else {
        let mut doc = String::from("<section>");
        let mut t =
            String::from(unpack!(slide => "title" = yaml_str![], as_str, "wrong title", (i, j)));
        if !t.is_empty() {
            doc.push_str(&format!("<h2>{}</h2><hr/>", t));
        }
        t = String::from(unpack!(slide => "doc" = yaml_str![], as_str, "wrong doc", (i, j)));
        if !t.is_empty() {
            doc.push_str(&parse(&t));
        }
        t = String::from(unpack!(slide => "math" = yaml_str![], as_str, "wrong math", (i, j)));
        if !t.is_empty() {
            doc.push_str(&format!("\\[{}\\]", t));
        }
        doc.push_str("</section>");
        Ok(doc)
    }
}

/// Load YAML string as HTML.
pub fn loader(yaml_str: String) -> Result<String> {
    let yaml = match YamlLoader::load_from_str(yaml_str.as_str()) {
        Ok(v) => v,
        Err(e) => return Err(Error::new(ErrorKind::InvalidData, e.to_string())),
    };
    if yaml.len() < 2 {
        return Err(Error::new(ErrorKind::InvalidData, "Missing metadata"));
    }
    let mut reveal = String::from(TEMPLATE);
    let meta = unpack!(yaml[0], as_hash, "meta must be key values", 0);
    reveal = reveal.replace(
        "{@description}",
        unpack!(meta => "description" = yaml_str![], as_str, "wrong description", (0, 0)),
    );
    reveal = reveal.replace(
        "{@author}",
        unpack!(meta => "author" = yaml_str![], as_str, "wrong author", (0, 0)),
    );
    reveal = reveal.replace(
        "{@theme}",
        unpack!(meta => "theme" = yaml_str!["serif"], as_str, "wrong theme", (0, 0)),
    );
    let mut doc = String::new();
    for (i, s) in unpack!(yaml[1], as_vec, "slides must be array", 0)
        .iter()
        .enumerate()
    {
        doc.push_str("<section>");
        let slide = unpack!(s, as_hash, "unpack slide failed", (i, 0));
        doc.push_str(&slide_block(slide, i, 0)?);
        for (j, s) in unpack!(slide => "sub" = yaml_vec![], as_vec, "wrong title", (i, 0))
            .iter()
            .enumerate()
        {
            let slide = unpack!(s, as_hash, "unpack slide failed", (i, j));
            doc.push_str(&slide_block(slide, i, j)?);
        }
        if i == 0 {
            reveal = reveal.replace(
                "{@title}",
                &unpack!(slide => "title" = yaml_str![], as_str, "wrong title", (i, 0)),
            );
            doc.push_str("<section><h2>Outline</h2><hr/><ul>");
            for (i, s) in unpack!(yaml[1], as_vec).iter().enumerate() {
                let s = unpack!(s, as_hash);
                let t = String::from(unpack!(s => "title" = yaml_str![], as_str));
                if t.is_empty() {
                    continue;
                }
                doc.push_str(&format!("<li><a href=\"#/{}\">{}</a></li>", i, t));
                let sub = Vec::from(unpack!(s => "sub" = yaml_vec![], as_vec).as_slice());
                if sub.is_empty() {
                    continue;
                }
                doc.push_str("<ul>");
                for (j, s) in sub.iter().enumerate() {
                    let s = unpack!(s, as_hash);
                    let t = String::from(unpack!(s => "title" = yaml_str![], as_str));
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
    reveal = reveal.replace("{@slides}", &doc);
    Ok(reveal)
}
