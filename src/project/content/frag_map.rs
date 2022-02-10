use std::{cell::Cell, collections::HashMap, rc::Rc};

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
    pub frag: Vec<HashMap<String, String>>,
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
                let i = self.counter.get();
                head += &format!(
                    "<span class=\"fragment {}\" data-fragment-index=\"{}\">",
                    frag, i
                );
                end += "</span>";
                self.counter.set(i + 1);
            }
        }
        head + text + &end
    }
}
