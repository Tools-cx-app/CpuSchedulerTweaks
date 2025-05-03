use std::{
    process::Command,
    time::{Duration, Instant},
};

use regex::Regex;

const RESET_TIME: Duration = Duration::from_secs(1);

#[derive(Debug, Clone)]
pub struct TopAppWatch {
    time: Instant,
    topapps: String,
}

impl TopAppWatch {
    pub fn new() -> Self {
        Self {
            time: Instant::now(),
            topapps: String::new(),
        }
    }

    pub fn dump(&mut self) {
        if self.time.elapsed() > RESET_TIME {
            let output = loop {
                match Command::new("/system/bin/dumpsys")
                    .args(["activity", "lru"])
                    .output()
                {
                    Ok(o) => {
                        break String::from_utf8_lossy(&o.stdout).to_string();
                    }
                    Err(e) => {
                        log::error!("无法获取前台应用:{e}");
                        std::thread::sleep(Duration::from_secs(1));
                    }
                }
            };

            let re =
                Regex::new(r"  TOP  .*?([a-zA-Z][a-zA-Z0-9_]*(\.[a-zA-Z][a-zA-Z0-9_]*)+)").unwrap();
            for line in output.lines() {
                if let Some(caps) = re.captures(line) {
                    self.topapps = caps[1].to_string();
                    continue;
                }
            }

            self.time = Instant::now();
        }
    }

    pub fn get(&self) -> String {
        self.topapps.clone()
    }
}
