pub(crate) struct Error(pub(crate) &'static str, pub(crate) u64);

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
