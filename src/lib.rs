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
