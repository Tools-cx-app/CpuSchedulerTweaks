use std::{collections::HashMap, fs};

use data::CpuFreq;

use crate::defs;

pub mod data;

impl data::ConfigData {
    pub fn new() -> Self {
        let s = Self {
            powersave: CpuFreq {
                big_cpu_freq: 0,
                middle_cpu_freq: 0,
                small_cpu_freq: 0,
            },
            balance: CpuFreq {
                big_cpu_freq: 0,
                middle_cpu_freq: 0,
                small_cpu_freq: 0,
            },
            performance: CpuFreq {
                big_cpu_freq: 0,
                middle_cpu_freq: 0,
                small_cpu_freq: 0,
            },
            fast: CpuFreq {
                big_cpu_freq: 0,
                middle_cpu_freq: 0,
                small_cpu_freq: 0,
            },
            cpu_config: data::Cpu {
                big: 0,
                middle: 0,
                small: 0,
            },
            osm: String::new(),
            applist: HashMap::new(),
        };
        s.load_config()
    }

    pub fn load_config(&self) -> Self {
        if let Err(e) = fs::read_to_string(defs::CONFIG_PATH) {
            log::error!("无法读取配置文件:{},请检查{}", e, defs::CONFIG_PATH);
        }
        let content = fs::read_to_string(defs::CONFIG_PATH).unwrap();
        let toml: Self = toml::from_str(content.as_str()).unwrap();
        toml
    }
}
