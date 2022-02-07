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
    pub outline: bool,
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
    #[serde(skip)]
    mount: String,
    #[serde(skip)]
    auto_reload: bool,
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
            outline: true,
            theme: "serif".to_string(),
            code_theme: "zenburn".to_string(),
            style: String::new(),
            footer: super::Footer::default(),
            option: super::JsOption::default(),
            plugin: super::JsPlugin::default(),
            mount: String::new(),
            auto_reload: false,
        }
    }
}
