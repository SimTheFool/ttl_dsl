mod js_custom_resolver;
pub use js_custom_resolver::*;
use lib_core::ports::ResolverPort;

pub enum Resolver {
    JsResolver(JsCustomResolver),
}
impl Resolver {
    pub fn get(&self) -> &impl ResolverPort {
        match self {
            Self::JsResolver(resolver) => resolver,
        }
    }
}
