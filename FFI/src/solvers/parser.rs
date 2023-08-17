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
    pub api_version: String,
    #[serde(rename = "kind")]
    pub kind: ConfigKind,
    #[serde(rename = "clusters")]
    pub clusters: Vec<Cluster>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "users")]
    pub users: Vec<String>,
}

pub struct ConfigParser {
    pub path_dir: String,
}

impl ConfigParser {
    pub fn read_config(&self, file_path: &str) -> anyhow::Result<Config> {
        let data = fs::read_to_string(file_path)?;
        let config: Config = serde_json::from_str(&data)?;
        Ok(config)
    }
}

#[cfg(test)]
pub mod tests {

    use crate::solvers::parser::ConfigParser;
    #[test]
    fn read_config_test() {
        let parser = ConfigParser {
            path_dir: String::from("../test_configs"),
        };
        let config = parser.read_config("../test_configs/config_0.json").unwrap();

        assert!(!config.name.is_empty())
    }
}
