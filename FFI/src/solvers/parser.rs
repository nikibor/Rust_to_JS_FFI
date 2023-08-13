use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config{
    
}

pub struct ConfigParser{
    path_dir: String,
}

impl ConfigParser{
    fn read_config(&self, path: String) -> anyhow::Result<()>{

        Ok(())
    }
}