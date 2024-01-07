use lib_core::result::AppResult;
use wasm_bindgen::prelude::*;

pub fn into_app_result<T>(res: AppResult<T>) -> Result<T, JsValue> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}
