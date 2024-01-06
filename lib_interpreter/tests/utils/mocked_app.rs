use lib_interpreter::{
    commands::AssembleFromStr,
    domain::resolution::ResolvedResource,
    ports::{ConfigProviderPort, ResolverPort},
    result::{AppError, AppResult},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct MockedApp {
    resolver: Rc<RefCell<MockedResolverAdapter<'static>>>,
    config: Rc<RefCell<MockedConfigProviderAdapter>>,
    formatter: Rc<RefCell<NoFormatterAdapter>>,
}

impl MockedApp {
    pub fn new() -> (
        MockedApp,
        Rc<RefCell<MockedResolverAdapter<'static>>>,
        Rc<RefCell<MockedConfigProviderAdapter>>,
    ) {
        let resolver = Rc::new(RefCell::new(MockedResolverAdapter::new()));
        let config = Rc::new(RefCell::new(MockedConfigProviderAdapter::new()));
        let formatter = Rc::new(RefCell::new(NoFormatterAdapter::new()));

        let app = MockedApp {
            resolver: resolver.clone(),
            config: config.clone(),
            formatter: formatter.clone(),
        };

        (app, resolver, config)
    }
}

impl MockedApp {
    pub fn assemble_from_str(&self, file_str: &str) -> AppResult<Vec<ResolvedResource>> {
        let assemble_from_str = AssembleFromStr {
            resolver: &*self.resolver.borrow(),
            config: &*self.config.borrow(),
            formatter: &*self.formatter.borrow(),
        };

        assemble_from_str.execute(file_str)
    }
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
        Ok(file.to_string())
    }
}

pub struct MockedConfigProviderAdapter {
    mocking_store: Vec<String>,
}
impl MockedConfigProviderAdapter {
    pub fn new() -> Self {
        MockedConfigProviderAdapter {
            mocking_store: Vec::new(),
        }
    }

    pub fn add_layer(&mut self, layer: &'static str) {
        self.mocking_store.push(layer.to_string());
    }
}
impl ConfigProviderPort for MockedConfigProviderAdapter {
    fn get_transform_layers(&self) -> AppResult<Vec<String>> {
        Ok(self.mocking_store.clone())
    }
}

pub struct NoFormatterAdapter {}

impl NoFormatterAdapter {
    pub fn new() -> Self {
        NoFormatterAdapter {}
    }
}

impl lib_interpreter::ports::FormatterPort for NoFormatterAdapter {
    type Format = Vec<ResolvedResource>;

    fn format(&self, resources: Vec<ResolvedResource>) -> AppResult<Self::Format> {
        Ok(resources)
    }
}
