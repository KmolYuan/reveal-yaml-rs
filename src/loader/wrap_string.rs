pub(crate) trait WrapString {
    fn wrap(self, prefix: &str, suffix: &str) -> String;
}

impl<S: AsRef<str>> WrapString for S {
    fn wrap(self, prefix: &str, suffix: &str) -> String {
        let s = self.as_ref();
        if s.is_empty() {
            s.to_string()
        } else {
            format!("{}{}{}", prefix, s, suffix)
        }
    }
}
