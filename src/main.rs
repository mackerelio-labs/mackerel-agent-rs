extern crate mackerel_agent_rs;

use clap::{load_yaml, App};
use ini::Ini;
use mackerel_agent_rs::{Agent, Config};
use mackerel_client::client::Client;
use std::{fs::File, io::prelude::*, path::Path};

// const HOST_PATH: &str = "/var/lib/mackerel-agent";
// TODO: change path as /var/lib/mackerel-agent/id
const HOST_ID_PATH: &str = "./id";

// Register the running host or get host own id.
async fn initialize(client: &Client) -> std::io::Result<String> {
    // if !Path::exists(Path::new(HOST_PATH)) {
    //     std::fs::create_dir(HOST_PATH)?;
    // }
    Ok(if let Ok(file) = File::open(HOST_ID_PATH) {
        let mut file = file;
        let mut buf = String::new();
        if file.read_to_string(&mut buf).is_err() {
            unimplemented!()
        }
        buf
    } else {
        let hostname = hostname::get();
        if hostname.is_err() {
            todo!();
        }
        let hostname = hostname.unwrap().to_str().unwrap().to_owned();
        let meta = mackerel_agent_rs::host_meta::collect_as_json();
        let param = mackerel_client::create_host_param!({
            name -> format!("{}.rs", hostname)
            meta -> meta
        });
        let result = client.create_host(param).await;
        if result.is_err() {
            unimplemented!();
        }
        let registerd_host_id = result.unwrap();
        let mut file = File::create(HOST_ID_PATH)?;
        file.write_all(registerd_host_id.as_bytes())?;
        registerd_host_id
    })
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
    let host_id = initialize(&client).await;
    if host_id.is_err() {
        todo!()
    }
    let agent = Agent::new(conf, host_id.unwrap());
    agent.run().await;
    Ok(())
}
