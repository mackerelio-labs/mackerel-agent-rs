use clap::{load_yaml, App};
use ini::Ini;
use mackerel_client::client::Client;
use os_stat_rs::cpu;
use std::{collections::HashMap, fs::File, io::prelude::*, path::Path, time::Duration};
use tokio::time;

const HOST_PATH: &'static str = "/var/lib/mackerel-agent";
const HOST_ID_PATH: &'static str = "./id";

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
        conf.apibase = map
            .get("apibase")
            .unwrap_or(&"https://api.mackerelio.com/")
            .to_string();
        conf
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let path = Path::new(
        matches
            .value_of("config")
            .unwrap_or("/src/mackerel-agent.conf"),
    );
    let ini = Ini::load_from_file(path).unwrap();
    let conf = dbg!(Config::from_ini(ini));
    let client = Client::new(&conf.api_key);
    // if !Path::exists(Path::new(HOST_PATH)) {
    //     std::fs::create_dir(HOST_PATH)?;
    // }
    let _host_id = if let Ok(file) = File::open(HOST_ID_PATH) {
        let mut file = file;
        let mut buf = String::new();
        if file.read_to_string(&mut buf).is_err() {
            unimplemented!()
        }
        buf
    } else {
        let param = mackerel_client::create_host_param!({
            // TODO: Get hostname
            name -> "TEST".to_string()
        });
        dbg!("AAA");
        let result = client.create_host(param).await;
        dbg!(&result);
        if result.is_err() {
            unimplemented!();
        }
        let registerd_host_id = result.unwrap();
        let mut file = File::create(HOST_ID_PATH)?;
        file.write(registerd_host_id.as_bytes())?;
        registerd_host_id
    };

    let mut interval = time::interval(Duration::from_secs(1));
    loop {
        dbg!(interval.tick().await);
        dbg!(cpu::get());
    }
}
