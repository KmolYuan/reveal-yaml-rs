macro_rules! err {
    ($msg:expr) => {{
        use std::io::{Error, ErrorKind};
        Err(Error::new(ErrorKind::InvalidData, $msg))
    }};
}

macro_rules! yaml_bad {
    [] => { &yaml_rust::Yaml::BadValue };
}

macro_rules! yaml_bool {
    [$b:expr] => { &yaml_rust::Yaml::Boolean($b) };
}

macro_rules! yaml_str {
    [] => { yaml_str![""] };
    [$t:expr] => { &yaml_rust::Yaml::String(String::from($t)) };
}

macro_rules! yaml_vec {
    [$($v:tt)?] => { &yaml_rust::Yaml::Array(vec![$($v)?]) };
}

pub use crate::loader::*;
pub use crate::pack::*;
pub use crate::serve::*;
pub use crate::update::*;

mod content;
mod loader;
mod pack;
mod serve;
mod update;
