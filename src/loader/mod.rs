use self::background::Background;
use self::content::*;
use self::error::Error;
use self::footer::footer;
use self::js_option::js_option;
use self::js_plugin::js_plugin;
use self::slides::slides;
use self::visible_title::visible_title;
use self::wrap_string::WrapString;
use std::io::{Error as IoError, ErrorKind};
use yaml_peg::{indicated_msg, parse, repr::RcRepr, Anchors, Array, Node};

mod background;
mod content;
mod error;
mod footer;
mod js_option;
mod js_plugin;
mod slides;
mod visible_title;
pub(crate) mod wrap_string;

const TEMPLATE: &str = include_str!("../assets/template.html");
const ICON: &str = "help/icon.png";
const RELOAD: &str = "setInterval(function() { $.ajax({ url: '/changed/', contentType: 'json', success: data => { if (data['modified']) location.reload(); }}); }, 1000);";

fn load_main(yaml: Array<RcRepr>, v: &Anchors, mount: &str, reload: bool) -> Result<String, Error> {
    let meta = &yaml[0];
    let bg = Background::new(meta)?;
    let outline = meta.get_default("outline", true, Node::as_bool)?;
    let style = meta.get_default("style", "", Node::as_str)?;
    let (doc, title) = slides(yaml[1].as_array()?, v, bg, outline)?;
    let title = meta.get_default("title", title.as_ref(), Node::as_str)?;
    let description = meta.get_default("description", title.as_ref(), Node::as_str)?;
    let author = meta.get_default("author", "", Node::as_str)?;
    let theme = meta.get_default("theme", "serif", Node::as_str)?;
    let code_theme = meta.get_default("code-theme", "zenburn", Node::as_str)?;
    let (plugin_names, plugin_files) = js_plugin(meta)?;
    let reload_script = if reload { RELOAD } else { "" };
    Ok(TEMPLATE
        .to_string()
        .replace("{%icon}", meta.get_default("icon", ICON, Node::as_str)?)
        .replace("{%lang}", meta.get_default("lang", "en", Node::as_str)?)
        .replace("{%title}", &title.escape())
        .replace("{%description}", &description.escape())
        .replace("{%author}", &author.escape())
        .replace("{%theme}", theme)
        .replace("{%code-theme}", code_theme)
        .replace("{%footer}", &footer(meta)?)
        .replace("{%slides}", &doc)
        .replace("/* {%auto-reload} */", reload_script)
        .replace("/* {%option} */", &js_option(meta)?)
        .replace("/* {%style} */", style)
        .replace("/* {%plugin} */", &plugin_names)
        .replace("<!-- {%plugin} -->", &plugin_files)
        .replace("{%mount}", mount))
}

/// Load YAML string as HTML.
pub(crate) fn loader(yaml_str: &str, mount: &str, reload: bool) -> Result<String, IoError> {
    let (yaml, anchor) = parse(yaml_str).map_err(|s| IoError::new(ErrorKind::InvalidData, s))?;
    if yaml.len() < 2 {
        return Err(IoError::new(
            ErrorKind::InvalidData,
            "Missing metadata or slides".to_string(),
        ));
    }
    load_main(yaml, &anchor, mount, reload).map_err(|Error(name, pos)| {
        IoError::new(
            ErrorKind::InvalidData,
            format!("{}:\n{}", name, indicated_msg(yaml_str, pos)),
        )
    })
}
