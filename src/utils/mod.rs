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
}
