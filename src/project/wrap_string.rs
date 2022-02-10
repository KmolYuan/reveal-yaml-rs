/// Some string operations.
pub trait StringWrap {
    /// Wrap string with prefix and suffix.
    fn wrap(&self, prefix: &str, suffix: &str) -> String;
    /// HTML escaping.
    fn escape(&self) -> String;
}

impl<S: AsRef<str>> StringWrap for S {
    fn wrap(&self, prefix: &str, suffix: &str) -> String {
        let s = self.as_ref();
        if s.is_empty() {
            s.to_string()
        } else {
            format!("{}{}{}", prefix, s, suffix)
        }
    }

    fn escape(&self) -> String {
        let s = self.as_ref();
        s.replace('\n', "\\n").replace('"', "\\\"")
    }
}
