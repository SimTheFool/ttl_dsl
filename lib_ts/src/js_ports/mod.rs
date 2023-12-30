use lib_interpreter::{
    ports::ResolverPort,
    result::{AppError, AppResult},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type JsResolverPort;

    #[wasm_bindgen(structural, method, catch)]
    pub fn read(this: &JsResolverPort, input: &str) -> Result<String, JsValue>;
}

unsafe impl Sync for JsResolverPort {}
unsafe impl Send for JsResolverPort {}
impl ResolverPort for JsResolverPort {
    fn read(&self, path: &str) -> AppResult<String> {
        let res = self
            .read(path)
            .map_err(|e| AppError::String(e.as_string().unwrap_or_default()))?;
        Ok(res)
    }
}
