#[cfg(test)]
mod tests {
    use assemble_from_str::AssembleFromStr;

    use crate::{
        commands::assemble_from_str,
        domain::resource::ResolvedResources,
        ports::{MockedConfigProviderAdapter, MockedResolverAdapter},
    };

    #[test]
    fn it_should_create_resources() {
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

        match first_ressource {
            ResolvedResources::String(x) => {
                assert_eq!(x.identifier, Some("var05".to_string()));
                assert_eq!(x.value, "hello");
            }
            _ => panic!("Should be a string"),
        }

        match second_ressource {
            ResolvedResources::Number(x) => {
                assert_eq!(x.identifier, Some("var07".to_string()));
                assert_eq!(x.value, 7.0);
            }
            _ => panic!("Should be a number"),
        }

        match third_ressource {
            ResolvedResources::Number(x) => {
                assert_eq!(x.identifier, Some("var08".to_string()));
                assert_eq!(x.value, 8.0);
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

        match first_ressource {
            ResolvedResources::String(x) => {
                assert_eq!(x.identifier, Some("var05".to_string()));
                assert_eq!(x.value, "hello");
                assert_eq!(x.context.variables, None);
                assert_eq!(x.context.path, Some("var05".to_string()));
            }
            _ => panic!("Should be a string"),
        }

        match second_ressource {
            ResolvedResources::Number(x) => {
                assert_eq!(x.identifier, Some("var07".to_string()));
                assert_eq!(x.value, 7.0);
                assert_eq!(x.context.variables, None);
                assert_eq!(x.context.path, Some("var06.var07".to_string()));
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

        match first_ressource {
            ResolvedResources::Number(x) => {
                assert_eq!(x.identifier, Some("somevar01".to_string()));
                assert_eq!(x.value, 1.0);
            }
            _ => panic!("Should be a number"),
        }

        match second_ressource {
            ResolvedResources::String(x) => {
                assert_eq!(x.identifier, Some("somevar02".to_string()));
                assert_eq!(x.value, "002");
            }
            _ => panic!("Should be a string"),
        }

        match third_ressource {
            ResolvedResources::String(x) => {
                assert_eq!(x.identifier, Some("someothervar".to_string()));
                assert_eq!(x.value, "statistics");
                let metas = x.metas.as_ref().unwrap();

                match metas.get(0).unwrap() {
                    ResolvedResources::Number(x) => {
                        assert_eq!(x.identifier, None);
                        assert_eq!(x.value, 1.0);
                    }
                    _ => panic!("Should be a number"),
                }

                match metas.get(1).unwrap() {
                    ResolvedResources::String(x) => {
                        assert_eq!(x.identifier, None);
                        assert_eq!(x.value, "002");
                    }
                    _ => panic!("Should be a string"),
                }
            }
            _ => panic!("Should be a string"),
        }

        match fourth_ressource {
            ResolvedResources::Number(x) => {
                assert_eq!(x.identifier, Some("var03".to_string()));
                assert_eq!(x.value, 3.0);
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

        match first_ressource {
            ResolvedResources::Number(x) => {
                assert_eq!(x.identifier, Some("x".to_string()));
                assert_eq!(x.value, 45.0);
            }
            _ => panic!("Should be a number"),
        }
    }
}
