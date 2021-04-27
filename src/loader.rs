use pulldown_cmark::{html::push_html, CodeBlockKind, Event, Options, Parser, Tag};
use std::io::Result;
use yaml_rust::{yaml::Hash, Yaml, YamlLoader};

const TEMPLATE: &str = include_str!("assets/template.html");
const MARKED: Options = Options::from_bits_truncate(
    Options::ENABLE_TABLES.bits()
        | Options::ENABLE_SMART_PUNCTUATION.bits()
        | Options::ENABLE_TASKLISTS.bits()
        | Options::ENABLE_STRIKETHROUGH.bits(),
);

macro_rules! yaml_bool {
    [$bool:literal] => { &Yaml::Boolean($bool) };
}

macro_rules! yaml_str {
    [] => { yaml_str![""] };
    [$text:expr] => { &Yaml::String(String::from($text)) };
}

macro_rules! yaml_vec {
    [] => { &Yaml::Array(vec![]) };
}

macro_rules! unpack {
    ($v:expr$(=>$key:expr => $default:expr)?, $method:ident, $msg:expr) => {
        match $v$(.get(yaml_str!($key)).unwrap_or($default))?.$method() {
            Some(v) => v,
            None => return err!($msg),
        }
    };
    ($v:expr$(=>$key:literal => $default:expr)?, $method:ident) => {
        $v$(.get(yaml_str!($key)).unwrap_or($default))?.$method().unwrap()
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

fn sized_block(img: &Hash, i: usize, j: usize) -> Result<String> {
    let mut doc = String::new();
    for attr in &["width", "height"] {
        doc.push_str(&format!(" {}=", attr));
        let value = match img.get(yaml_str![*attr]).unwrap_or(yaml_str![]) {
            Yaml::Real(v) | Yaml::String(v) => v.clone(),
            Yaml::Integer(v) => v.to_string(),
            _ => return err!(format!("invalid attribute: {}:{}", i, j)),
        };
        doc.push_str(&format!("\"{}\"", value));
    }
    Ok(doc)
}

fn img_block(img: &Hash, i: usize, j: usize) -> Result<String> {
    let mut doc = String::from("<div class=\"img-column\"><figure>");
    let src = String::from(
        unpack!(img => "src" => yaml_str![], as_str, format!("wrong src: {}:{}", i, j)),
    );
    if src.is_empty() {
        return err!(format!("No image source: {}:{}", i, j));
    }
    doc.push_str(&format!("<img src=\"{}\"{}>", src, sized_block(img, i, j)?));
    let label = String::from(
        unpack!(img => "label" => yaml_str![], as_str, format!("wrong label: {}:{}", i, j)),
    );
    if !label.is_empty() {
        doc.push_str(&format!("<figcaption>{}</figcaption>", label));
    }
    doc.push_str("</figure></div>");
    Ok(doc)
}

fn slide_block(slide: &Hash, i: usize, j: usize) -> Result<String> {
    if slide.is_empty() {
        return err!(format!("empty slide block, {:?}", (i, j)));
    }
    let mut doc = String::from("<section>");
    let mut t = String::from(
        unpack!(slide => "title" => yaml_str![], as_str, format!("wrong title: {}:{}", i, j)),
    );
    if !t.is_empty() {
        doc.push_str(&format!("<h2>{}</h2><hr/>", t));
    }
    t = String::from(
        unpack!(slide => "no-title" => yaml_str![], as_str, format!("wrong no-title: {}:{}", i, j)),
    );
    if !t.is_empty() {
        doc.push_str(&format!("<h2>{}</h2><hr/>", t));
    }
    t = String::from(
        unpack!(slide => "doc" => yaml_str![], as_str, format!("wrong doc: {}:{}", i, j)),
    );
    if !t.is_empty() {
        doc.push_str(&parse(&t));
    }
    match slide.get(yaml_str!["img"]).unwrap_or(yaml_vec![]) {
        Yaml::Array(imgs) => {
            doc.push_str("<div class=\"img-row\">");
            for img in imgs {
                doc.push_str(&img_block(
                    unpack!(img, as_hash, format!("img must be key values: {}:{}", i, j)),
                    i,
                    j,
                )?);
            }
            doc.push_str("</div>");
        }
        Yaml::Hash(img) => {
            doc.push_str(&img_block(img, i, j)?);
        }
        _ => return err!(format!("wrong img: {:?}", (i, j))),
    }
    t = String::from(
        unpack!(slide => "math" => yaml_str![], as_str, format!("wrong math: {}:{}", i, j)),
    );
    if !t.is_empty() {
        doc.push_str(&format!("\\[{}\\]", t));
    }
    doc.push_str("</section>");
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
    let mut reveal = String::from(TEMPLATE).replace("{@mount}", mount);
    let mut title = String::new();
    let meta = unpack!(yaml[0], as_hash, "meta must be key values");
    let mut doc = String::new();
    for (i, s) in unpack!(yaml[1], as_vec, "slides must be array: 0")
        .iter()
        .enumerate()
    {
        doc.push_str("<section>");
        let slide = unpack!(s, as_hash, format!("unpack slide failed: {}:0", i));
        doc.push_str(&slide_block(slide, i, 0)?);
        for (j, s) in
            unpack!(slide => "sub" => yaml_vec![], as_vec, format!("wrong sub-slides: {}", i))
                .iter()
                .enumerate()
        {
            let slide = unpack!(s, as_hash, format!("unpack slide failed: {}:{}", i, j));
            doc.push_str(&slide_block(slide, i, j)?);
        }
        if i == 0 {
            title.push_str(
                unpack!(slide => "title" => yaml_str![], as_str, format!("wrong title: {}", i)),
            );
            if !unpack!(meta => "outline" => yaml_bool![true], as_bool, "wrong outline") {
                continue;
            }
            doc.push_str("<section><h2>Outline</h2><hr/><ul>");
            for (i, s) in unpack!(yaml[1], as_vec).iter().enumerate() {
                let s = unpack!(s, as_hash);
                let t = String::from(unpack!(s => "title" => yaml_str![], as_str));
                if t.is_empty() {
                    continue;
                }
                doc.push_str(&format!("<li><a href=\"#/{}\">{}</a></li>", i, t));
                let sub = Vec::from(unpack!(s => "sub" => yaml_vec![], as_vec).as_slice());
                if sub.is_empty() {
                    continue;
                }
                doc.push_str("<ul>");
                for (j, s) in sub.iter().enumerate() {
                    let s = unpack!(s, as_hash);
                    let t = String::from(unpack!(s => "title" => yaml_str![], as_str));
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
    for (attr, default) in &[
        ("icon", "img/icon.png"),
        ("title", &title),
        ("description", ""),
        ("author", ""),
        ("theme", "serif"),
        ("code-theme", "zenburn"),
    ] {
        reveal = reveal.replace(
            &format!("{{@{}}}", attr),
            unpack!(meta => *attr => yaml_str![*default], as_str, format!("wrong {}", attr)),
        );
    }
    reveal = reveal.replace("{@slides}", &doc);
    Ok(reveal)
}
