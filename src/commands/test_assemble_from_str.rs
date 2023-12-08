#[cfg(test)]
mod tests {
    use assemble_from_str::AssembleFromStr;

    use crate::{
        commands::assemble_from_str,
        domain::resolution::ResolvedResourceValue,
        ports::{MockedConfigProviderAdapter, MockedResolverAdapter},
    };

    #[test]
    fn it_should_create_resources_only() {
        let mocked_resolver = MockedResolverAdapter::new();
        let mocked_config = MockedConfigProviderAdapter::new();
        let assemble_from_str = AssembleFromStr {
            resolver: &mocked_resolver,
            config: &mocked_config,
        };
        let values = assemble_from_str
            .execute(
                r#"{
                var05: "hello"
                var06: {
                    var07: 07
                    var08: 08
                }
            }"#,
            )
            .unwrap();

        assert_eq!(values.len(), 3);

        let first_ressource = values.get(0).unwrap();
        let second_ressource = values.get(1).unwrap();
        let third_ressource = values.get(2).unwrap();

        assert_eq!(first_ressource.identifier, Some("var05".to_string()));
        match &first_ressource.value {
            ResolvedResourceValue::String(x) => {
                assert_eq!(x, "hello");
            }
            _ => panic!("Should be a string"),
        }

        assert_eq!(second_ressource.identifier, Some("var07".to_string()));
        match &second_ressource.value {
            ResolvedResourceValue::Number(x) => {
                assert_eq!(x, &7.0);
            }
            _ => panic!("Should be a number"),
        }

        assert_eq!(third_ressource.identifier, Some("var08".to_string()));
        match &third_ressource.value {
            ResolvedResourceValue::Number(x) => {
                assert_eq!(x, &8.0);
            }
            _ => panic!("Should be a number"),
        }
    }

    #[test]
    fn it_should_create_resources_with_context() {
        let mocked_resolver = MockedResolverAdapter::new();
        let mocked_config = MockedConfigProviderAdapter::new();
        let assemble_from_str = AssembleFromStr {
            resolver: &mocked_resolver,
            config: &mocked_config,
        };

        let values = assemble_from_str
            .execute(
                r#"{
                var05: "hello"
                var06: {
                    var07: 07
                }
            }"#,
            )
            .unwrap();

        assert_eq!(values.len(), 2);

        let first_ressource = values.get(0).unwrap();
        let second_ressource = values.get(1).unwrap();

        assert_eq!(first_ressource.identifier, Some("var05".to_string()));
        match &first_ressource.value {
            ResolvedResourceValue::String(x) => {
                assert_eq!(x, "hello");
            }
            _ => panic!("Should be a string"),
        }

        assert_eq!(second_ressource.identifier, Some("var07".to_string()));
        match &second_ressource.value {
            ResolvedResourceValue::Number(x) => {
                assert_eq!(x, &7.0);
            }
            _ => panic!("Should be a number"),
        }
    }

    #[test]
    fn it_should_create_resources_with_import() {
        let mut mocked_resolver = MockedResolverAdapter::new();
        let mocked_config = MockedConfigProviderAdapter::new();

        mocked_resolver.mock_file(
            "./stats",
            r#"{
                somevar01: var01
                somevar02: var02
                [var01 var02]
                someothervar: "statistics"
            }"#,
        );

        let assemble_from_str = AssembleFromStr {
            resolver: &mocked_resolver,
            config: &mocked_config,
        };

        let values = assemble_from_str
            .execute(
                r#"{
                << ./stats
                    with var01 : 001
                    with var02 : "002"
                var03: 003
            }"#,
            )
            .unwrap();

        assert_eq!(values.len(), 4);

        let first_ressource = values.get(0).unwrap();
        let second_ressource = values.get(1).unwrap();
        let third_ressource = values.get(2).unwrap();
        let fourth_ressource = values.get(3).unwrap();

        assert_eq!(first_ressource.identifier, Some("somevar01".to_string()));
        match &first_ressource.value {
            ResolvedResourceValue::Number(x) => {
                assert_eq!(x, &1.0);
            }
            _ => panic!("Should be a number"),
        }

        assert_eq!(second_ressource.identifier, Some("somevar02".to_string()));
        match &second_ressource.value {
            ResolvedResourceValue::String(x) => {
                assert_eq!(x, "002");
            }
            _ => panic!("Should be a string"),
        }

        assert_eq!(third_ressource.identifier, Some("someothervar".to_string()));
        match &third_ressource.value {
            ResolvedResourceValue::String(x) => {
                assert_eq!(x, "statistics");
            }
            _ => panic!("Should be a string"),
        }

        let third_resources_metas = &third_ressource.metas;
        assert_eq!(third_resources_metas.len(), 2);

        let first_meta = third_resources_metas.get(0).unwrap();
        assert_eq!(first_meta.identifier, None);
        match first_meta.value {
            ResolvedResourceValue::Number(x) => {
                assert_eq!(x, 1.0);
            }
            _ => panic!("Should be a number"),
        }

        let second_meta = third_resources_metas.get(1).unwrap();
        assert_eq!(second_meta.identifier, None);
        match &second_meta.value {
            ResolvedResourceValue::String(x) => {
                assert_eq!(x, "002");
            }
            _ => panic!("Should be a string"),
        }

        assert_eq!(fourth_ressource.identifier, Some("var03".to_string()));
        match fourth_ressource.value {
            ResolvedResourceValue::Number(x) => {
                assert_eq!(x, 3.0);
            }
            _ => panic!("Should be a number"),
        }
    }

    #[test]
    fn it_should_create_resources_with_transforms() {
        let mocked_resolver = MockedResolverAdapter::new();
        let mut mocked_config = MockedConfigProviderAdapter::new();
        mocked_config.add_layer("FIRST_LAYER");
        mocked_config.add_layer("SECOND_LAYER");
        let assemble_from_str = AssembleFromStr {
            resolver: &mocked_resolver,
            config: &mocked_config,
        };

        let values = assemble_from_str
            .execute(
                r#"
            {
                x: 5
            }

            @TRANSFORM SECOND_LAYER
            > x += 3
            > x *= 3

            @TRANSFORM FIRST_LAYER
            > x *= 2
            > x += 2

            "#,
            )
            .unwrap();

        assert_eq!(values.len(), 1);

        let first_ressource = values.get(0).unwrap();
        assert_eq!(first_ressource.identifier, Some("x".to_string()));
        match first_ressource.value {
            ResolvedResourceValue::Number(x) => {
                assert_eq!(x, 45.0);
            }
            _ => panic!("Should be a number"),
        }
    }
}
