use crate::into_json::IntoJson;

use self::string_checker::StringChecker;

mod string_checker;

pub struct ValueValidator(String);

impl ValueValidator {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn escape(self) -> String {
        self.0
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

    pub fn trim_quotes(&mut self) {
        self.0 = self
            .0
            .trim_start_matches('"')
            .trim_end_matches('"')
            .to_owned();
    }
}

impl IntoJson for ValueValidator {
    fn into_json(mut self) -> String {
        if self.0.is_vec() || self.0.is_bool() || self.0.is_number() {
            self.0
        } else {
            self.trim_quotes();
            format!("\"{}\"", self.escape())
        }
    }
}
