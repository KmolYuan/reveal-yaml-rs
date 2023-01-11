use super::*;

/// [Layout stack](https://revealjs.com/layout/#stack) for images.
#[derive(Default, serde::Deserialize)]
#[serde(default)]
pub struct LayImg {
    /// The animation option. Independent from `FragMap` option.
    pub frag: Option<String>,
    /// This item is sized. (*flatten*)
    #[serde(flatten)]
    pub size: Sized,
}

impl ToHtml for LayImg {
    fn to_html(self, _ctx: &Ctx) -> String {
        let Self { frag, size } = self;
        let frag = frag.unwrap_or_default();
        format!("<img class=\"fragment {frag}\"{size}/>")
    }
}
