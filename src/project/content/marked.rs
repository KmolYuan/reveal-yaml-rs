use pulldown_cmark::{html::push_html, CodeBlockKind, Event, Options, Parser, Tag};
use std::fmt::Write;

const MARKED: Options = {
    let bits = Options::ENABLE_TABLES.bits()
        | Options::ENABLE_SMART_PUNCTUATION.bits()
        | Options::ENABLE_TASKLISTS.bits()
        | Options::ENABLE_STRIKETHROUGH.bits();
    Options::from_bits_truncate(bits)
};

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
                write!(head, "<pre><code class=\"language-{lang}\"").unwrap();
                if !line.is_empty() {
                    write!(head, " data-line-numbers=\"{line}\"").unwrap();
                }
                head += ">";
            }
            Event::Html(head.into())
        }
        _ => e,
    }
}

/// Translate Markdown to HTML.
pub fn md2html(text: &str) -> String {
    let mut doc = String::new();
    push_html(&mut doc, Parser::new_ext(text, MARKED).map(marked));
    doc
}
