use lib_core::ports::FormatterPort;
use wasm_bindgen::JsValue;

mod js_json_formatter;
pub use js_json_formatter::*;

pub enum Formatter {
    JsonFormatter(JsJsonFormatter),
}
impl Formatter {
    pub fn get(&self) -> &impl FormatterPort<Format = JsValue> {
        match self {
            Self::JsonFormatter(formatter) => formatter,
        }
    }
}
