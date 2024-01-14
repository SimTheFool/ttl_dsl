use crate::{
    ports::ResolverPort,
    result::{AppError, AppResult},
};

pub struct FSResolver {
    resolution_dir: String,
}
impl FSResolver {
    pub fn new(resolution_dir: impl Into<String>) -> Self {
        Self {
            resolution_dir: resolution_dir.into(),
        }
    }
}
impl ResolverPort for FSResolver {
    fn read(&self, path: &str) -> AppResult<String> {
        let path = std::path::Path::new(&self.resolution_dir).join(path);
        match path.exists() {
            true => std::fs::read_to_string(path).map_err(|e| AppError::String(e.to_string())),
            false => Err(AppError::String(format!(
                "File {} does not exist",
                path.display()
            ))),
        }
    }
}
