use std::fs;

use crate::defs;

pub mod data;

impl data::ConfigData {
    pub fn load_config(&self) -> Self {
        if let Err(e) = fs::read_to_string(defs::CONFIG_PATH) {
            log::error!("无法读取配置文件:{},请检查{}", e, defs::CONFIG_PATH);
        }
        let content = fs::read_to_string(defs::CONFIG_PATH).unwrap();
        let toml: Self = toml::from_str(content.as_str()).unwrap();
        toml
    }
}
