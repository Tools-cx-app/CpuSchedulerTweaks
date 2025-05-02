#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Powersave,
    Balance,
    Performance,
    Fast,
}

pub struct Looper {
    mode: Mode,
}

impl Looper {
    pub fn new() -> Self {
        Self {
            mode: Mode::Balance,
        }
    }

    pub fn init(&mut self) {
        log::debug!("{:?}", self.mode);
    }
}
