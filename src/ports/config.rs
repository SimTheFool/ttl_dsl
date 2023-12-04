use crate::utils::result::AppResult;

pub trait ConfigProviderPort: Sync {
    fn get_transform_layers(&self) -> AppResult<Vec<&str>>;
}

pub struct MockedConfigProviderAdapter {
    mocking_store: Vec<&'static str>,
}

impl MockedConfigProviderAdapter {
    pub fn new() -> Self {
        MockedConfigProviderAdapter {
            mocking_store: Vec::new(),
        }
    }

    pub fn add_layer(&mut self, layer: &'static str) {
        self.mocking_store.push(layer);
    }
}

impl ConfigProviderPort for MockedConfigProviderAdapter {
    fn get_transform_layers(&self) -> AppResult<Vec<&str>> {
        return Ok(self.mocking_store.clone());
    }
}
