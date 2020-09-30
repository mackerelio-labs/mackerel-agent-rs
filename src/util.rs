use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SANITIZER_REG: Regex = Regex::new("[^A-Za-z0-9_-]").unwrap();
}

// sanitize_metric_key sanitize metric keys to be Mackerel friendly
pub fn sanitize_metric_key(key: &str) -> String {
    SANITIZER_REG.replace_all(key, "_").into()
}
