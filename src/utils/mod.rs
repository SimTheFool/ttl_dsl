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
}
