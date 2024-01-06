use crate::{domain::resolution::ResolvedResource, result::AppResult};

pub trait ConfigProviderPort: Sync {
    fn get_transform_layers(&self) -> AppResult<Vec<String>>;
}

pub trait ResolverPort: Sync {
    fn read(&self, path: &str) -> AppResult<String>;
}

pub trait FormatterPort: Sync {
    type Format;
    fn format(&self, data: Vec<ResolvedResource>) -> AppResult<Self::Format>;
}
