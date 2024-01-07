use lib_core::ports::ConfigProviderPort;

mod js_custom_config;
pub use js_custom_config::*;

pub enum ConfigProvider {
    JsConfigProvider(JsCustomConfig),
}
impl ConfigProvider {
    pub fn get(&self) -> &impl ConfigProviderPort {
        match self {
            Self::JsConfigProvider(config_provider) => config_provider,
        }
    }
}
