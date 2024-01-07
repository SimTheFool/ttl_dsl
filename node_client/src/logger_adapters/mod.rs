mod js_console_logger;
pub use js_console_logger::*;

pub enum Logger {
    ConsoleLogger(JSConsoleLogger),
}
impl Logger {
    pub fn get_owned(self) -> impl lib_core::statics::logger::Log {
        match self {
            Self::ConsoleLogger(logger) => logger,
        }
    }
}
