use crate::{
    domain::resolution::{ResolvedResource, ResolvedTransformation},
    result::{AppError, AppResult},
};
use indexmap::IndexMap;

use super::transformable_list::TransformableList;

pub fn apply_transforms(
    resources: IndexMap<String, ResolvedResource>,
    transforms: Vec<ResolvedTransformation>,
    layers: Vec<&str>,
) -> AppResult<IndexMap<String, ResolvedResource>> {
    let mut transforms_by_ordered_layer = Vec::new();

    for layer in layers {
        let layer_transforms: Vec<&ResolvedTransformation> = transforms
            .iter()
            .filter(|t| t.layer.as_deref() == Some(&layer))
            .collect();

        if !layer_transforms.is_empty() {
            transforms_by_ordered_layer.push((layer, layer_transforms));
        }
    }

    let mut transformable_list: TransformableList = resources.into();

    for (_, transforms) in transforms_by_ordered_layer {
        transforms.into_iter().try_for_each(|transform| {
            let rule = &transform.rule;
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
    use crate::domain::resolution::{
        RawTransformation, Resolvable, ResolvedResourceBuilder, ResolvedResourceValue,
    };

    use super::*;
    use indexmap::IndexMap;
    use std::collections::HashMap;

    #[test]
    fn it_should_transform_ressources() {
        let mut ctx_variables = HashMap::<String, ResolvedResource>::new();
        let ctx_path = Some("root".to_string());

        ctx_variables.insert(
            "factor".to_string(),
            ResolvedResourceBuilder::default()
                .identifier(Some("factor".to_string()))
                .build_as_number(2.0)
                .unwrap(),
        );

        /* let context = ResolutionContext {
            variables: Some(ctx_variables),
            path: Some("root".to_string()),
        }; */

        let transform_x_1 = RawTransformation {
            rule: "$.x += 5".to_string(),
            layer: Some("FIRST_LAYER".to_string()),
            ctx_path: ctx_path.clone(),
            ctx_variables: Some(ctx_variables.clone()),
        };

        let transform_x_2 = RawTransformation {
            rule: "$.x *= $factor".to_string(),
            layer: Some("SECOND_LAYER".to_string()),
            ctx_path: ctx_path.clone(),
            ctx_variables: Some(ctx_variables.clone()),
        };

        let layers = vec!["FIRST_LAYER", "SECOND_LAYER"];

        let transforms = vec![transform_x_1, transform_x_2];

        let mut resources = IndexMap::<String, ResolvedResource>::new();
        resources.insert(
            "root.x".to_string(),
            ResolvedResourceBuilder::default()
                .identifier(Some("x".to_string()))
                .build_as_number(7.0)
                .unwrap(),
        );

        let transformed_resources =
            apply_transforms(resources, transforms.try_resolve().unwrap(), layers).unwrap();

        assert_eq!(transformed_resources.len(), 1);
        let x = transformed_resources.get("root.x").unwrap();
        match x.value {
            ResolvedResourceValue::Number(n) => assert_eq!(n, 24.0),
            _ => panic!("Unexpected resource type"),
        }
    }
}
