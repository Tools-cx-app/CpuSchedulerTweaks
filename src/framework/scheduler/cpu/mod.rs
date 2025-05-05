#![allow(unused_assignments)]
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

        let super_big_path = format!(
            "/sys/devices/system/cpu/cpufreq/policy{}",
            self.config.cpu_config.super_big
        );
        let super_big = Path::new(super_big_path.as_str());
        let mut has_super_big = false;
        let mut has_small_big = false;

        if !big.exists() {
            log::error!("CPU簇{}不存在", self.config.cpu_config.big);
            return;
        }
        if !middle.exists() {
            log::error!("CPU簇{}不存在", self.config.cpu_config.middle);
            return;
        }
        has_super_big = super_big.exists();
        has_small_big = small.exists();

        #[cfg(debug_assertions)]
        {
            for (name, path) in [
                ("big", big),
                ("middle", middle),
                ("small", small),
                ("super_big", super_big),
            ] {
                log::debug!("{}簇:{}", name, path.display());
            }
        }

        let mut big_freq = Vec::new();
        let mut middle_freq = Vec::new();
        let mut small_freq = Vec::new();
        let mut super_big_freq = Vec::new();
        match mode {
            Mode::Powersave => {
                big_freq.insert(0, self.config.powersave.big_cpu_freq.max);
                big_freq.insert(1, self.config.powersave.big_cpu_freq.min);
                middle_freq.insert(0, self.config.powersave.middle_cpu_freq.max);
                middle_freq.insert(1, self.config.powersave.middle_cpu_freq.min);
                if has_small_big {
                    small_freq.insert(0, self.config.powersave.small_cpu_freq.max);
                    small_freq.insert(1, self.config.powersave.small_cpu_freq.min);
                }
                if has_super_big {
                    super_big_freq.insert(0, self.config.powersave.super_big_cpu_freq.max);
                    super_big_freq.insert(1, self.config.powersave.super_big_cpu_freq.min);
                }
            }
            Mode::Balance => {
                big_freq.insert(0, self.config.balance.big_cpu_freq.max);
                big_freq.insert(1, self.config.balance.big_cpu_freq.min);
                middle_freq.insert(0, self.config.balance.middle_cpu_freq.max);
                middle_freq.insert(1, self.config.balance.middle_cpu_freq.min);
                small_freq.insert(0, self.config.balance.small_cpu_freq.max);
                small_freq.insert(1, self.config.balance.small_cpu_freq.min);
                if has_small_big {
                    small_freq.insert(0, self.config.balance.small_cpu_freq.max);
                    small_freq.insert(1, self.config.balance.small_cpu_freq.min);
                }
                if has_super_big {
                    super_big_freq.insert(0, self.config.balance.super_big_cpu_freq.max);
                    super_big_freq.insert(1, self.config.balance.super_big_cpu_freq.min);
                }
            }
            Mode::Performance => {
                big_freq.insert(0, self.config.performance.big_cpu_freq.max);
                big_freq.insert(1, self.config.performance.big_cpu_freq.min);
                middle_freq.insert(0, self.config.performance.middle_cpu_freq.max);
                middle_freq.insert(1, self.config.performance.middle_cpu_freq.min);
                small_freq.insert(0, self.config.performance.small_cpu_freq.max);
                small_freq.insert(1, self.config.performance.small_cpu_freq.min);
                if has_small_big {
                    small_freq.insert(0, self.config.performance.small_cpu_freq.max);
                    small_freq.insert(1, self.config.performance.small_cpu_freq.min);
                }
                if has_super_big {
                    super_big_freq.insert(0, self.config.performance.super_big_cpu_freq.max);
                    super_big_freq.insert(1, self.config.performance.super_big_cpu_freq.min);
                }
            }
            Mode::Fast => {
                big_freq.insert(0, self.config.fast.big_cpu_freq.max);
                big_freq.insert(1, self.config.fast.big_cpu_freq.min);
                middle_freq.insert(0, self.config.fast.middle_cpu_freq.max);
                middle_freq.insert(1, self.config.fast.middle_cpu_freq.min);
                small_freq.insert(0, self.config.fast.small_cpu_freq.max);
                small_freq.insert(1, self.config.fast.small_cpu_freq.min);
                if has_small_big {
                    small_freq.insert(0, self.config.fast.small_cpu_freq.max);
                    small_freq.insert(1, self.config.fast.small_cpu_freq.min);
                }
                if has_super_big {
                    super_big_freq.insert(0, self.config.fast.super_big_cpu_freq.max);
                    super_big_freq.insert(1, self.config.fast.super_big_cpu_freq.min);
                }
            }
        }
        let _ = self.write_freq(big, big_freq);
        let _ = self.write_freq(middle, middle_freq);
        if has_small_big {
            let _ = self.write_freq(small, small_freq);
        }
        if has_super_big {
            let _ = self.write_freq(super_big, super_big_freq);
        }
    }

    fn write_freq(&self, path: &Path, freq: Vec<u64>) -> Result<()> {
        let max = &path.join("/sys/devices/system/cpu/cpufreq/policy4/scaling_max_freq");
        let min = &path.join("/sys/devices/system/cpu/cpufreq/policy4/scaling_min_freq");
        set_permissions(max, Permissions::from_mode(0o644)).context("无法设置权限")?;
        write(max, freq[0].to_string()).context("无法写入文件")?;
        set_permissions(max, Permissions::from_mode(0o400)).context("无法设置权限")?;
        set_permissions(min, Permissions::from_mode(0o644)).context("无法设置权限")?;
        write(min, freq[1].to_string()).context("无法写入文件")?;
        set_permissions(min, Permissions::from_mode(0o400)).context("无法设置权限")?;
        Ok(())
    }
}
