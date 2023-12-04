use crate::{
    domain::resource::ResolvedResources,
    result::{AppError, AppResult},
};
use indexmap::IndexMap;

use super::{transformable_list::TransformableList, Transform};

pub fn apply_transforms(
    resources: IndexMap<String, ResolvedResources>,
    transforms: Vec<Transform>,
    layers: Vec<&str>,
) -> AppResult<IndexMap<String, ResolvedResources>> {
    let mut transforms_by_ordered_layer = Vec::new();

    for layer in layers {
        let layer_transforms: Vec<&Transform> = transforms
            .iter()
            .filter(|t| t.layer.as_deref() == Some(&layer))
            .collect();

        if !layer_transforms.is_empty() {
            transforms_by_ordered_layer.push((layer, layer_transforms));
        }
    }

    let mut transformable_list: TransformableList = resources.into();

    for (_, transforms) in transforms_by_ordered_layer {
        transforms.iter().try_for_each(|transform| {
            let rule = transform.get_resolved_rule()?;
            evalexpr::eval_with_context_mut(&rule, &mut transformable_list).map_err(|e| {
                AppError::String(format!("Error while evaluating rule {} : {}", rule, e))
            })?;
            AppResult::Ok(())
        })?;
    }

    Ok(transformable_list.into())
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use indexmap::IndexMap;

    use crate::domain::resource::{
        RawNumberBuilder, RawStringBuilder, ResolvedResources, ResourceContext, TryResolveResource,
    };

    use super::*;

    #[test]
    fn it_should_transform_ressources() {
        let mut variables = HashMap::<String, ResolvedResources>::new();

        variables.insert(
            "".to_string(),
            RawStringBuilder::default()
                .identifier(Some("".to_string()))
                .value("root".to_string())
                .build_string_resource()
                .unwrap()
                .try_resolve()
                .unwrap(),
        );

        variables.insert(
            "factor".to_string(),
            RawNumberBuilder::default()
                .identifier(Some("factor".to_string()))
                .value(2.0)
                .build_number_resource()
                .unwrap()
                .try_resolve()
                .unwrap(),
        );

        let context = ResourceContext {
            variables: Some(variables),
            ..Default::default()
        };

        let transform_x_1 = Transform {
            context: Box::new(context.clone()),
            rule: "$.x += 5".to_string(),
            layer: Some("FIRST_LAYER".to_string()),
        };

        let transform_x_2 = Transform {
            context: Box::new(context.clone()),
            rule: "$.x *= $factor".to_string(),
            layer: Some("SECOND_LAYER".to_string()),
        };

        let layers = vec!["FIRST_LAYER", "SECOND_LAYER"];

        let transforms = vec![transform_x_1, transform_x_2];

        let mut resources = IndexMap::<String, ResolvedResources>::new();
        resources.insert(
            "root.x".to_string(),
            RawNumberBuilder::default()
                .identifier(Some("x".to_string()))
                .value(7.0)
                .build_number_resource()
                .unwrap()
                .try_resolve()
                .unwrap(),
        );

        let transformed_resources = apply_transforms(resources, transforms, layers).unwrap();

        assert_eq!(transformed_resources.len(), 1);
        let x = transformed_resources.get("root.x").unwrap();
        match x {
            ResolvedResources::Number(resource) => assert_eq!(resource.value, 24.0),
            _ => panic!("Unexpected resource type"),
        }
    }
}
