use super::*;
use yaml_peg::serde::Optional;

pub(crate) const GEAR_TAG: &str = "gear!";
pub(crate) const GEAR_URL: &str =
    "https://raw.githubusercontent.com/rust-lang/rust-artwork/master/logo/rust-logo-gear-only.svg";
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
    /// Webpage icon path, a Rust gear icon by default.
    pub icon: String,
    /// Webpage "lang" attribute, "en" by default.
    pub lang: String,
    /// Webpage title, defaults to the first page.
    pub title: String,
    /// Webpage description.
    pub description: String,
    /// Webpage author.
    pub author: String,
    /// Global background setting.
    ///
    /// For local option, please see [`Slide::background`].
    pub background: Background,
    /// Global [auto-animate](https://revealjs.com/auto-animate/) function.
    ///
    /// For local option, please see [`Slide::auto_animate`].
    #[serde(rename = "auto-animate")]
    pub auto_animate: bool,
    /// Show chapter header on the slides.
    #[serde(rename = "chapter-header")]
    pub chapter_header: bool,
    /// Auto generated table of the contents (TOC), `true` by default.
    ///
    /// To specify the outline title, just provide a string,
    /// a markdown level 1 title without `#` notation.
    ///
    /// Outline page has an id "outline".
    pub outline: Optional<String>,
    /// Reveal.js theme, "serif" by default.
    pub theme: String,
    /// Highlight theme, "zenburn" by default.
    #[serde(rename = "code-theme")]
    pub code_theme: String,
    /// Extra CSS script in `<style>` tag.
    pub style: String,
    /// Global footer option. You can add your logo here.
    pub footer: Footer,
    /// Other Reveal.js options.
    pub option: JsOption,
    /// Third-party Reveal plugins.
    pub plugin: JsPlugin,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            icon: GEAR_URL.to_string(),
            lang: "en".to_string(),
            title: String::new(),
            description: String::new(),
            author: String::new(),
            background: Background::default(),
            auto_animate: false,
            chapter_header: false,
            outline: Optional::Bool(true),
            theme: "serif".to_string(),
            code_theme: "zenburn".to_string(),
            style: String::new(),
            footer: Footer::default(),
            option: JsOption::default(),
            plugin: JsPlugin::default(),
        }
    }
}

impl Metadata {
    /// Build HTML from template.
    pub fn build(self, slides: Slides, mount: &str, auto_reload: bool) -> String {
        let Self {
            icon,
            lang,
            title,
            description,
            author,
            background,
            auto_animate,
            chapter_header,
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
            auto_animate,
            chapter_header: if chapter_header {
                Some(Default::default())
            } else {
                None
            },
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
