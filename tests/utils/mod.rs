#![allow(dead_code, unused_variables)]

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

#[macro_export]
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

#[macro_export]
macro_rules! assert_resource {
    ($name:ident : $ident:expr, $type:ident $value:expr) => {
        assert_eq!($name.identifier, Some($ident.to_string()));

        match &$name.value {
            ResolvedResourceValue::$type(x) => {
                assert_eq!(x, &$value);
            }
            _ => panic!(concat!("Should be a ", stringify!($type))),
        }
    };
    ($name:ident : $type:ident $value:expr) => {
        match &$name.value {
            ResolvedResourceValue::$type(x) => {
                assert_eq!(x, &$value);
            }
            _ => panic!(concat!("Should be a ", stringify!($type))),
        }
    };
}

#[macro_export]
macro_rules! assert_resource_at {
    ($name:ident : $identifier:expr => $type:ident $value:expr) => {
        let value = &$name
            .iter()
            .find(|r| r.identifier == Some($identifier.to_string()))
            .unwrap()
            .value;

        match value {
            ResolvedResourceValue::$type(n) => {
                assert_eq!(n, &$value, "Error for id: {}", stringify!($identifier))
            }
            _ => panic!(concat!(
                stringify!($identifier),
                " should be a ",
                stringify!($type)
            )),
        }
    };
}
