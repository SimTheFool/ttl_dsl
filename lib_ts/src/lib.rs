use crate::{js_ports::JsResolverPort, utils::into_app_result};
use lib_interpreter::ports::ResolverPort;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;

pub mod js_ports;
mod utils;

#[wasm_bindgen]
pub struct Interpreter {
    resolver: Rc<RefCell<dyn ResolverPort>>,
}

#[wasm_bindgen]
impl Interpreter {
    #[wasm_bindgen(constructor)]
    pub fn new(resolver: JsResolverPort) -> Self {
        let resolver = Rc::new(RefCell::new(resolver));

        Self { resolver }
    }

    pub fn test(&self, input: &str) -> Result<String, JsValue> {
        let res = self.resolver.borrow().read(input);
        into_app_result(res)
    }
}
