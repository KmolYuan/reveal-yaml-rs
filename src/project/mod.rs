//! Reveal.yaml definition.
//!
//! For metadata, please see [`Metadata`]. For slides, please see [`Slides`].
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
//! ## metadata block
//! description: ...
//! author: ...
//! ---
//! ## slides block
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
//! ### Multi-document Mode
//!
//! A regular Reveal.yaml project has two parts, metadata and a list of slides.
//! [`Metadata`] and [`Slides`].
//!
//! ```yaml
//! ## metadata block (map) - Metadata
//! ---
//! ## Slider block (sequence) - Sliders
//! - title: Title 1
//! - title: Title 2
//! ```
//!
//! Or multi-document with a leading metadata doc to reduce the indents.
//! [`Metadata`] and [`ChapterSlide`].
//!
//! ```yaml
//! ## metadata block (map) - Metadata
//! ---
//! ## Slider block 1 (map) - ChapterSlide
//! title: Title 1
//! ---
//! ## Slider block 2 (map) - ChapterSlide
//! title: Title 2
//! ```
//!
//! Please be aware that anchors cannot be referenced between different documents.
//!
//! ### Layout
//!
//! There are two layout types, called "horizontal stack" ([`Content::h_stack`]) and "vertical stack" ([`Content::v_stack`]). The vertical layout is default, as same as HTML.
//!
//! The stack blocks list in the `h-stack` / `v-stack` tag, and the stack tags can be nested.
//!
//! ```yaml
//! h-stack:
//!   - doc: Left
//!   - v-stack:
//!     - doc: Right Top
//!     - doc: Right Bottom
//! ```
//!
//! The stack tag can add a suffix `*-border` to add a borderline between the elements.
//! ([`Content::h_stack_border`] and [`Content::v_stack_border`])
//!
//! ```yaml
//! h-stack-border:
//!   - doc: Left
//!   - doc: Right
//! ```
//!
//! ### Sized Attribute
//!
//! The images and resizeable items are support [`Sized`] attribute, which contains three options: `src`, `width` and `height`. The `src` option is required, otherwise the feature will be disabled or invalid.
//!
//! The `width` and `height` options are the same as the attributes on the `<img>` tag, they are optional.
//!
//! ```yaml
//! footer:  # Footer is a metadata option
//!   src: img/icon.png
//!   width: 50pt
//!   label: Reveal.yaml
//!   link: https://github.com/KmolYuan/reveal-yaml/
//! img:
//!   - src: img/icon.png
//!     width: 50% # same as width="50%"
//!     height: 70 # same as height="70"
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
//! Additional plugins can be added via [`JsPlugin`] in metadata.
//!
//! ## Writing Guide
//!
//! Generally, `reveal.yaml` divides into two parts, the first part will be deserialized to [`Metadata`],
//! and the second part will be deserialized to [`Slides`], other parts will be ignored.
//! YAML parser will not check extra key values.
//!
//! During the deserialization, "flatten" field means the child field will be inherited by the parent,
//! such as [`Sized`].
//!
//! Most of functions are planed to be demonstrated in the help page.
//! Open the help page by adding `/help/` after URL, like `http://localhost:8080/help/`.
pub use self::{
    background::*, content::*, footer::*, js_option::*, js_plugin::*, metadata::*, slides::*,
    to_html::*, wrap_string::*,
};
use serde::Deserialize as _;
use std::io::{Error as IoError, ErrorKind};
use yaml_peg::{indicated_msg, parse, repr::RcRepr, serde::SerdeError};

mod background;
mod content;
mod footer;
mod js_option;
mod js_plugin;
mod metadata;
mod slides;
mod to_html;
mod wrap_string;

fn metadata<'d, D>(metadata: D) -> Result<Metadata, D::Error>
where
    D: serde::Deserializer<'d>,
{
    Metadata::deserialize(metadata)
}

pub(crate) fn load(doc: &str, mount: &str, auto_reload: bool) -> Result<String, IoError> {
    let yaml =
        parse::<RcRepr>(doc).map_err(|e| IoError::new(ErrorKind::InvalidData, e.to_string()))?;
    let disp = |SerdeError { msg, pos }| {
        eprintln!("{}\n{}", msg, indicated_msg(doc.as_bytes(), pos));
        IoError::new(ErrorKind::InvalidData, msg)
    };
    let (metadata, slides) = match yaml.as_slice() {
        [] => return Err(IoError::new(ErrorKind::InvalidData, "Empty project")),
        [n1] => {
            let slides = Vec::deserialize(n1.clone()).map_err(disp)?;
            (Metadata::default(), Slides { slides })
        }
        [n1, n2] => {
            let slides = if let Ok(slides) = Vec::deserialize(n2.clone()) {
                slides
            } else {
                vec![ChapterSlide::deserialize(n2.clone()).map_err(disp)?]
            };
            (metadata(n1.clone()).map_err(disp)?, Slides { slides })
        }
        [n1, ns @ ..] => {
            let slides = ns
                .iter()
                .map(|n| ChapterSlide::deserialize(n.clone()))
                .collect::<Result<_, _>>()
                .map_err(disp)?;
            (metadata(n1.clone()).map_err(disp)?, Slides { slides })
        }
    };
    Ok(metadata.build(slides, mount, auto_reload))
}

pub(crate) fn error_page(error: IoError) -> String {
    Slides::single("Error", format!("```\n{}\n```", error))
}
