mod modifier;
mod type_checker;

use crate::into_json::IntoJson;

use self::modifier::Modifier;
use self::type_checker::TypeChecker;

impl IntoJson for &str {
    fn into_json(self) -> String {
        if self.is_vec() || self.is_bool() || self.is_number() {
            self.into()
        } else {
            let modified = self.trim_quotes().escape();

            format!("\"{modified}\"")
        }
    }
}
