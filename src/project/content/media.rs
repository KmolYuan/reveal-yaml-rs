use super::*;

/// Embed images.
#[derive(Default, serde::Deserialize)]
#[serde(default)]
pub struct Img {
    /// Image `<caption>`.
    pub label: String,
    /// Pop-up modal image, boolean `false` by default.
    pub pop: bool,
    /// This item is sized. (*flatten*)
    #[serde(flatten)]
    pub size: super::Sized,
}

/// Embed videos.
#[derive(serde::Deserialize)]
#[serde(default)]
pub struct Video {
    /// Allow controls, boolean `true` by default.
    controls: bool,
    /// Allow autoplay, boolean `false` by default.
    autoplay: bool,
    /// Video type, default to "video/mp4".
    pub r#type: String,
    /// This item is sized. (*flatten*)
    #[serde(flatten)]
    pub size: super::Sized,
}

impl Default for Video {
    fn default() -> Self {
        Self {
            controls: true,
            autoplay: false,
            r#type: "video/mp4".to_string(),
            size: super::Sized::default(),
        }
    }
}

/// Embed `<iframe>` structures, such as YouTube videos.
///
/// Please be aware that `<iframe>` maybe slow down your web browser and cause security issues!
#[derive(Default, serde::Deserialize)]
#[serde(default)]
pub struct IFrame {
    /// This item is sized. (*flatten*)
    #[serde(flatten)]
    pub size: super::Sized,
}

impl super::ToHtml for Img {
    fn to_html(self, _ctx: &Ctx) -> String {
        let Self { label, pop, size } = self;
        let pop = if pop {
            " class=\"img-pop\" onclick=\"show_modal(this)\" title=\"click to pop-up the image\""
        } else {
            ""
        };
        let s = format!("<img alt=\"{}\"{}{}/>", label, pop, size);
        if label.is_empty() {
            s
        } else {
            format!("<figure>{}<figcaption>{}</figcaption></figure>", s, label)
        }
    }
}

impl super::ToHtml for Video {
    fn to_html(self, _ctx: &Ctx) -> String {
        let Self {
            controls,
            autoplay,
            r#type,
            size,
        } = self;
        let (src, size) = size.size();
        let controls = if controls { " controls" } else { "" };
        let autoplay = if autoplay { " autoplay" } else { "" };
        format!(
            "<video{}{}{}><source{} type=\"{}\"></video>",
            size, controls, autoplay, src, r#type
        )
    }
}

impl super::ToHtml for IFrame {
    fn to_html(self, _ctx: &Ctx) -> String {
        let Self { size } = self;
        format!("<iframe{}></iframe>", size)
    }
}
