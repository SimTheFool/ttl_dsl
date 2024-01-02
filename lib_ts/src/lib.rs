use crate::utils::into_app_result;
use js_interfaces::IResolver;
use lib_interpreter::ports::ResolverPort;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;

pub mod js_interfaces;
mod utils;

#[wasm_bindgen]
pub struct Interpreter {
    resolver: Rc<RefCell<dyn ResolverPort>>,
}

#[wasm_bindgen]
impl Interpreter {
    #[wasm_bindgen(constructor)]
    pub fn new(resolver: IResolver) -> Self {
        let resolver = Rc::new(RefCell::new(resolver));
        Self { resolver }
    }

    pub fn test(&self, input: &str) -> Result<String, JsValue> {
        let res = self.resolver.borrow().read(input);
        into_app_result(res)
    }
}
