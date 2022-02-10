use super::*;

/// [Layout stack](https://revealjs.com/layout/#stack) for images.
#[derive(Default, serde::Deserialize)]
#[serde(default)]
pub struct LayImg {
    /// The animation option. Independent from `FragMap` option.
    pub fragment: String,
    /// This item is sized. (*flatten*)
    #[serde(flatten)]
    pub size: Sized,
}

impl ToHtml for LayImg {
    fn to_html(self, _ctx: &Ctx) -> String {
        let Self { fragment, size } = self;
        format!("<img class=\"fragment {}\"{}/>", fragment, size)
    }
}
