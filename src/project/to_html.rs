use std::cell::RefCell;
use yaml_peg::Anchors;

macro_rules! with_methods {
    ($($(#[$meta:meta])* fn $field:ident.$method:ident($ty:ty))+) => {$(
        $(#[$meta])*
        pub fn $method(&self, $field: $ty) {
            self.$field.replace(Some($field));
        }
    )+};
}

/// A shared data between parent configuration and its children.
pub struct Ctx {
    /// The anchor of the original YAML file.
    pub anchor: Anchors,
    /// Fragment setting.
    pub frag: RefCell<Option<super::FragMap>>,
    /// Background setting.
    pub background: RefCell<Option<super::Background>>,
}

impl Ctx {
    /// Create a context object.
    pub fn new(anchor: Anchors) -> Self {
        Self {
            anchor,
            frag: RefCell::new(None),
            background: RefCell::new(None),
        }
    }

    with_methods! {
        /// Replace fragment setting.
        fn frag.with_frag(super::FragMap)
        /// Replace background setting.
        fn background.with_background(super::Background)
    }
}

/// Let data structure transform to HTML.
pub trait ToHtml {
    /// Transform option `self` into HTML string.
    ///
    /// Warn: The returned string might be only a partial of HTML.
    fn to_html(&self, ctx: &Ctx) -> String;
}
