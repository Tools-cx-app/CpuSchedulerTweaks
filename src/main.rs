mod defs;
mod framework;

use std::fs;

use anyhow::{Context, Result};

fn check() -> Result<()> {
    let procs = procfs::process::all_processes().context("无法获取进程列表")?;
    let self_name = fs::read_to_string("/proc/self/comm")?;

    for proc in procs {
        let proc = match proc {
            Ok(p) => p,
            Err(e) => {
                log::error!("无法获取进程信息:{}", e);
                break;
            }
        };
        if let Ok(states) = proc.status() {
            if states.name.contains(self_name.trim()) && states.pid != std::process::id() as i32 {
                log::error!("发现其他进程,pid:{},即将退出", states.pid);
                std::process::exit(1);
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    simple_logger::SimpleLogger::new()
        .with_local_timestamps()
        .with_level(log::LevelFilter::Debug)
        .init()?;
    check()?;
    log::info!("life-death-scheduler v{}", defs::VERSION);
    log::info!("life-death-scheduler正在启动");
    let mut framework = crate::framework::scheduler::Looper::new();
    framework.init();
    framework.enter_looper()?;
    Ok(())
}
