use pulldown_cmark::{html::push_html, Options, Parser};
use yaml_rust::yaml::Hash;
use yaml_rust::{Yaml, YamlLoader};

const TEMPLATE: &str = include_str!("../assets/template.html");

macro_rules! err {
    ($v:expr) => {
        match $v {
            Ok(v) => v,
            Err(e) => e,
        }
    };
}

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
            None => return Err(format!("{}: {:?}", $msg, $pos).into()),
        }
    };
}

fn parse(text: &str) -> Result<String, String> {
    let parser = Parser::new_ext(
        text,
        Options::ENABLE_TABLES
            | Options::ENABLE_SMART_PUNCTUATION
            | Options::ENABLE_TASKLISTS
            | Options::ENABLE_STRIKETHROUGH,
    );
    let mut html_output = String::new();
    push_html(&mut html_output, parser);
    Ok(html_output)
}

fn check_slide(slide: &Hash, i: usize, j: usize) -> Result<String, String> {
    if slide.is_empty() {
        Err(format!("empty slide block, {:?}", (i, j)).to_string())
    } else {
        let mut doc = String::from("<section>");
        let mut t =
            String::from(unpack!(slide => "title" = yaml_str![], as_str, "wrong title", (i, j)));
        if !t.is_empty() {
            doc.push_str("<h2>");
            doc.push_str(&t);
            doc.push_str("</h2><hr/>");
        }
        t = String::from(unpack!(slide => "doc" = yaml_str![], as_str, "wrong doc", (i, j)));
        if !t.is_empty() {
            doc.push_str(&err!(parse(&t)));
        }
        t = String::from(unpack!(slide => "math" = yaml_str![], as_str, "wrong math", (i, j)));
        if !t.is_empty() {
            doc.push_str("\\[");
            doc.push_str(&t);
            doc.push_str("\\]");
        }
        doc.push_str("</section>");
        Ok(doc)
    }
}

fn inner_loader(yaml_str: &String) -> Result<String, String> {
    let yaml = match YamlLoader::load_from_str(yaml_str.as_str()) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };
    if yaml.len() < 2 {
        return Err(format!("Missing metadata").into());
    }
    let mut template = String::from(TEMPLATE);
    let meta = unpack!(yaml[0], as_hash, "meta must be key values", 0);
    template = template.replace(
        "{@description}",
        unpack!(meta => "description" = yaml_str![], as_str, "wrong description", (0, 0)),
    );
    template = template.replace(
        "{@author}",
        unpack!(meta => "author" = yaml_str![], as_str, "wrong author", (0, 0)),
    );
    template = template.replace(
        "{@theme}",
        unpack!(meta => "theme" = yaml_str!["serif"], as_str, "wrong theme", (0, 0)),
    );
    let mut doc = String::new();
    let mut title = String::new();
    for (i, s) in unpack!(yaml[1], as_vec, "slides must be array", 0)
        .iter()
        .enumerate()
    {
        doc.push_str("<section>");
        let slide = unpack!(s, as_hash, "unpack slide failed", (i, 0));
        if i == 0 {
            title.push_str(unpack!(slide => "title" = yaml_str![], as_str, "wrong title", (i, 0)));
        }
        doc.push_str(&err!(check_slide(slide, i, 0)));
        for (j, s) in unpack!(slide => "sub" = yaml_vec![], as_vec, "wrong title", (i, 0))
            .iter()
            .enumerate()
        {
            let slide = unpack!(s, as_hash, "unpack slide failed", (i, j));
            doc.push_str(&err!(check_slide(slide, i, j)));
        }
        doc.push_str("</section>");
    }
    template = template.replace("{@title}", &title);
    template = template.replace("{@slides}", &doc);
    Ok(template)
}

pub(crate) fn loader(yaml_str: String) -> String {
    match inner_loader(&yaml_str) {
        Ok(v) => v,
        Err(e) => {
            println!("ERROR: {}", e);
            String::from(format!("<section>{}</section>", e))
        }
    }
}
