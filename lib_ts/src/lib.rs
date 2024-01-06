use crate::{formatter_adapters::JsJsonFormatter, utils::into_app_result};
use config_provider_adapter::{ConfigProvider, JsCustomConfig};
use formatter_adapters::Formatter;
use lib_interpreter::commands::AssembleFromStr;
use resolver_adapters::{JsCustomResolver, Resolver};
use wasm_bindgen::prelude::*;

mod config_provider_adapter;
mod formatter_adapters;
mod resolver_adapters;
mod utils;

#[wasm_bindgen]
pub struct InterpreterBuilder {
    resolver: Option<Resolver>,
    config_provider: Option<ConfigProvider>,
    formatter: Option<Formatter>,
}
#[wasm_bindgen]
impl InterpreterBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            resolver: None,
            config_provider: None,
            formatter: None,
        }
    }

    pub fn with_custom_resolver(mut self, resolver: JsCustomResolver) -> Self {
        self.resolver = Some(Resolver::JsResolver(resolver));
        self
    }

    pub fn with_custom_config_provider(mut self, config_provider: JsCustomConfig) -> Self {
        self.config_provider = Some(ConfigProvider::JsConfigProvider(config_provider));
        self
    }

    pub fn with_json_formatter(mut self) -> Self {
        self.formatter = Some(Formatter::JsonFormatter(JsJsonFormatter::new()));
        self
    }

    pub fn build(self) -> Result<Interpreter, JsValue> {
        match (self.resolver, self.config_provider, self.formatter) {
            (Some(resolver), Some(config), Some(formatter)) => Ok(Interpreter {
                resolver,
                config_provider: config,
                formatter,
            }),
            (None, _, _) => Err(JsValue::from_str(
                "Cannot build interpreter: resolver is not set",
            )),
            (_, None, _) => Err(JsValue::from_str(
                "Cannot build interpreter: config provider is not set",
            )),
            (_, _, None) => Err(JsValue::from_str(
                "Cannot build interpreter: formatter is not set",
            )),
        }
    }
}

#[wasm_bindgen]
pub struct Interpreter {
    resolver: Resolver,
    config_provider: ConfigProvider,
    formatter: Formatter,
}

#[wasm_bindgen]
impl Interpreter {
    pub fn assemble_from_str(&self, input: &str) -> Result<JsValue, JsValue> {
        let command = AssembleFromStr {
            config: self.config_provider.get(),
            resolver: self.resolver.get(),
            formatter: self.formatter.get(),
        };
        let res = command.execute(input);
        into_app_result(res)
    }
}
