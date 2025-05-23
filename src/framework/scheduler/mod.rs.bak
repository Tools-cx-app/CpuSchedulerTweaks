mod cpu;
mod dump;

use std::{fs::write, process::Command};

use anyhow::Result;
use cpu::Cpu;
use dump::topapps::TopAppWatch;
use glob::glob;
use inotify::{Inotify, WatchMask};

use super::config::data::ConfigData;
use crate::defs;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Powersave,
    Balance,
    Performance,
    Fast,
}

pub struct Looper {
    cpu: Cpu,
    mode: Mode,
    config: ConfigData,
    topapp: TopAppWatch,
}

impl Looper {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            mode: Mode::Balance,
            config: ConfigData::new(),
            topapp: TopAppWatch::new(),
        }
    }

    pub fn init(&self) {
        let commands = vec![
            ("stop", vec!["miuibooster"]),
            ("stop", vec!["oneplus_brain_service"]),
            ("stop", vec!["vendor.perfservice"]),
            ("stop", vec!["perfd"]),
            (
                "stop",
                vec!["orms-hal-1-0", "vendor.oplus.ormsHalService-aidl-default"],
            ),
            ("setprop", vec!["persist.sys.hardcoder.name", ""]),
            ("setprop", vec!["persist.miui.miperf.enable", "false"]),
        ];

        for i in defs::BOOST_PATHS {
            for path in glob(i).unwrap() {
                let path = path.unwrap();
                let _ = write(&path, "0");
                log::debug!("{}", path.display());
            }
        }
        let _ = write(
            "/sys/devices/system/cpu/cpufreq/hotplug/cpu_hotplug_disable",
            "1",
        );
        for path in glob("/sys/module/control_center/parameters/*").unwrap() {
            let path = path.unwrap();
            let _ = write(&path, "N");
            log::debug!("{}", path.display());
        }

        for (command, args) in commands {
            let _ = Command::new(command).args(args).output();
        }
    }

    pub fn enter_looper(&mut self) -> Result<()> {
        let mut inotify = Inotify::init()?;
        inotify.watches().add("/dev/input", WatchMask::ACCESS)?;

        loop {
            inotify.read_events_blocking(&mut [0; 1024])?;
            self.config.load_config();
            self.cpu.load_config(self.config.clone());
            self.topapp.dump();

            #[cfg(debug_assertions)]
            {
                log::debug!("Current topapp: {}", self.topapp.get());
                log::debug!("Current mode: {:?}", self.mode);
                log::debug!("Current config: {:?}", self.config);
            }

            for (app, mode) in self.config.applist.clone() {
                if self.topapp.get() == app {
                    log::info!("正在为{app}配置{mode}模式");
                    self.cpu.set_freq(self.switch_mode(mode.as_str()));
                } else {
                    self.cpu
                        .set_freq(self.switch_mode(self.config.osm.as_str()));
                }
            }
        }
    }

    fn switch_mode(&self, mode: &str) -> Mode {
        match mode {
            "powersave" => Mode::Powersave,
            "balance" => Mode::Balance,
            "performance" => Mode::Performance,
            "fast" => Mode::Fast,
            _ => {
                log::error!("配置文件错误,模式默认使用balance");
                Mode::Balance
            }
        }
    }
}
