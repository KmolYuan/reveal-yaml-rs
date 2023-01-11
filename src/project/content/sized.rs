use super::*;
use yaml_peg::serde::Stringify;

/// Sized item option.
#[derive(Default, serde::Deserialize)]
#[serde(default)]
pub struct Sized {
    /// Source link.
    pub src: String,
    /// Item width.
    pub width: Stringify,
    /// Item height.
    pub height: Stringify,
}

impl Sized {
    /// Return size information.
    pub fn size(&self) -> (String, String) {
        let Self { src, width, height } = self;
        let src = src.wrap(" src=\"", "\"");
        let size = width.to_string().wrap(" width=\"", "\"")
            + &height.to_string().wrap(" height=\"", "\"");
        (src, size)
    }
}

impl std::fmt::Display for Sized {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (src, size) = self.size();
        write!(f, "{src}{size}")
    }
}
