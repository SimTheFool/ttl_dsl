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
    ($name:ident: $identifier:expr => $type:ident $value:expr) => {
        let regex = Regex::new($identifier).unwrap();

        let values = &$name
            .iter()
            .filter(|r| r.identifier.is_some() && regex.is_match(&r.identifier.as_ref().unwrap()))
            .map(|r| &r.value)
            .collect::<Vec<_>>();

        let res1 = values
            .into_iter()
            .filter_map(|v| match v {
                ResolvedResourceValue::$type(x) => Some(x),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert!(
            res1.len() >= 1,
            "No {} found for id: {}",
            stringify!($type),
            $identifier
        );

        let res2 = res1.into_iter().any(|x| x == &$value);

        assert!(res2, "Error for id: {}", $identifier);
    };
}
