use super::Error;
use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;
use yaml_peg::{Anchors, Node};

pub(crate) struct FragMapOld(HashMap<String, HashMap<usize, String>>);

impl FragMapOld {
    pub(crate) fn new(slide: &Node, v: &Anchors, count: &mut usize) -> Result<Self, Error> {
        let mut frag_map = HashMap::new();
        for h in slide.with(v, "fragment", vec![], Node::as_seq)? {
            for (k, v) in h.as_map()?.iter() {
                let k = k.as_str()?;
                let v = v.as_str()?;
                if !frag_map.contains_key(k) {
                    frag_map.insert(k.to_string(), HashMap::new());
                }
                frag_map.get_mut(k).unwrap().insert(*count, v.to_string());
            }
            *count += 1;
        }
        Ok(Self(frag_map))
    }

    pub(crate) fn wrap(&self, tag: &str, text: &str) -> String {
        if text.is_empty() {
            return "".to_string();
        }
        let mut head = String::new();
        let mut end = String::new();
        if let Some(frag) = self.0.get(tag) {
            for (index, frag) in frag {
                head += &format!(
                    "<span class=\"fragment {}\" data-fragment-index=\"{}\">",
                    frag, index
                );
                end += "</span>";
            }
        }
        head + text + &end
    }
}

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
    /// Inner data structure. (*flatten*)
    #[serde(flatten)]
    pub inner: Vec<std::collections::HashMap<String, String>>,
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
        for map in &self.inner {
            if let Some(frag) = map.get(tag) {
                head += &format!(
                    "<span class=\"fragment {}\" data-fragment-index=\"{}\">",
                    frag,
                    self.counter.get()
                );
                end += "</span>";
                self.counter.set(self.counter.get() + 1);
            }
        }
        head + text + &end
    }
}
