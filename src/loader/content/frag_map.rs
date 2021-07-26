use super::Error;
use std::collections::HashMap;
use yaml_peg::Node;

pub(crate) struct FragMap(HashMap<String, HashMap<usize, String>>);

impl FragMap {
    pub(crate) fn new(slide: &Node, count: &mut usize) -> Result<Self, Error> {
        let mut frag_map = HashMap::new();
        for h in slide.get_default("fragment", vec![], Node::as_array)? {
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

    pub(crate) fn fragment(&mut self, tag: &str, inner: &str) -> String {
        if inner.is_empty() {
            return "".to_string();
        }
        let tag = tag.to_string();
        let mut head = String::new();
        let mut end = String::new();
        if let Some(frag) = self.0.get(&tag) {
            for (index, frag) in frag {
                head += &format!(
                    "<span class=\"fragment {}\" data-fragment-index=\"{}\">",
                    frag, index
                );
                end += "</span>";
            }
        }
        head + inner + &end
    }
}
