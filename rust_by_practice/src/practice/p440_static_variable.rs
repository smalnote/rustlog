use std::{cell::RefCell, sync::Mutex};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum LogLevel {
    Debug,
    Error,
    Fatal,
}

struct Logger();

impl Logger {
    fn debug(&self, msg: &str) {
        if *LOG_LEVEL.lock().unwrap().borrow() as u32 > LogLevel::Debug as u32 {
            return;
        }
        println!("debug: {}", msg);
    }
    fn error(&self, msg: &str) {
        if *LOG_LEVEL.lock().unwrap().borrow() as u32 > LogLevel::Error as u32 {
            return;
        }
        println!("error: {}", msg);
    }
}

/// static variable is evaluated at runtime
pub static LOG_LEVEL: Mutex<RefCell<LogLevel>> = Mutex::new(RefCell::new(LogLevel::Debug));

/// static variables initialized at first use
use lazy_static::lazy_static;
lazy_static! {
    static ref LOGGER: Mutex<Logger> = Mutex::new(Logger());
}

/// constant variable is evaluated at compile time
/// let var = something is just a immutable variable
pub const SILENT_LEVEL: LogLevel = LogLevel::Fatal;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_static_log_level() {
        let logger = LOGGER.lock().unwrap();
        logger.debug("hello, logger!");
        logger.error("something wrong!");

        {
            *LOG_LEVEL.lock().unwrap().borrow_mut() = LogLevel::Error;
        }

        logger.debug("hello, debugger!");

        {
            *LOG_LEVEL.lock().unwrap().borrow_mut() = SILENT_LEVEL;
        }
        logger.debug("hello, logger!");
        logger.error("something wrong!");
    }
}
