use crate::utils::result::AppResult;

pub trait TTLInputPort {
    fn read(&self, path: &str) -> AppResult<String>;
}
