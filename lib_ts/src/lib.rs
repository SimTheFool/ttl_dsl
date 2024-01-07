use crate::{
    formatter_adapters::JsJsonFormatter, logger_adapters::JSConsoleLogger, utils::into_app_result,
};
use config_provider_adapter::{ConfigProvider, JsCustomConfig};
use formatter_adapters::Formatter;
use lib_interpreter::{commands::AssembleFromStr, statics::logger};
use logger_adapters::Logger;
use resolver_adapters::{JsCustomResolver, Resolver};
use wasm_bindgen::prelude::*;

mod config_provider_adapter;
mod formatter_adapters;
mod logger_adapters;
mod resolver_adapters;
mod utils;

#[wasm_bindgen]
pub struct InterpreterBuilder {
    resolver: Option<Resolver>,
    config_provider: Option<ConfigProvider>,
    formatter: Option<Formatter>,
    logger: Option<Logger>,
}
#[wasm_bindgen]
impl InterpreterBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            resolver: None,
            config_provider: None,
            formatter: Some(Formatter::JsonFormatter(JsJsonFormatter::new())),
            logger: Some(Logger::ConsoleLogger(JSConsoleLogger)),
        }
    }

    #[wasm_bindgen(js_name = withResolverCustom)]
    pub fn with_resolver_custom(mut self, resolver: JsCustomResolver) -> Self {
        self.resolver = Some(Resolver::JsResolver(resolver));
        self
    }

    #[wasm_bindgen(js_name = withConfProviderCustom)]
    pub fn with_config_provider_custom(mut self, config_provider: JsCustomConfig) -> Self {
        self.config_provider = Some(ConfigProvider::JsConfigProvider(config_provider));
        self
    }

    #[wasm_bindgen(js_name = withFormatterJson)]
    pub fn with_formatter_json(mut self) -> Self {
        self.formatter = Some(Formatter::JsonFormatter(JsJsonFormatter::new()));
        self
    }

    #[wasm_bindgen(js_name = withLoggerConsole)]
    pub fn with_logger_console(mut self) -> Self {
        self.logger = Some(Logger::ConsoleLogger(JSConsoleLogger));
        self
    }

    pub fn build(self) -> Result<Interpreter, JsValue> {
        match (
            self.resolver,
            self.config_provider,
            self.formatter,
            self.logger,
        ) {
            (Some(resolver), Some(config), Some(formatter), Some(logger)) => {
                Self::init_interpreter(resolver, config, formatter, logger)
            }
            (None, _, _, _) => Err(JsValue::from_str(
                "Cannot build interpreter: resolver is not set",
            )),
            (_, None, _, _) => Err(JsValue::from_str(
                "Cannot build interpreter: config provider is not set",
            )),
            (_, _, None, _) => Err(JsValue::from_str(
                "Cannot build interpreter: formatter is not set",
            )),
            (_, _, _, None) => Err(JsValue::from_str(
                "Cannot build interpreter: logger is not set",
            )),
        }
    }

    fn init_interpreter(
        resolver: Resolver,
        config_provider: ConfigProvider,
        formatter: Formatter,
        logger: Logger,
    ) -> Result<Interpreter, JsValue> {
        let logger = logger.get_owned();
        logger::set_static_logger(Box::new(logger));
        logger::set_static_logger_level(logger::LevelFilter::Trace);

        Ok(Interpreter {
            resolver,
            config_provider,
            formatter,
        })
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
