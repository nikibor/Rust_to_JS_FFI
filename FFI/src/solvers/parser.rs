use std::fs;

use anyhow::Ok;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ConfigKind {
    #[serde(rename = "Config")]
    Congig,
    #[serde(rename = "Rule")]
    Rule,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cluster {
    #[serde(rename = "proxy_url")]
    proxy_url: String,
    #[serde(rename = "server")]
    server: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "apiVersion")]
    api_version: String,
    #[serde(rename = "kind")]
    kind: ConfigKind,
    #[serde(rename = "clusters")]
    clusters: Vec<Cluster>,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "users")]
    users: Vec<String>,
}

pub struct ConfigParser {
    pub path_dir: String,
}

impl ConfigParser {
    pub fn read_config(&self, file_path: String) -> anyhow::Result<Config> {
        let data = fs::read_to_string(file_path)?;
        let config: Config = serde_json::from_str(&data)?;
        Ok(config)
    }
}

#[cfg(test)]
pub mod tests{

    #[test]
    fn read_config_test(){
        use crate::solvers::parser::ConfigParser;

        let path = String::from("test_configs/config_0.json");
        let parser = ConfigParser{
            path_dir: path,
        };

        let config = parser.read_config("../test_configs/config_0.json".to_string()).unwrap();
        print!("{config:?}");
        assert!(true)
    }
}