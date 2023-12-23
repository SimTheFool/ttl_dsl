use crate::result::AppResult;

pub trait ConfigProviderPort: Sync {
    fn get_transform_layers(&self) -> AppResult<Vec<&str>>;
}

pub trait ResolverPort: Sync {
    fn read(&self, path: &str) -> AppResult<String>;
}
