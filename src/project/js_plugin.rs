use super::Error;
use yaml_peg::Node;

pub(crate) fn js_plugin(meta: &Node) -> Result<(String, String), Error> {
    if let Ok(plugin) = meta.get("plugin") {
        let mut names = String::new();
        let mut files = String::new();
        for (name, file) in plugin.as_map()? {
            names += name.as_str()?;
            names += ", ";
            for f in file.as_seq()? {
                files += &format!("<script src=\"{}\"></script>\n", f.as_str()?);
            }
        }
        Ok((names, files))
    } else {
        Ok(("".to_string(), "".to_string()))
    }
}

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
    pub inner: std::collections::HashMap<String, Vec<String>>,
}
