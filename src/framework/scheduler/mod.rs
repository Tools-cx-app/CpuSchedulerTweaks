mod dump;

use anyhow::Result;
use dump::topapps::TopAppWatch;
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
    mode: Mode,
    config: ConfigData,
    topapp: TopAppWatch,
}

impl Looper {
    pub fn new() -> Self {
        Self {
            mode: Mode::Balance,
            config: ConfigData::new(),
            topapp: TopAppWatch::new(),
        }
    }

    pub fn init(&mut self) -> Result<()> {
        let mut inotify = Inotify::init()?;
        inotify.watches().add("/dev/input", WatchMask::ACCESS)?;

        loop {
            inotify.read_events_blocking(&mut [0; 1024])?;
            self.config.load_config();
            self.topapp.dump();

            log::debug!("Current topapp: {}", self.topapp.get());
            log::debug!("Current mode: {:?}", self.mode);
            log::debug!("Current config: {:?}", self.config);
        }
    }
}
