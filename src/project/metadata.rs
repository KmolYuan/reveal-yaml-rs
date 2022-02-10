use super::{Ctx, StringWrap, ToHtml};
use yaml_peg::{serialize::Optional, Anchors};

const TEMPLATE: &str = include_str!("../assets/template.html");
const RELOAD: &str = "\
let ws = new WebSocket(\"ws://\" + window.location.host + \"/ws/\");
        ws.onmessage = _ => location.reload();";

/// Metadata contains HTML settings and global slide settings, they are totally YAML Maps.
///
/// The definition contains in the first YAML doc, split by horizontal line `---`.
#[derive(serde::Deserialize)]
#[serde(default)]
pub struct Metadata {
    /// Icon path, "img/icon.png" by default.
    pub icon: String,
    /// Set the "lang" attribute for the page, "en" by default.
    pub lang: String,
    /// The webpage title, defaults to the first page.
    pub title: String,
    /// Webpage description.
    pub description: String,
    /// Webpage author.
    pub author: String,
    /// Global background setting.
    pub background: super::Background,
    /// Auto generated table of the contents (TOC), `true` by default.
    pub outline: Optional<String>,
    /// Reveal.js theme, "serif" by default.
    pub theme: String,
    /// Highlight theme, "zenburn" by default.
    #[serde(rename = "code-theme")]
    pub code_theme: String,
    /// Extra CSS script in `<style>` tag.
    pub style: String,
    /// Global footer option. You can add your logo here.
    pub footer: super::Footer,
    /// Other Reveal.js options.
    pub option: super::JsOption,
    /// Third-party Reveal plugins.
    pub plugin: super::JsPlugin,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            icon: "help/icon.png".to_string(),
            lang: "en".to_string(),
            title: String::new(),
            description: String::new(),
            author: String::new(),
            background: super::Background::default(),
            outline: Optional::Bool(true),
            theme: "serif".to_string(),
            code_theme: "zenburn".to_string(),
            style: String::new(),
            footer: super::Footer::default(),
            option: super::JsOption::default(),
            plugin: super::JsPlugin::default(),
        }
    }
}

impl Metadata {
    /// Build HTML from template.
    pub fn build(
        self,
        slides: super::Slides,
        anchor: Anchors,
        mount: &str,
        auto_reload: bool,
    ) -> String {
        let Self {
            icon,
            lang,
            title,
            description,
            author,
            background,
            outline,
            theme,
            code_theme,
            style,
            footer,
            option,
            plugin,
        } = self;
        let outline = match outline {
            Optional::Bool(true) => "Outline".to_string(),
            Optional::Bool(false) => String::new(),
            Optional::Some(outline) => outline,
        };
        let ctx = Ctx {
            outline,
            anchor,
            background: background.to_html(&Default::default()),
            frag: Default::default(),
        };
        let title = match (title.as_str(), slides.slides.first()) {
            ("", Some(chapter)) => &chapter.slide.title,
            (title, _) => title,
        };
        let auto_reload = if auto_reload { RELOAD } else { "" };
        let (plugin_names, plugin_files) = plugin.name_and_files();
        TEMPLATE
            .replace("{%icon}", &icon)
            .replace("{%lang}", &lang)
            .replace("{%title}", &title.escape())
            .replace("{%description}", &description.escape())
            .replace("{%author}", &author.escape())
            .replace("{%theme}", &theme)
            .replace("{%code-theme}", &code_theme)
            .replace("{%footer}", &footer.to_html(&ctx))
            .replace("{%slides}", &slides.to_html(&ctx))
            .replace("/* {%auto-reload} */", auto_reload)
            .replace("/* {%option} */", &option.to_html(&ctx))
            .replace("/* {%style} */", &style)
            .replace("/* {%plugin} */", &plugin_names)
            .replace("<!-- {%plugin} -->", &plugin_files)
            .replace("{%mount}", mount)
    }
}
