macro_rules! err {
    ($msg:expr) => {{
        use std::io::{Error, ErrorKind};
        Err(Error::new(ErrorKind::InvalidData, $msg))
    }};
}

pub use crate::loader::*;
pub use crate::serve::*;

mod loader;
mod serve;
