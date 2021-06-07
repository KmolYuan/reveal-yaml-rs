macro_rules! get_archive {
    () => {{
        use std::env::current_exe;
        let mut path = current_exe()?.with_file_name(ARCHIVE);
        path.set_extension("zip");
        path
    }};
}

pub use crate::blank::*;
pub use crate::fmt::*;
pub use crate::loader::*;
pub use crate::pack::*;
pub use crate::serve::*;
pub use crate::update::*;

mod blank;
mod fmt;
mod loader;
mod pack;
mod serve;
mod update;
