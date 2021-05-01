macro_rules! err {
    ($msg:expr) => {{
        use std::io::{Error, ErrorKind};
        Err(Error::new(ErrorKind::InvalidData, $msg))
    }};
}

macro_rules! get_archive {
    () => {{
        use std::env::current_exe;
        let mut path = current_exe()?.with_file_name(ARCHIVE);
        path.set_extension("zip");
        path
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

pub use crate::fmt::*;
pub use crate::loader::*;
pub use crate::pack::*;
pub use crate::serve::*;
pub use crate::update::*;

mod content;
mod fmt;
mod loader;
mod pack;
mod serve;
mod update;
