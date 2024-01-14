use super::AssembleFromStr;
use crate::ports::{ConfigProviderPort, FormatterPort, ResolverPort};
use crate::statics::logger::info;
use crate::utils::result::AppResult;

pub struct AssembleFromPath<'a, R: ResolverPort, C: ConfigProviderPort, F: FormatterPort> {
    pub resolver: &'a R,
    pub config: &'a C,
    pub formatter: &'a F,
}

impl<'a, R: ResolverPort, C: ConfigProviderPort, F: FormatterPort> AssembleFromPath<'a, R, C, F> {
    pub fn new(resolver: &'a R, config: &'a C, formatter: &'a F) -> AssembleFromPath<'a, R, C, F> {
        AssembleFromPath {
            resolver,
            config,
            formatter,
        }
    }
}

impl<R: ResolverPort, C: ConfigProviderPort, F: FormatterPort> AssembleFromPath<'_, R, C, F> {
    pub fn execute(&self, path: &str) -> AppResult<F::Format> {
        info!("Assembling from path {}", path);

        let str = self.resolver.read(path)?;
        let assemble_from_str = AssembleFromStr {
            resolver: self.resolver,
            config: self.config,
            formatter: self.formatter,
        };

        assemble_from_str.execute(&str)
    }
}
