#[cfg(test)]
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
#[cfg(test)]
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
#[cfg(test)]
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
#[cfg(test)]
#[macro_export]
macro_rules! assert_resource_at {
    ($name:ident: $identifier:expr => $type:ident $value:expr) => {
        let regex = Regex::new($identifier).unwrap();

        let values = &$name
            .iter()
            .filter(|r| r.identifier.is_some() && regex.is_match(&r.identifier.as_ref().unwrap()))
            .map(|r| &r.value)
            .collect::<Vec<_>>();

        let filtered_match = values
            .into_iter()
            .filter_map(|v| match v {
                ResolvedResourceValue::$type(x) => Some(x),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert!(
            filtered_match.len() >= 1,
            "No {} found for id: {}",
            stringify!($type),
            $identifier
        );

        let has_value = filtered_match.clone().into_iter().any(|x| x == &$value);

        if !has_value {
            panic!(
                "No value {} for {} in {:?}",
                $value, $identifier, filtered_match
            );
        }
    };
    ($name:ident: $identifier:expr => $type:ident) => {
        let regex = Regex::new($identifier).unwrap();

        let values = &$name
            .iter()
            .filter(|r| r.identifier.is_some() && regex.is_match(&r.identifier.as_ref().unwrap()))
            .map(|r| &r.value)
            .collect::<Vec<_>>();

        let has_variant = values.into_iter().any(|v| match v {
            ResolvedResourceValue::$type => true,
            _ => false,
        });

        if !has_variant {
            panic!("No {} found for id: {}", stringify!($type), $identifier);
        }
    };
}
