use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ConfigKind{
    Congig,
    Rule,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cluster{
    proxy_url: String,
    server: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    api_version: String,
    kind: ConfigKind,
    clusters: Vec<Cluster>,
    name: String,
    users: Vec<String>
}

pub struct ConfigParser {
    path_dir: String,
}

impl ConfigParser {
    fn read_config(&self, path: String) -> anyhow::Result<()> {
        Ok(())
    }
}
