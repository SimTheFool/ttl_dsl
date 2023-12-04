use std::collections::HashMap;

use crate::utils::result::{AppError, AppResult};

pub trait ResolverPort: Sync {
    fn read(&self, path: &str) -> AppResult<String>;
}

pub struct MockedResolverAdapter<'a> {
    mocking_store: std::collections::HashMap<&'a str, &'a str>,
}

impl<'a> MockedResolverAdapter<'a> {
    pub fn new() -> Self {
        MockedResolverAdapter {
            mocking_store: HashMap::new(),
        }
    }

    pub fn mock_file(&mut self, path: &'a str, content: &'a str) {
        self.mocking_store.insert(path, content);
    }
}

impl<'a> ResolverPort for MockedResolverAdapter<'a> {
    fn read(&self, path: &str) -> AppResult<String> {
        let file = self
            .mocking_store
            .get(path)
            .ok_or(AppError::Str("Cannot find mocked file"))?;
        return Ok(file.to_string());
    }
}
