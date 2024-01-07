use lib_core::{
    domain::resolution::ResolvedResource,
    ports::FormatterPort,
    prefab::JsonFormatter,
    result::{AppError, AppResult},
};
use serde::ser::Serialize;
use wasm_bindgen::JsValue;

pub struct JsJsonFormatter {
    json_formatter: JsonFormatter,
}
impl JsJsonFormatter {
    pub fn new() -> Self {
        Self {
            json_formatter: JsonFormatter::new(),
        }
    }
}
impl FormatterPort for JsJsonFormatter {
    type Format = JsValue;

    fn format(&self, data: Vec<ResolvedResource>) -> AppResult<Self::Format> {
        let json = self.json_formatter.format(data)?;
        let serializer = serde_wasm_bindgen::Serializer::json_compatible();
        let js_json = json
            .serialize(&serializer)
            .map_err(|err| AppError::String(format!("Failed to serialize json: {:?}", err)))?;

        Ok(js_json)
    }
}
