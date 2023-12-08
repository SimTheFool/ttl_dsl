use std::{cell::RefCell, rc::Rc};

use custom_dsl::{
    commands::AssembleFromStr,
    domain::resolution::ResolvedResource,
    ports::{MockedConfigProviderAdapter, MockedResolverAdapter},
    result::AppResult,
};

pub struct MockedApp {
    resolver: Rc<RefCell<MockedResolverAdapter<'static>>>,
    config: Rc<RefCell<MockedConfigProviderAdapter>>,
}

impl MockedApp {
    pub fn new() -> (
        MockedApp,
        Rc<RefCell<MockedResolverAdapter<'static>>>,
        Rc<RefCell<MockedConfigProviderAdapter>>,
    ) {
        let resolver = Rc::new(RefCell::new(MockedResolverAdapter::new()));
        let config = Rc::new(RefCell::new(MockedConfigProviderAdapter::new()));

        let app = MockedApp {
            resolver: resolver.clone(),
            config: config.clone(),
        };

        return (app, resolver, config);
    }
}

impl MockedApp {
    pub fn assemble_from_str(&self, file_str: &str) -> AppResult<Vec<ResolvedResource>> {
        let assemble_from_str = AssembleFromStr {
            resolver: &*self.resolver.borrow(),
            config: &*self.config.borrow(),
        };

        assemble_from_str.execute(file_str)
    }
}

macro_rules! unwrap_or_print_error {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(err) => {
                eprintln!("Error: {}", err);
                let xxx = format!("{:?}", err);
                panic!("{}", xxx);
            }
        }
    };
}
