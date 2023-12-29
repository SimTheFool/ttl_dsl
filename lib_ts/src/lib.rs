use crate::utils::{into_app_result, AppErr};
use lib_interpreter::ports::ResolverPort;
use resolver_adapters::JSClosureResolver;
use wasm_bindgen::prelude::*;

mod resolver_adapters;
mod utils;

#[wasm_bindgen]
pub struct Interpreter {
    resolver: JSClosureResolver,
}

#[wasm_bindgen]
impl Interpreter {
    #[wasm_bindgen(constructor)]
    pub fn new(f: &js_sys::Function) -> Self {
        let resolver = JSClosureResolver::new(f.clone());
        Self { resolver }
    }

    pub fn test(&self, input: &str) -> Result<String, AppErr> {
        let res = self.resolver.read(input);
        into_app_result(res)
    }
}
