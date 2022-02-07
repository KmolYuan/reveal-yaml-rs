//! Reveal.yaml definition.
//!
//! For metadata, please see [`Metadata`]. For slides, please see [`ChapterSlide`].
//!
//! ## Tutorial
//!
//! **Slides in HTML**: In Reveal.js, the HTML structure shown as following. The typesetting is done by original HTML and CSS.
//!
//! ```html
//! <section> <!-- Horizontal slide 1 -->
//!   <section>...</section> <!-- Vertical slide 1 -->
//!   <section>...</section> <!-- Vertical slide 2 -->
//!   ...
//! </section>
//! <section> <!-- Horizontal slide 2 -->
//!   ...
//! </section>
//! ```
//!
//! **Slides in YAML**: The horizontal slides are as listed in the second block, which is an array. A slide can work with at least one attribute structure.
//!
//! ```yaml
//! # metadata block
//! description: ...
//! author: ...
//! ---
//! # slides block
//! - title: ...  # Works!
//! - doc: ...  # Works!
//! - img: ...  # Works!
//! ```
//!
//! The vertical slides work under the `sub` node of first slide, the attributes are same as horizontal slides.
//!
//! ```yaml
//! - title: Horizontal slide 1
//!   sub:
//!     - title: Vertical slide 1
//!     - title: Vertical slide 2
//! - title: Horizontal slide 2
//! ```
//!
//! This work supports YAML 1.2, and the anchor function supports for specific fields, such as content blocks.
//!
//! ### Layout
//!
//! There are two layout types, called "horizontal stack" (`hstack`) and "vertical stack" (`vstack`). The vertical layout is default, as same as HTML.
//!
//! The stack blocks list in the `hstack` / `vstack` tag, and the stack tags can be nested.
//!
//! ```yaml
//! hstack:
//!   - doc: Left
//!   - vstack:
//!     - doc: Right Top
//!     - doc: Right Bottom
//! ```
//!
//! The stack tag can add a dollar sign `$` to add a borderline between the elements.
//!
//! ```yaml
//! $hstack:
//!   - doc: Splitted left
//!   - doc: Splitted right
//! ```
//!
//! ### Sized Attribute
//!
//! The images and resizeable items are support "sized" attribute, which contains three options: `src`, `width` and `height`. The `src` option is required, otherwise the feature will be disabled or invalid.
//!
//! The `width` and `height` options are the same as the attributes on the `<img>` tag, they are optional.
//!
//! ```yaml
//! footer:  # Footer is a metadata option
//!   src: help/icon.png
//!   width: 50pt
//!   label: Reveal.yaml
//!   link: https://github.com/KmolYuan/reveal-yaml/
//! img:
//!   - src: img/icon.png
//!     width: 50%  # same as width="50%"
//!     height: 70  # same as height="70"
//! video:
//!   width: 320
//!   height: 240
//!   src: https://www.w3schools.com/html/movie.mp4
//!   autoplay: true
//! ```
//!
//! ### Reveal Plugins
//!
//! The plugins excluding `markdown` are enabled by default.
//!
//! ## Functions
//!
//! Here are the implemented features, or the functions are designed in progress.
//!
//! Generally, the parser will not check extra key values. In addition, almost all values support the `null` placeholder to present the default value (blank, `~`, `null`).
//!
//! Some functions are planed to be demonstrated in the help page. Open the help page by adding `/help/` after URL, like `http://localhost:8080/help/`.
pub use self::{
    background::*, content::*, footer::*, js_option::*, js_plugin::*, metadata::*, slides::*,
};
use self::{error::*, visible_title::*, wrap_string::*};
use std::io::{Error as IoError, ErrorKind};
use yaml_peg::{indicated_msg, parse, repr::RcRepr, Anchors, Node, Seq};

mod background;
mod content;
mod error;
mod footer;
mod js_option;
mod js_plugin;
mod metadata;
mod slides;
mod visible_title;
pub(crate) mod wrap_string;

const TEMPLATE: &str = include_str!("../assets/template.html");
const ICON: &str = "help/icon.png";
const RELOAD: &str = "\
let ws = new WebSocket(\"ws://\" + window.location.host + \"/ws/\");
        ws.onmessage = _ => location.reload();";

fn load_main(yaml: Seq<RcRepr>, v: &Anchors, mount: &str, reload: bool) -> Result<String, Error> {
    let meta = &yaml[0];
    let bg = Background::new(meta)?;
    let outline = meta.get_default("outline", true, Node::as_bool)?;
    let style = meta.get_default("style", "", Node::as_str)?;
    let (doc, title) = slides(yaml[1].as_seq()?, v, bg, outline)?;
    let title = meta.get_default("title", title.as_ref(), Node::as_str)?;
    let description = meta.get_default("description", title.as_ref(), Node::as_str)?;
    let author = meta.get_default("author", "", Node::as_str)?;
    let theme = meta.get_default("theme", "serif", Node::as_str)?;
    let code_theme = meta.get_default("code-theme", "zenburn", Node::as_str)?;
    let (plugin_names, plugin_files) = js_plugin(meta)?;
    let reload_script = if reload { RELOAD } else { "" };
    Ok(TEMPLATE
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
