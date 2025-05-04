use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigData {
    pub osm: String,
    pub cpu_config: Cpu,
    pub powersave: CpuFreq,
    pub balance: CpuFreq,
    pub performance: CpuFreq,
    pub fast: CpuFreq,
    pub applist: HashMap<String, String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CpuFreq {
    pub big_cpu_freq: u64,
    pub middle_cpu_freq: u64,
    pub small_cpu_freq: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Cpu {
    pub big: u16,
    pub middle: u16,
    pub small: u16,
}
