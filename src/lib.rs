//! Rust implementation of [Reveal.js](https://github.com/hakimel/reveal.js) YAML server, a command line interface (CLI) tool.
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
pub use crate::blank::{blank, ROOT};
pub use crate::fmt::fmt;
pub use crate::pack::pack;
pub use crate::serve::serve;
pub use crate::update::update;

mod blank;
mod fmt;
mod loader;
mod pack;
mod serve;
mod update;
