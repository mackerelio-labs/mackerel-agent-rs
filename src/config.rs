use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(default)]
pub struct Config {
    pub apibase: String,
    pub apikey: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            apibase: "https://api.mackerelio.com/".to_string(),
            apikey: String::new(),
            roles: None,
            display_name: None,
        }
    }
}

impl Config {
    pub fn from_file(conf_path: &Path) -> Self {
        Self::from_toml(&fs::read_to_string(conf_path).unwrap())
    }

    fn from_toml(toml: &str) -> Self {
        toml::from_str(toml).unwrap()
    }
}

#[test]
fn test_with_minimum_essentials() {
    let toml = r#"
apikey = "my-api-key"
"#;
    let expected = Config {
        apikey: "my-api-key".to_string(),
        ..Default::default()
    };
    assert_eq!(Config::from_toml(toml), expected);
}

#[test]
fn test_from_toml() {
    let toml = r#"
apibase = "https://example.com"
apikey = "example_apikey"
roles = ["example_service: example_role"]
display_name = "my-host"
"#;
    let expected = Config {
        apibase: "https://example.com".to_owned(),
        apikey: "example_apikey".to_owned(),
        roles: Some(vec!["example_service: example_role".to_owned()]),
        display_name: Some("my-host".to_string()),
    };
    assert_eq!(Config::from_toml(toml), expected);
}

#[test]
fn test_from_toml_without_apibase() {
    let toml = r#"
apikey = "example_apikey"
roles = ["example_service: example_role"]
"#;
    let expected = Config {
        apikey: "example_apikey".to_owned(),
        roles: Some(vec!["example_service: example_role".to_owned()]),
        ..Default::default()
    };
    assert_eq!(Config::from_toml(toml), expected);
}

#[test]
fn test_from_toml_without_roles() {
    let toml = r#"
apikey = "example_apikey"
apibase = "https://example.com"
"#;
    let expected = Config {
        apibase: "https://example.com".to_owned(),
        apikey: "example_apikey".to_owned(),
        ..Default::default()
    };
    assert_eq!(Config::from_toml(toml), expected);
}

#[test]
fn test_from_toml_with_display_name() {
    let toml = r#"
apikey = "example_apikey"
display_name = "host"
"#;
    let expected = Config {
        apikey: "example_apikey".to_owned(),
        display_name: Some("host".to_owned()),
        ..Default::default()
    };
    assert_eq!(Config::from_toml(toml), expected);
}
