//! Rust implementation of [Reveal.js](https://github.com/hakimel/reveal.js) YAML server,
//! a command line interface (CLI) tool.
//!
//! *This documentation is prepared for inline-API.*
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
pub use crate::{
    blank::{blank, ROOT},
    fmt::fmt,
    pack::pack,
    serve::serve,
    update::update,
};

mod blank;
mod fmt;
mod pack;
pub mod project;
mod serve;
mod update;
