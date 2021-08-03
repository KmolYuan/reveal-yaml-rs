use super::Error;
use yaml_peg::Node;

pub(crate) fn js_plugin(meta: &Node) -> Result<(String, String), Error> {
    if let Ok(plugin) = meta.get("plugin") {
        let mut names = String::new();
        let mut files = String::new();
        for (name, file) in plugin.as_map()? {
            names += name.as_str()?;
            names += ", ";
            for f in file.as_array()? {
                files += &format!("<script src=\"{{%mount}}{}\"></script>\n", f.as_str()?);
            }
        }
        Ok((names, files))
    } else {
        Ok(("".to_string(), "".to_string()))
    }
}
