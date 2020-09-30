extern crate mackerel_agent;

use clap::{load_yaml, App};
use mackerel_agent::{config::Config, Agent};
use mackerel_client::client::Client;
use std::{fs::File, io::prelude::*, path::Path};

// const HOST_PATH: &str = "/var/lib/mackerel-agent";
// TODO: change path as /var/lib/mackerel-agent/id
const HOST_ID_PATH: &str = "./id";

// Register the running host or get host own id.
async fn initialize(client: &Client, conf: &Config) -> std::io::Result<String> {
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
        let meta = mackerel_agent::host_meta::collect_as_json();
        let param = mackerel_client::create_host_param!({
            name -> format!("{}.rs", hostname) // TODO .rs を付けてるのはは暫定的
            meta -> meta
            role_fullnames -> conf.roles.clone()
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
    let conf = dbg!(Config::from_file(path));
    let client = Client::new(&conf.apikey);
    let host_id = initialize(&client, &conf).await;
    if host_id.is_err() {
        todo!()
    }
    let agent = Agent::new(conf, host_id.unwrap());
    agent.run().await;
    Ok(())
}
