use std::{cell::Cell, rc::Rc};

/// A shared data between parent configuration and its children.
#[derive(Default)]
pub struct Ctx {
    /// Outline option.
    pub outline: String,
    /// The anchor of the original YAML file.
    pub anchor: yaml_peg::Anchors,
    /// Background setting (in HTML).
    pub background: String,
    /// Fragment counter.
    pub frag: Rc<Cell<u8>>,
}

/// Let data structure transform to HTML.
pub trait ToHtml {
    /// Consume option `self` into HTML string.
    ///
    /// Warn: The returned string might be only a partial of HTML.
    fn to_html(self, ctx: &Ctx) -> String;
}

impl<I: IntoIterator> ToHtml for I
where
    I::Item: ToHtml,
{
    fn to_html(self, ctx: &Ctx) -> String {
        self.into_iter().map(|t| t.to_html(ctx) + "\n").collect()
    }
}
