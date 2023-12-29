use lib_interpreter::result::{AppError, AppResult};
use wasm_bindgen::JsValue;

pub struct JSClosureResolver {
    closure: js_sys::Function,
}
impl JSClosureResolver {
    pub fn new(closure: js_sys::Function) -> Self {
        Self { closure }
    }
}
unsafe impl Send for JSClosureResolver {}
unsafe impl Sync for JSClosureResolver {}
impl lib_interpreter::ports::ResolverPort for JSClosureResolver {
    fn read(&self, input: &str) -> AppResult<String> {
        let this = JsValue::null();
        let res = self.closure.call1(&this, &JsValue::from(input));
        let res = res.map_err(|e| AppError::String(e.as_string().unwrap_or_default()))?;
        let res = res.as_string().ok_or(AppError::Str("not a string"))?;
        Ok(res)
    }
}
