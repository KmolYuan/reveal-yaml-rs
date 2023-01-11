use std::{cell::Cell, collections::HashMap, fmt::Write, rc::Rc};

/// [Fragment](https://revealjs.com/fragments/) option.
///
/// + The index are the `data-fragment-index`.
/// + Block are **content**, but exclude stacks (sub-contents).
/// + Stacks can have local fragment option, but still ordered.
#[derive(Default, serde::Deserialize)]
#[serde(default)]
pub struct FragMap {
    #[serde(skip)]
    counter: Rc<Cell<u8>>,
    /// Fragment data structure.
    pub frag: Vec<HashMap<String, Option<String>>>,
}

impl FragMap {
    /// Set the counter.
    pub fn with_counter(&mut self, counter: Rc<Cell<u8>>) {
        self.counter = counter;
    }

    /// Wrap inner text with fragment options.
    pub fn wrap(&self, tag: &str, text: &str) -> String {
        if text.is_empty() {
            return String::new();
        }
        let mut head = String::new();
        let mut end = String::new();
        for map in &self.frag {
            if let Some(frag) = map.get(tag) {
                let frag = frag.as_ref().map(String::as_str).unwrap_or_default();
                let i = self.counter.get();
                write!(
                    head,
                    "<span class=\"fragment {frag}\" data-fragment-index=\"{i}\">"
                )
                .unwrap();
                end += "</span>";
                self.counter.set(i + 1);
            }
        }
        head + text + &end
    }
}
