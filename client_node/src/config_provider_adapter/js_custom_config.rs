use lib_core::{
    ports::ConfigProviderPort,
    result::{AppError, AppResult},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const DEFINITION: &'static str = r#"
interface ICustomConfig {
    getTransformLayers: () => string[];
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ICustomConfig")]
    pub type JsCustomConfig;

    #[wasm_bindgen(structural, method, catch)]
    pub fn getTransformLayers(this: &JsCustomConfig) -> Result<Vec<String>, JsValue>;
}

unsafe impl Sync for JsCustomConfig {}
unsafe impl Send for JsCustomConfig {}
impl ConfigProviderPort for JsCustomConfig {
    fn get_transform_layers(&self) -> AppResult<Vec<String>> {
        let res = self
            .getTransformLayers()
            .map_err(|e| AppError::String(e.as_string().unwrap_or_default()))?;
        Ok(res)
    }
}
