pub(crate) struct Error(pub &'static str, pub u64);

impl From<u64> for Error {
    fn from(pos: u64) -> Self {
        Self("invalid value type", pos)
    }
}

impl From<(&'static str, u64)> for Error {
    fn from((v, pos): (&'static str, u64)) -> Self {
        Self(v, pos)
    }
}
