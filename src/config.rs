use std::sync::RwLock;
use std::fs;
use std::io::prelude::*;
use serde::{Serialize, Deserialize, serde_if_integer128};
use serde;
use std::env;

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Cfg {
    pub ws_root_path: String,
}

impl Cfg {
    pub fn init(&mut self) -> Result<(), i32> {
        // get config path
        let conf_file = self.getPathToConfig();

        let _lcfg: Cfg =
            toml::from_slice(&fs::read(conf_file).unwrap()).unwrap();

        self.ws_root_path = _lcfg.ws_root_path;

        Ok(())
    }
    pub fn save(&self) -> Result<(), i32> {
        // save current state to file

        let conf_file = self.getPathToConfig();
        // let c = CONFIG.read().unwrap();
        &fs::write(conf_file, toml::to_string(&self).unwrap()).unwrap();
        Ok(())
    }

    fn getPathToConfig(&self) -> String {
        // get debug or release path
        let mut config_path = "".to_string();
        match env::var("XDG_CONFIG_HOME") {
            Ok(v) => config_path = v.clone(),
            Err(e) => config_path = "./config".to_string()
        }

        // add config filename
        config_path += "/settings.toml";

        /* read the config file */
        println!("{}",config_path);
        return config_path
    }
}
