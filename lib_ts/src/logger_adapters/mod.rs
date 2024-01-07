mod js_console_logger;
pub use js_console_logger::*;

pub enum Logger {
    ConsoleLogger(JSConsoleLogger),
}
impl Logger {
    pub fn get(self) -> impl lib_interpreter::statics::logger::Log {
        match self {
            Self::ConsoleLogger(logger) => logger,
        }
    }
}
