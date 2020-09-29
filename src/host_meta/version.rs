pub struct Version {
    pub version: String,
}

impl Version {
    pub fn new() -> Version {
        Version {
            version: env!("CARGO_PKG_VERSION").to_owned(),
        }
    }
}
