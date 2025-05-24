use std::{
    fs::{Permissions, set_permissions, write},
    os::unix::fs::PermissionsExt,
};

use anyhow::{Context, Result};

use crate::{defs::TOP_APP_CPUCTL, framework::config::data::ConfigData};

use super::Mode;

pub struct CpuCtl {
    config: ConfigData,
}

impl CpuCtl {
    pub fn new() -> Self {
        Self {
            config: ConfigData::new(),
        }
    }

    /*
     * 配置文件加载
     * 2025-05-25
     */
    pub fn load_config(&mut self, config: ConfigData) {
        self.config = config;
    }

    /*
     * 设置uclamp
     * 2025-05-25
     */
    pub fn set_uclamp(&self, mode: Mode) -> Result<()> {
        let top_app_uclamp_max = TOP_APP_CPUCTL.join("cpu.uclamp.max");
        let top_app_uclamp_min = TOP_APP_CPUCTL.join("cpu.uclamp.min");
        let top_app_shares = TOP_APP_CPUCTL.join("cpu.shares");

        match mode {
            Mode::Powersave => {
                set_permissions(&top_app_uclamp_max, Permissions::from_mode(0o644))
                    .context("无法设置最大频率权限")?;
                write(
                    &top_app_uclamp_max,
                    self.config.powersave.cpuctl.top_app.uclamp.max.to_string(),
                )
                .context("无法写入最大频率")?;
                set_permissions(&top_app_uclamp_max, Permissions::from_mode(0o400))
                    .context("无法恢复最大频率权限")?;
                set_permissions(&top_app_uclamp_max, Permissions::from_mode(0o644))
                    .context("无法设置最大频率权限")?;
                write(
                    &top_app_uclamp_min,
                    self.config.powersave.cpuctl.top_app.uclamp.min.to_string(),
                )
                .context("无法写入最大频率")?;
                set_permissions(&top_app_uclamp_min, Permissions::from_mode(0o400))
                    .context("无法恢复最大频率权限")?;
                set_permissions(&top_app_shares, Permissions::from_mode(0o644))
                    .context("无法设置最大频率权限")?;
                write(&top_app_shares, self.config.powersave.cpuctl.top_app.shares.to_string())
                    .context("无法写入最大频率")?;
                set_permissions(&top_app_shares, Permissions::from_mode(0o400))
                    .context("无法恢复最大频率权限")?;
            }
            Mode::Balance => {
                set_permissions(&top_app_uclamp_max, Permissions::from_mode(0o644))
                    .context("无法设置最大频率权限")?;
                write(
                    &top_app_uclamp_max,
                    self.config.balance.cpuctl.top_app.uclamp.max.to_string(),
                )
                .context("无法写入最大频率")?;
                set_permissions(&top_app_uclamp_max, Permissions::from_mode(0o400))
                    .context("无法恢复最大频率权限")?;
                set_permissions(&top_app_uclamp_max, Permissions::from_mode(0o644))
                    .context("无法设置最大频率权限")?;
                write(
                    &top_app_uclamp_min,
                    self.config.balance.cpuctl.top_app.uclamp.min.to_string(),
                )
                .context("无法写入最大频率")?;
                set_permissions(&top_app_uclamp_min, Permissions::from_mode(0o400))
                    .context("无法恢复最大频率权限")?;
                set_permissions(&top_app_shares, Permissions::from_mode(0o644))
                    .context("无法设置最大频率权限")?;
                write(&top_app_shares, self.config.balance.cpuctl.top_app.shares.to_string())
                    .context("无法写入最大频率")?;
                set_permissions(&top_app_shares, Permissions::from_mode(0o400))
                    .context("无法恢复最大频率权限")?;
            }
            Mode::Performance => {
                set_permissions(&top_app_uclamp_max, Permissions::from_mode(0o644))
                    .context("无法设置最大频率权限")?;
                write(
                    &top_app_uclamp_max,
                    self.config.performance.cpuctl.top_app.uclamp.max.to_string(),
                )
                .context("无法写入最大频率")?;
                set_permissions(&top_app_uclamp_max, Permissions::from_mode(0o400))
                    .context("无法恢复最大频率权限")?;
                set_permissions(&top_app_uclamp_max, Permissions::from_mode(0o644))
                    .context("无法设置最大频率权限")?;
                write(
                    &top_app_uclamp_min,
                    self.config.performance.cpuctl.top_app.uclamp.min.to_string(),
                )
                .context("无法写入最大频率")?;
                set_permissions(&top_app_uclamp_min, Permissions::from_mode(0o400))
                    .context("无法恢复最大频率权限")?;
                set_permissions(&top_app_shares, Permissions::from_mode(0o644))
                    .context("无法设置最大频率权限")?;
                write(
                    &top_app_shares,
                    self.config.performance.cpuctl.top_app.shares.to_string(),
                )
                .context("无法写入最大频率")?;
                set_permissions(&top_app_shares, Permissions::from_mode(0o400))
                    .context("无法恢复最大频率权限")?;
            }
            Mode::Fast => {
                set_permissions(&top_app_uclamp_max, Permissions::from_mode(0o644))
                    .context("无法设置最大频率权限")?;
                write(
                    &top_app_uclamp_max,
                    self.config.fast.cpuctl.top_app.uclamp.max.to_string(),
                )
                .context("无法写入最大频率")?;
                set_permissions(&top_app_uclamp_max, Permissions::from_mode(0o400))
                    .context("无法恢复最大频率权限")?;
                set_permissions(&top_app_uclamp_max, Permissions::from_mode(0o644))
                    .context("无法设置最大频率权限")?;
                write(
                    &top_app_uclamp_min,
                    self.config.fast.cpuctl.top_app.uclamp.min.to_string(),
                )
                .context("无法写入最大频率")?;
                set_permissions(&top_app_uclamp_min, Permissions::from_mode(0o400))
                    .context("无法恢复最大频率权限")?;
                set_permissions(&top_app_shares, Permissions::from_mode(0o644))
                    .context("无法设置最大频率权限")?;
                write(
                    &top_app_shares,
                    self.config.fast.cpuctl.top_app.shares.to_string(),
                )
                .context("无法写入最大频率")?;
                set_permissions(&top_app_shares, Permissions::from_mode(0o400))
                    .context("无法恢复最大频率权限")?;
            }
        }

        Ok(())
    }
}
