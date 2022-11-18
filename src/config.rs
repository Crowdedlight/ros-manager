use std::sync::RwLock;
use std::fs;
use std::io::prelude::*;
use serde::{Serialize, Deserialize, serde_if_integer128};
use serde;

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Cfg {
    pub ws_root_path: String,
}

impl Cfg {
    pub fn init(&mut self) -> Result<(), i32> {
        // let mut w = CONFIG.write().unwrap();

        /* read the config file */
        let _lcfg: Cfg =
            toml::from_slice(&fs::read("./src/settings.toml").unwrap()).unwrap();

        self.ws_root_path = _lcfg.ws_root_path;

        Ok(())
    }
    pub fn save(&self) -> Result<(), i32> {
        // save current state to file
        // let c = CONFIG.read().unwrap();
        &fs::write("./src/settings.toml", toml::to_string(&self).unwrap()).unwrap();
        Ok(())
    }
}