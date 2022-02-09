use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// A shared data between parent configuration and its children.
pub struct Ctx {
    /// The anchor of the original YAML file.
    pub anchor: yaml_peg::Anchors,
    /// Fragment counter.
    pub frag: Rc<Cell<u8>>,
    /// Background setting.
    pub background: RefCell<super::Background>,
}

impl Ctx {
    /// Create a context object.
    pub fn new(anchor: yaml_peg::Anchors) -> Self {
        Self {
            anchor,
            frag: Default::default(),
            background: Default::default(),
        }
    }

    /// Replace background setting.
    pub fn with_background(&self, bg: super::Background) {
        self.background.replace(bg);
    }
}

/// Let data structure transform to HTML.
pub trait ToHtml {
    /// Consume option `self` into HTML string.
    ///
    /// Warn: The returned string might be only a partial of HTML.
    fn to_html(self, ctx: &Ctx) -> String;
}

impl<T: ToHtml> ToHtml for yaml_peg::serialize::InlineList<T> {
    fn to_html(self, ctx: &Ctx) -> String {
        if self.is_single() {
            self.into_iter().map(|t| t.to_html(ctx)).collect()
        } else {
            self.into_iter().map(|t| t.to_html(ctx) + "\n").collect()
        }
    }
}
