use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Default, Debug, Deserialize, PartialEq)]
pub struct Config {
    #[serde(default = "Config::default_apibase")]
    pub apibase: String,
    pub apikey: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            apibase: String::new(),
            apikey: String::new(),
            roles: Some(vec![]),
            custom: None,
        }
    }

    pub fn default_apibase() -> String {
        "https://api.mackerelio.com/".to_owned()
    }

    pub fn from_file(conf_path: &Path) -> Self {
        Self::from_toml(&fs::read_to_string(conf_path).unwrap())
    }

    fn from_toml(toml: &str) -> Self {
        toml::from_str(toml).unwrap()
    }
}

#[test]
fn test_from_toml() {
    let toml = r#"
apibase = "https://example.com"
apikey = "example_apikey"
roles = ["example_service: example_role"]
custom = "ls -l"
"#;
    let expected = Config {
        apibase: "https://example.com".to_owned(),
        apikey: "example_apikey".to_owned(),
        roles: Some(vec!["example_service: example_role".to_owned()]),
        custom: Some("ls -l".into()),
    };
    assert_eq!(Config::from_toml(toml), expected);
}

#[test]
fn test_from_toml_without_apibase() {
    let toml = r#"
apikey = "example_apikey"
roles = ["example_service: example_role"]
custom = "ls -l"
"#;
    let expected = Config {
        apikey: "example_apikey".to_owned(),
        apibase: "https://api.mackerelio.com/".to_owned(),
        roles: Some(vec!["example_service: example_role".to_owned()]),
        custom: Some("ls -l".into()),
    };
    assert_eq!(Config::from_toml(toml), expected);
}

#[test]
fn test_from_toml_without_roles() {
    let toml = r#"
apikey = "example_apikey"
apibase = "https://example.com"
custom = "ls -l"
"#;
    let expected = Config {
        apibase: "https://example.com".to_owned(),
        apikey: "example_apikey".to_owned(),
        roles: None,
        custom: Some("ls -l".into()),
    };
    assert_eq!(Config::from_toml(toml), expected);
}

#[test]
fn test_from_toml_without_custom() {
    let toml = r#"
apikey = "example_apikey"
apibase = "https://example.com"
roles = ["example_service: example_role"]
"#;
    let expected = Config {
        apibase: "https://example.com".to_owned(),
        apikey: "example_apikey".to_owned(),
        roles: Some(vec!["example_service: example_role".to_owned()]),
        custom: None,
    };
    assert_eq!(Config::from_toml(toml), expected);
}
