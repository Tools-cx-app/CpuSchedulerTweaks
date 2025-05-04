use std::{
    fs::{Permissions, set_permissions, write},
    os::unix::fs::PermissionsExt,
    path::Path,
};

use anyhow::{Context, Result};

use crate::framework::config::data::ConfigData;

use super::Mode;

pub struct Cpu {
    config: ConfigData,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            config: ConfigData::new(),
        }
    }
    pub fn load_config(&mut self, config: ConfigData) {
        self.config = config;
    }

    pub fn set_freq(&self, mode: Mode) {
        let big_path = format!(
            "/sys/devices/system/cpu/cpufreq/policy{}",
            self.config.cpu_config.big
        );
        let big = Path::new(big_path.as_str());

        let middle_path = format!(
            "/sys/devices/system/cpu/cpufreq/policy{}",
            self.config.cpu_config.middle
        );
        let middle = Path::new(middle_path.as_str());

        let small_path = format!(
            "/sys/devices/system/cpu/cpufreq/policy{}",
            self.config.cpu_config.small
        );
        let small = Path::new(small_path.as_str());

        if !big.exists() {
            log::error!("CPU簇{}不存在", self.config.cpu_config.big);
            return;
        }
        if !middle.exists() {
            log::error!("CPU簇{}不存在", self.config.cpu_config.middle);
            return;
        }
        if !small.exists() {
            log::error!("CPU簇{}不存在", self.config.cpu_config.small);
            return;
        }

        #[cfg(debug_assertions)]
        {
            log::debug!("{}簇:{}", self.config.cpu_config.big, big.display());
            log::debug!("{}簇:{}", self.config.cpu_config.middle, middle.display());
            log::debug!("{}簇:{}", self.config.cpu_config.small, small.display());
        }

        let mut big_freq: u64 = 0;
        let mut middle_freq: u64 = 0;
        let mut small_freq: u64 = 0;
        match mode {
            Mode::Powersave => {
                big_freq = self.config.powersave.big_cpu_freq;
                middle_freq = self.config.powersave.middle_cpu_freq;
                small_freq = self.config.powersave.small_cpu_freq;
            }
            Mode::Balance => {
                big_freq = self.config.balance.big_cpu_freq;
                middle_freq = self.config.balance.middle_cpu_freq;
                small_freq = self.config.balance.small_cpu_freq;
            }
            Mode::Performance => {
                big_freq = self.config.performance.big_cpu_freq;
                middle_freq = self.config.performance.middle_cpu_freq;
                small_freq = self.config.performance.small_cpu_freq;
            }
            Mode::Fast => {
                big_freq = self.config.fast.big_cpu_freq;
                middle_freq = self.config.fast.middle_cpu_freq;
                small_freq = self.config.fast.small_cpu_freq;
            }
        }
        let _ = self.write_freq(big, big_freq);
        let _ = self.write_freq(middle, middle_freq);
        let _ = self.write_freq(small, small_freq);
    }

    fn write_freq(&self, path: &Path, freq: u64) -> Result<()> {
        set_permissions(path, Permissions::from_mode(0o644)).context("无法设置权限")?;
        write(path, freq.to_string()).context("无法写入文件")?;
        set_permissions(path, Permissions::from_mode(0o400)).context("无法设置权限")?;
        Ok(())
    }
}
