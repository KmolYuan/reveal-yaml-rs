use crate::project::{wrap_string::StringWrap, Ctx};
use yaml_peg::serialize::Stringify;

/// Background setting.
#[derive(serde::Deserialize)]
#[serde(untagged)]
pub enum Background {
    /// [Color Backgrounds](https://revealjs.com/backgrounds/#color-backgrounds),
    /// a color string.
    Color(String),
    /// [Image Backgrounds](https://revealjs.com/backgrounds/#image-backgrounds).
    Img(ImgBackground),
}

impl Default for Background {
    fn default() -> Self {
        Self::Img(ImgBackground::default())
    }
}

impl super::ToHtml for Background {
    fn to_html(self, _ctx: &Ctx) -> String {
        match self {
            Background::Color(color) => color.wrap(" data-background-color=\"", "\""),
            Background::Img(img) => img.to_html(_ctx),
        }
    }
}

/// Image backgrounds setting.
#[derive(Default, serde::Deserialize)]
#[serde(default)]
pub struct ImgBackground {
    /// Background source.
    pub src: String,
    /// Background size.
    pub size: Stringify,
    /// Background position.
    pub position: String,
    /// Background repeat. (repeat / no-repeat)
    pub repeat: String,
    /// Background opacity from zero to one.
    pub opacity: Stringify,
}

impl super::ToHtml for ImgBackground {
    fn to_html(self, _ctx: &Ctx) -> String {
        let Self {
            src,
            size,
            position,
            repeat,
            opacity,
        } = self;
        if src.is_empty() {
            String::new()
        } else {
            format!(" src={}", src)
                + &size.to_string().wrap(" data-background-size=\"", "\"")
                + &position.wrap(" data-background-position=\"", "\"")
                + &repeat.wrap(" data-background-repeat=\"", "\"")
                + &opacity
                    .to_string()
                    .wrap(" data-background-opacity=\"", "\"")
        }
    }
}
