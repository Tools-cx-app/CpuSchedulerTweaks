#![allow(unused_assignments)]
use std::{
    fs::{Permissions, set_permissions, write},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
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

    /*
    * cpu频率设置，mode为当前的模式，自动根据配置文件设置
    * 2025-05-24
    */
    pub fn set_freq(&self, mode: Mode) {
        let big_path = PathBuf::from(format!(
            "/sys/devices/system/cpu/cpufreq/policy{}",
            self.config.cpu_config.big
        ));
        let big = big_path.as_path();

        let middle_path = PathBuf::from(format!(
            "/sys/devices/system/cpu/cpufreq/policy{}",
            self.config.cpu_config.middle
        ));
        let middle = middle_path.as_path();

        let small_path = self
            .config
            .cpu_config
            .small
            .map(|id| PathBuf::from(format!("/sys/devices/system/cpu/cpufreq/policy{}", id)));
        let small = small_path.as_ref().map(|p| p.as_path());

        let super_big_path = self
            .config
            .cpu_config
            .super_big
            .map(|id| PathBuf::from(format!("/sys/devices/system/cpu/cpufreq/policy{}", id)));
        let super_big = super_big_path.as_ref().map(|p| p.as_path());

        if !big.exists() {
            log::error!("CPU簇{}不存在", self.config.cpu_config.big);
            return;
        }
        if !middle.exists() {
            log::error!("CPU簇{}不存在", self.config.cpu_config.middle);
            return;
        }
        let has_small_big = small.map(|p| p.exists()).unwrap_or(false);
        let has_super_big = super_big.map(|p| p.exists()).unwrap_or(false);

        #[cfg(debug_assertions)]
        {
            log::debug!("big簇: {}", big.display());
            log::debug!("middle簇: {}", middle.display());
            if let Some(s) = small {
                log::debug!("small簇: {}", s.display());
            }
            if let Some(sb) = super_big {
                log::debug!("super_big簇: {}", sb.display());
            }
        }

        let mut big_freq = Vec::new();
        let mut middle_freq = Vec::new();
        let mut small_freq = Vec::new();
        let mut super_big_freq = Vec::new();

        /*
        * 自动判断，并按顺序写入频率变量
        * 2025-05-24
        */
        match mode {
            Mode::Powersave => {
                big_freq.extend(&[
                    self.config.powersave.big_cpu_freq.max,
                    self.config.powersave.big_cpu_freq.min,
                ]);
                middle_freq.extend(&[
                    self.config.powersave.middle_cpu_freq.max,
                    self.config.powersave.middle_cpu_freq.min,
                ]);
                if has_small_big {
                    small_freq.extend(&[
                        self.config.powersave.small_cpu_freq.max,
                        self.config.powersave.small_cpu_freq.min,
                    ]);
                }
                if has_super_big {
                    super_big_freq.extend(&[
                        self.config.powersave.super_big_cpu_freq.max,
                        self.config.powersave.super_big_cpu_freq.min,
                    ]);
                }
            }
            Mode::Balance => {
                big_freq.extend(&[
                    self.config.balance.big_cpu_freq.max,
                    self.config.balance.big_cpu_freq.min,
                ]);
                middle_freq.extend(&[
                    self.config.balance.middle_cpu_freq.max,
                    self.config.balance.middle_cpu_freq.min,
                ]);
                if has_small_big {
                    small_freq.extend(&[
                        self.config.balance.small_cpu_freq.max,
                        self.config.balance.small_cpu_freq.min,
                    ]);
                }
                if has_super_big {
                    super_big_freq.extend(&[
                        self.config.balance.super_big_cpu_freq.max,
                        self.config.balance.super_big_cpu_freq.min,
                    ]);
                }
            }
            Mode::Performance => {
                big_freq.extend(&[
                    self.config.performance.big_cpu_freq.max,
                    self.config.performance.big_cpu_freq.min,
                ]);
                middle_freq.extend(&[
                    self.config.performance.middle_cpu_freq.max,
                    self.config.performance.middle_cpu_freq.min,
                ]);
                if has_small_big {
                    small_freq.extend(&[
                        self.config.performance.small_cpu_freq.max,
                        self.config.performance.small_cpu_freq.min,
                    ]);
                }
                if has_super_big {
                    super_big_freq.extend(&[
                        self.config.performance.super_big_cpu_freq.max,
                        self.config.performance.super_big_cpu_freq.min,
                    ]);
                }
            }
            Mode::Fast => {
                big_freq.extend(&[
                    self.config.fast.big_cpu_freq.max,
                    self.config.fast.big_cpu_freq.min,
                ]);
                middle_freq.extend(&[
                    self.config.fast.middle_cpu_freq.max,
                    self.config.fast.middle_cpu_freq.min,
                ]);
                if has_small_big {
                    small_freq.extend(&[
                        self.config.fast.small_cpu_freq.max,
                        self.config.fast.small_cpu_freq.min,
                    ]);
                }
                if has_super_big {
                    super_big_freq.extend(&[
                        self.config.fast.super_big_cpu_freq.max,
                        self.config.fast.super_big_cpu_freq.min,
                    ]);
                }
            }
        }

        let _ = self.write_freq(big, big_freq);
        let _ = self.write_freq(middle, middle_freq);
        if has_small_big {
            if let Some(s) = small {
                let _ = self.write_freq(s, small_freq);
            }
        }
        if has_super_big {
            if let Some(sb) = super_big {
                let _ = self.write_freq(sb, super_big_freq);
            }
        }
    }

    /*
     * 频率写入部分
     * 2025-05-24
     */
    fn write_freq(&self, path: &Path, freq: Vec<u64>) -> Result<()> {
        let max = path.join("/scaling_max_freq");
        let min = path.join("/scaling_min_freq");

        if freq.len() < 2 {
            anyhow::bail!("无效的频率参数，需要最大和最小频率");
        }

        set_permissions(&max, Permissions::from_mode(0o644)).context("无法设置最大频率权限")?;
        write(&max, freq[0].to_string()).context("无法写入最大频率")?;
        set_permissions(&max, Permissions::from_mode(0o400)).context("无法恢复最大频率权限")?;

        set_permissions(&min, Permissions::from_mode(0o644)).context("无法设置最小频率权限")?;
        write(&min, freq[1].to_string()).context("无法写入最小频率")?;
        set_permissions(&min, Permissions::from_mode(0o400)).context("无法恢复最小频率权限")?;

        Ok(())
    }
}

