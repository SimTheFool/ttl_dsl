use lib_core::{
    ports::ResolverPort,
    result::{AppError, AppResult},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const IRESOLVER_PORT_DEFINITION: &'static str = r#"
interface ICustomResolver {
    read: (input: string) => string;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ICustomResolver")]
    pub type JsCustomResolver;

    #[wasm_bindgen(structural, method, catch)]
    pub fn read(this: &JsCustomResolver, input: &str) -> Result<String, JsValue>;
}

unsafe impl Sync for JsCustomResolver {}
unsafe impl Send for JsCustomResolver {}
impl ResolverPort for JsCustomResolver {
    fn read(&self, path: &str) -> AppResult<String> {
        let res = self
            .read(path)
            .map_err(|e| AppError::String(e.as_string().unwrap_or_default()))?;
        Ok(res)
    }
}
