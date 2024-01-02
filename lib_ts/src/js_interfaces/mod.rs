use lib_interpreter::{
    ports::ResolverPort,
    result::{AppError, AppResult},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const IRESOLVER_PORT_DEFINITION: &'static str = r#"
interface IResolver {
    read: (input: string) => string;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IResolver")]
    pub type IResolver;

    #[wasm_bindgen(structural, method, catch)]
    pub fn read(this: &IResolver, input: &str) -> Result<String, JsValue>;
}

unsafe impl Sync for IResolver {}
unsafe impl Send for IResolver {}
impl ResolverPort for IResolver {
    fn read(&self, path: &str) -> AppResult<String> {
        let res = self
            .read(path)
            .map_err(|e| AppError::String(e.as_string().unwrap_or_default()))?;
        Ok(res)
    }
}
