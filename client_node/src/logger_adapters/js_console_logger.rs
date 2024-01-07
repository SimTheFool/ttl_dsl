use lib_core::statics::logger::{self, Level};
use wasm_bindgen::JsValue;
use web_sys::console;

pub struct JSConsoleLogger;

impl logger::Log for JSConsoleLogger {
    fn enabled(&self, metadata: &logger::Metadata) -> bool {
        metadata.level() <= logger::Level::Info
    }
    fn log(&self, record: &logger::Record) {
        let enabled = self.enabled(record.metadata());

        if !enabled {
            return;
        }

        let message = format!("{}", record.args());
        let message = JsValue::from_str(&message);

        match record.level() {
            Level::Error => console::error_1(&message),
            _ => console::log_1(&message),
        };
    }
    fn flush(&self) {}
}
