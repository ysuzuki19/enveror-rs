pub(super) trait Modifier {
    fn escape(self) -> String;
    fn trim_quotes(self) -> String;
}

impl<S: AsRef<str>> Modifier for S {
    fn escape(self) -> String {
        self.as_ref()
            .replace('\\', "\\\\")
            .replace('\n', "\\n")
            .replace('\t', "\\t")
            .replace('"', "\\\"")
            .replace('\r', "\\r")
            .replace('\x08', "\\b") // バックスペース
            .replace('\x0C', "\\f") // フォームフィード
            .replace('\x1B', "\\u001B") // ANSIエスケープコードの開始
            .replace('\x07', "\\u0007") // ベル
    }

    fn trim_quotes(self) -> String {
        self.as_ref()
            .trim_start_matches('"')
            .trim_end_matches('"')
            .to_owned()
    }
}
