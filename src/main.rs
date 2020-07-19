use clap::{load_yaml, App};
use ini::Ini;
use std::{collections::HashMap, fs::File, io::prelude::*, path::Path};

#[derive(Debug)]
struct Config {
    api_key: String,
    apibase: String,
}

impl Config {
    fn new() -> Self {
        Self {
            api_key: String::new(),
            apibase: String::new(),
        }
    }

    fn from_ini(ini: ini::Ini) -> Self {
        let mut conf = Self::new();
        let map = &ini
            .iter()
            .map(|(_, val)| val.iter().collect::<HashMap<_, _>>())
            .collect::<Vec<_>>()[0];
        conf.api_key = map.get("apikey").unwrap().to_string();
        conf.apibase = map.get("apibase").unwrap().to_string();
        conf
    }
}

fn main() -> std::io::Result<()> {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let path = Path::new(
        matches
            .value_of("config")
            .unwrap_or("/usr/local/etc/mackerel-agent.conf"),
    );
    let ini = Ini::load_from_file(path).unwrap();
    dbg!(Config::from_ini(ini));
    Ok(())
}
