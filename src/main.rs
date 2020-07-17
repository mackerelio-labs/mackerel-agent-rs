use clap::{load_yaml, App};
use ini::Ini;
use std::{fs::File, io::prelude::*, path::Path};

fn main() -> std::io::Result<()> {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let path = Path::new(
        matches
            .value_of("config")
            .unwrap_or("/usr/local/etc/mackerel-agent.conf"),
    );
    let conf = Ini::load_from_file(path).unwrap();
    for (key, val) in &conf {
        dbg!((key, val));
    }
    Ok(())
}
