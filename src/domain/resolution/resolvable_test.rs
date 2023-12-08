#[cfg(test)]
mod tests {
    use crate::domain::resolution::resolvable::Resolvable;
    use crate::domain::resolution::ResolvedResourceValue;
    use crate::domain::resolution::{
        RawResourceBuilder, ResolvedResource, ResolvedResourceBuilder,
    };
    use std::collections::HashMap;

    #[test]
    fn it_should_resolved_raw_ressources() {
        let mut variables = HashMap::<String, ResolvedResource>::new();

        variables.insert(
            "stringvar".to_string(),
            ResolvedResourceBuilder::default()
                .build_as_string("hello")
                .unwrap(),
        );
        variables.insert(
            "numbervar".to_string(),
            ResolvedResourceBuilder::default()
                .build_as_number(42.0)
                .unwrap(),
        );

        let raw_string_ref = RawResourceBuilder::default()
            .ctx_variables(Some(variables.clone()))
            .identifier(Some("var01".to_string()))
            .build_as_reference("stringvar")
            .unwrap();

        let resolved_string_ref = raw_string_ref.try_resolve().unwrap();
        assert_eq!(resolved_string_ref.identifier, Some("var01".to_string()));
        assert_eq!(
            resolved_string_ref.value,
            ResolvedResourceValue::String("hello".to_string())
        );

        let raw_number_ref = RawResourceBuilder::default()
            .ctx_variables(Some(variables))
            .identifier(Some("var02".to_string()))
            .build_as_reference("numbervar")
            .unwrap();

        let resolved_number_ref = raw_number_ref.try_resolve().unwrap();
        assert_eq!(resolved_number_ref.identifier, Some("var02".to_string()));
        assert_eq!(
            resolved_number_ref.value,
            ResolvedResourceValue::Number(42.0)
        );
    }

    #[test]
    fn it_should_resolved_raw_metas() {
        let mut variables = HashMap::<String, ResolvedResource>::new();

        variables.insert(
            "numbervar".to_string(),
            ResolvedResourceBuilder::default()
                .build_as_number(42.0)
                .unwrap(),
        );

        let raw_meta_ref = RawResourceBuilder::default()
            .ctx_variables(Some(variables.clone()))
            .build_as_reference("numbervar")
            .unwrap();

        let raw_meta_number = RawResourceBuilder::default()
            .ctx_variables(Some(variables.clone()))
            .build_as_number(123.0)
            .unwrap();

        let raw_string_with_metas = RawResourceBuilder::default()
            .ctx_variables(Some(variables))
            .identifier(Some("var01".to_string()))
            .metas(vec![raw_meta_ref, raw_meta_number])
            .build_as_string("hello")
            .unwrap();

        let resolved_resource = raw_string_with_metas.try_resolve().unwrap();

        assert_eq!(resolved_resource.identifier, Some("var01".to_string()));
        assert_eq!(
            resolved_resource.value,
            ResolvedResourceValue::String("hello".to_string())
        );

        let metas = resolved_resource.metas;
        assert_eq!(metas.len(), 2);

        let first_meta = metas.get(0).unwrap();
        assert_eq!(first_meta.identifier, None);
        assert_eq!(first_meta.value, ResolvedResourceValue::Number(42.0));

        let second_meta = metas.get(1).unwrap();
        assert_eq!(second_meta.identifier, None);
        assert_eq!(second_meta.value, ResolvedResourceValue::Number(123.0));
    }
}
