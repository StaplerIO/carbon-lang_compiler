use console::style;

use crate::STDOUT;

pub enum LogLevel {
    Trace,
    Info,
    Warn,
    Error,
}

pub fn log_string(level: LogLevel, message: &str) {
    match level {
        LogLevel::Trace => STDOUT.write_line(format!("[{}] {}",style("Trace").blue(), message).as_str()).unwrap(),
        LogLevel::Info => STDOUT.write_line(format!("[{}] {}",style("Info").green(), message).as_str()).unwrap(),
        LogLevel::Warn => STDOUT.write_line(format!("[{}] {}",style("Warning").yellow(), message).as_str()).unwrap(),
        LogLevel::Error => STDOUT.write_line(format!("[{}] {}",style("Error").red(), message).as_str()).unwrap(),
    }
}

pub fn log_trace(message: &str) {
    log_string(LogLevel::Trace, message);
}

pub fn log_info(message: &str) {
    log_string(LogLevel::Info, message);
}

pub fn log_warn(message: &str) {
    log_string(LogLevel::Warn, message);
}

pub fn log_error(message: &str) {
    log_string(LogLevel::Error, message);
}
