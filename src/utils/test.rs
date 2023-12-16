#[macro_export]
macro_rules! as_variant {
    ($expr:expr, $( $enum:path ),+) => {
        match $expr {
            $(
                $enum(inner) => inner,
            )+
            _ => panic!("Unexpected variant: "),
        }
    };
}

#[macro_export]
macro_rules! print_unwrap {
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
macro_rules! str_to_ast {
    ($str:expr, $parser:path, $ast_type:path) => {{
        use $ast_type as f;
        let mut pairs = print_unwrap!(TTLParser::parse($parser, $str));
        print_unwrap!(f::from_pest(&mut pairs))
    }};
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
