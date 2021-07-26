use pulldown_cmark::{html::push_html, CodeBlockKind, Event, Options, Parser, Tag};

const MARKED: Options = Options::from_bits_truncate(
    Options::ENABLE_TABLES.bits()
        | Options::ENABLE_SMART_PUNCTUATION.bits()
        | Options::ENABLE_TASKLISTS.bits()
        | Options::ENABLE_STRIKETHROUGH.bits(),
);

fn marked(e: Event) -> Event {
    match e {
        // Support line number for code block
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

pub(in crate::loader) fn md2html(text: &str) -> String {
    if text.is_empty() {
        "".to_string()
    } else {
        let mut out = String::new();
        push_html(&mut out, Parser::new_ext(text, MARKED).map(marked));
        out
    }
}
