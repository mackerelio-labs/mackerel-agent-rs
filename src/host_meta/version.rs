use compile_time_run::run_command_str;

pub struct Version {
    pub revision: String,
    pub version: String,
}

impl Version {
    pub fn new() -> Version {
        Version {
            revision: run_command_str!("git", "rev-parse", "--short", "HEAD").to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
        }
    }
}
