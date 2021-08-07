pub(crate) trait WrapString {
    fn wrap(self, prefix: &str, suffix: &str) -> String;
    fn escape(self) -> String;
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

    fn escape(self) -> String {
        let s = self.as_ref();
        s.replace('\n', "\\n").replace('"', "\\\"")
    }
}
