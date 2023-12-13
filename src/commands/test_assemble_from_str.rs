#[cfg(test)]
mod tests {
    use assemble_from_str::AssembleFromStr;

    use crate::{
        as_variant,
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
        let value = as_variant!(&first_ressource.value, ResolvedResourceValue::String);
        assert_eq!(value, "hello");

        assert_eq!(second_ressource.identifier, Some("var06.var07".to_string()));
        let value = as_variant!(&second_ressource.value, ResolvedResourceValue::Number);
        assert_eq!(value, &7.0);

        assert_eq!(third_ressource.identifier, Some("var06.var08".to_string()));
        let value = as_variant!(&third_ressource.value, ResolvedResourceValue::Number);
        assert_eq!(value, &8.0);
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
        let value = as_variant!(&first_ressource.value, ResolvedResourceValue::String);
        assert_eq!(value, "hello");

        assert_eq!(second_ressource.identifier, Some("var06.var07".to_string()));
        let value = as_variant!(&second_ressource.value, ResolvedResourceValue::Number);
        assert_eq!(value, &7.0);
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
                    with var02 : "002" >
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
        let value = as_variant!(&first_ressource.value, ResolvedResourceValue::Number);
        assert_eq!(value, &1.0);

        assert_eq!(second_ressource.identifier, Some("somevar02".to_string()));
        let value = as_variant!(&second_ressource.value, ResolvedResourceValue::String);
        assert_eq!(value, "002");

        assert_eq!(third_ressource.identifier, Some("someothervar".to_string()));
        let value = as_variant!(&third_ressource.value, ResolvedResourceValue::String);
        assert_eq!(value, "statistics");

        let third_resources_metas = &third_ressource.metas;
        assert_eq!(third_resources_metas.len(), 2);

        let first_meta = third_resources_metas.get(0).unwrap();
        assert_eq!(first_meta.identifier, None);
        let meta_value = as_variant!(&first_meta.value, ResolvedResourceValue::Number);
        assert_eq!(meta_value, &1.0);

        let second_meta = third_resources_metas.get(1).unwrap();
        assert_eq!(second_meta.identifier, None);
        let meta_value = as_variant!(&second_meta.value, ResolvedResourceValue::String);
        assert_eq!(meta_value, "002");

        assert_eq!(fourth_ressource.identifier, Some("var03".to_string()));
        let value = as_variant!(&fourth_ressource.value, ResolvedResourceValue::Number);
        assert_eq!(value, &3.0);
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
        let value = as_variant!(&first_ressource.value, ResolvedResourceValue::Number);
        assert_eq!(value, &45.0);
    }

    #[test]
    fn it_should_assemble_nested_import() {
        let mut mocked_resolver = MockedResolverAdapter::new();
        let mocked_config = MockedConfigProviderAdapter::new();

        let stats_input = r#"{
            con: 1
            vol: 2
        }"#;
        let mag_input = r#"{
            mag: 3
        }"#;
        let root_input = r#"{
            << ./stats
                with << ./mag >
            >
        }"#;

        mocked_resolver.mock_file("./stats", stats_input);
        mocked_resolver.mock_file("./mag", mag_input);

        let assemble_from_str = AssembleFromStr {
            resolver: &mocked_resolver,
            config: &mocked_config,
        };

        let values = assemble_from_str.execute(root_input).unwrap();

        assert_eq!(values.len(), 3);

        let first_ressource = values.get(0).unwrap();
        assert_eq!(first_ressource.identifier, Some("con".to_string()));
        let value = as_variant!(&first_ressource.value, ResolvedResourceValue::Number);
        assert_eq!(value, &1.0);

        let second_ressource = values.get(1).unwrap();
        assert_eq!(second_ressource.identifier, Some("vol".to_string()));
        let value = as_variant!(&second_ressource.value, ResolvedResourceValue::Number);
        assert_eq!(value, &2.0);

        let third_ressource = values.get(2).unwrap();
        assert_eq!(third_ressource.identifier, Some("mag".to_string()));
        let value = as_variant!(&third_ressource.value, ResolvedResourceValue::Number);
        assert_eq!(value, &3.0);
    }
}
