use std::collections::HashMap;

/// Third-party Reveal plugins.
///
/// + The key is the plugin object names, such as `RevealNotes`.
///   The array is a list of plugin file paths, such as `plugin/notes/notes.js`.
/// + The external plugin folder can be placed nearing `reveal.yaml`.
/// + There is a repo demonstrate how to use [reveal.js-menu](https://github.com/denehyg/reveal.js-menu) plugin:
///   [Reveal.yaml-menu](https://github.com/KmolYuan/reveal.yaml-menu).
/// + [Here](https://github.com/hakimel/reveal.js/wiki/Plugins,-Tools-and-Hardware#plugins) is the plugin list recommend by official.
#[derive(Default, serde::Deserialize)]
#[serde(default)]
pub struct JsPlugin {
    /// Inner data structure. (*flatten*)
    #[serde(flatten)]
    pub inner: HashMap<String, Vec<String>>,
}

impl JsPlugin {
    /// Get plugin names and files.
    pub fn name_and_files(self) -> (String, String) {
        let mut names = String::new();
        let mut files = String::new();
        for (name, file) in self.inner {
            names += &format!("{}, ", name);
            for f in file {
                files += &format!("<script src=\"{}\"></script>\n", f);
            }
        }
        (names, files)
    }
}
