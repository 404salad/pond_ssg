use std::fmt::Display;
use std::sync::OnceLock;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Silent, // only error logs
    Normal, // everything
            // TODO: maybe add debug level
}

static LOG_LEVEL: OnceLock<LogLevel> = OnceLock::new();

pub fn set_log_level(level: LogLevel) {
    let _ = LOG_LEVEL.set(level); // ignore if already set
}

fn current_level() -> LogLevel {
    *LOG_LEVEL.get().unwrap_or(&LogLevel::Normal)
}

// Accepts anything that implements Display
pub fn log_info<T: Display>(msg: T) {
    if current_level() != LogLevel::Silent {
        println!("{msg}");
    }
}
