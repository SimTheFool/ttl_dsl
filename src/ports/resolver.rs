use std::collections::HashMap;

use crate::utils::result::{AppError, AppResult};

pub trait ResolverPort: Sync {
    fn read(&self, path: &str) -> AppResult<String>;
}

pub struct MockedResolverAdapter<'a> {
    mocking_store: std::collections::HashMap<String, &'a str>,
}

impl<'a> MockedResolverAdapter<'a> {
    pub fn new() -> Self {
        MockedResolverAdapter {
            mocking_store: HashMap::new(),
        }
    }

    pub fn mock_file(&mut self, path: impl Into<String>, content: &'a str) {
        self.mocking_store.insert(path.into(), content);
    }
}

impl<'a> ResolverPort for MockedResolverAdapter<'a> {
    fn read(&self, path: &str) -> AppResult<String> {
        let file = self
            .mocking_store
            .get(path)
            .ok_or(AppError::String(format!(
                "Cannot find mocked file, asking for {:#?}, available {:#?}",
                path,
                self.mocking_store.keys()
            )))?;
        return Ok(file.to_string());
    }
}
