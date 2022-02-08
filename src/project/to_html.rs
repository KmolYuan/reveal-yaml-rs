use std::cell::RefCell;

macro_rules! with_methods {
    ($($(#[$meta:meta])* fn $field:ident.$method:ident($ty:ty))+) => {$(
        $(#[$meta])*
        pub fn $method(&self, $field: $ty) {
            self.$field.replace($field);
        }
    )+};
}

/// A shared data between parent configuration and its children.
pub struct Ctx {
    /// The anchor of the original YAML file.
    pub anchor: yaml_peg::Anchors,
    /// Fragment setting.
    pub frag: RefCell<super::FragMap>,
    /// Background setting.
    pub background: RefCell<super::Background>,
}

impl Ctx {
    /// Create a context object.
    pub fn new(anchor: yaml_peg::Anchors) -> Self {
        Self {
            anchor,
            frag: RefCell::new(Default::default()),
            background: RefCell::new(Default::default()),
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
