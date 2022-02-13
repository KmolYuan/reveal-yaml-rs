/// Some string operations.
pub trait StringWrap {
    /// Wrap string with prefix and suffix.
    fn wrap(&self, prefix: &str, suffix: &str) -> String;
    /// HTML escaping.
    fn escape(&self) -> String;
}

impl StringWrap for str {
    fn wrap(&self, prefix: &str, suffix: &str) -> String {
        if self.is_empty() {
            self.to_string()
        } else {
            format!("{}{}{}", prefix, self, suffix)
        }
    }

    fn escape(&self) -> String {
        self.replace('\n', "\\n").replace('"', "\\\"")
    }
}
