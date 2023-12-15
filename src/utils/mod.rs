pub mod result;

#[cfg(test)]
pub mod test_utils {

    #[macro_export]
    macro_rules! as_variant {
        ($expr:expr, $( $enum:path ),+) => {
            match $expr {
                $(
                    $enum(inner) => inner,
                )+
                _ => panic!("Unexpected variant"),
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
}
