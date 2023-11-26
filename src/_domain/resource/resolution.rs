use super::{state, RawResources, ResolvedResources, ResourceBuilder};
use crate::result::{AppError, AppResult};

pub trait TryResolveResource {
    type ReturnType;
    fn try_resolve(self) -> AppResult<Self::ReturnType>;
}

impl TryResolveResource for Vec<RawResources> {
    type ReturnType = Vec<ResolvedResources>;
    fn try_resolve(self) -> AppResult<Self::ReturnType> {
        self.into_iter()
            .map(|raw_resource| raw_resource.try_resolve())
            .collect()
    }
}

impl TryResolveResource for RawResources {
    type ReturnType = ResolvedResources;
    fn try_resolve(self) -> AppResult<Self::ReturnType> {
        let resource = match self {
            Self::String(s) => ResolvedResources::String(
                ResourceBuilder::<String, state::Resolved>::default()
                    .context_box(s.context)
                    .identifier(s.identifier)
                    .value(s.value)
                    .metas(s.metas.map(|x| x.try_resolve()).transpose()?)
                    .build()?,
            ),
            Self::Number(n) => ResolvedResources::Number(
                ResourceBuilder::<f64, state::Resolved>::default()
                    .context_box(n.context)
                    .identifier(n.identifier)
                    .value(n.value)
                    .metas(n.metas.map(|x| x.try_resolve()).transpose()?)
                    .build()?,
            ),
            Self::Reference(r) => {
                let variable_name = r.value;
                let ref_path = r.context.path.clone();

                let variable_ctx = match &r.context.variables {
                    Some(x) => x,
                    None => Err(AppError::String(format!(
                        "No variables for {}",
                        ref_path.unwrap_or("".to_string())
                    )))?,
                };

                let variable_resource = variable_ctx
                    .get(&variable_name)
                    .ok_or(AppError::String(format!(
                        "No variables {variable_name} found"
                    )))?
                    .clone();

                return match variable_resource {
                    ResolvedResources::String(x) => {
                        let resource = ResolvedResources::String(
                            ResourceBuilder::<String, state::Resolved>::default()
                                .context_box(r.context)
                                .identifier(r.identifier)
                                .value(x.value.clone())
                                .metas(r.metas.map(|x| x.try_resolve()).transpose()?)
                                .build()?,
                        );

                        Ok(resource)
                    }
                    ResolvedResources::Number(x) => {
                        let resource = ResolvedResources::Number(
                            ResourceBuilder::<f64, state::Resolved>::default()
                                .context_box(r.context)
                                .identifier(r.identifier)
                                .value(x.value.clone())
                                .metas(r.metas.map(|x| x.try_resolve()).transpose()?)
                                .build()?,
                        );

                        Ok(resource)
                    }
                };
            }
        };

        Ok(resource)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::resource::{
        state::{Raw, Resolved},
        ResourceBuilder, ResourceContext,
    };

    use super::{ResolvedResources, TryResolveResource};
    use std::collections::HashMap;

    #[test]
    fn it_should_resolved_raw_ressources() {
        let mut variables = HashMap::<String, ResolvedResources>::new();

        variables.insert(
            "stringvar".to_string(),
            ResolvedResources::String(
                ResourceBuilder::<String, Resolved>::default()
                    .value("hello".to_string())
                    .build()
                    .unwrap(),
            ),
        );
        variables.insert(
            "numbervar".to_string(),
            ResolvedResources::Number(
                ResourceBuilder::<f64, Resolved>::default()
                    .value(42.0)
                    .build()
                    .unwrap(),
            ),
        );

        let ctx = Box::new(ResourceContext {
            variables: Some(variables),
            path: None,
        });

        let raw_string_ref = super::RawResources::Reference(
            ResourceBuilder::<String, Raw>::default()
                .context_box(ctx.clone())
                .identifier(Some("var01".to_string()))
                .value("stringvar".to_string())
                .build()
                .unwrap(),
        );

        let resolved_string_ref = raw_string_ref.try_resolve().unwrap();
        match resolved_string_ref {
            super::ResolvedResources::String(x) => {
                assert_eq!(x.identifier, Some("var01".to_string()));
                assert_eq!(x.value, "hello");
            }
            _ => panic!("Should be a reference"),
        }

        let raw_number_ref = super::RawResources::Reference(
            ResourceBuilder::<String, Raw>::default()
                .context_box(ctx.clone())
                .identifier(Some("var02".to_string()))
                .value("numbervar".to_string())
                .build()
                .unwrap(),
        );
        let resolved_number_ref = raw_number_ref.try_resolve().unwrap();
        match resolved_number_ref {
            super::ResolvedResources::Number(x) => {
                assert_eq!(x.identifier, Some("var02".to_string()));
                assert_eq!(x.value, 42.0);
            }
            _ => panic!("Should be a reference"),
        }
    }

    #[test]
    fn it_should_resolved_raw_metas() {
        let mut variables = HashMap::<String, ResolvedResources>::new();
        variables.insert(
            "numbervar".to_string(),
            ResolvedResources::Number(
                ResourceBuilder::<f64, Resolved>::default()
                    .value(42.42)
                    .build()
                    .unwrap(),
            ),
        );

        let ctx = Box::new(ResourceContext {
            variables: Some(variables),
            path: None,
        });

        let raw_meta_ref = super::RawResources::Reference(
            ResourceBuilder::<String, Raw>::default()
                .context_box(ctx.clone())
                .value("numbervar".to_string())
                .build()
                .unwrap(),
        );

        let raw_meta_number = super::RawResources::Number(
            ResourceBuilder::<f64, Raw>::default()
                .context_box(ctx.clone())
                .value(123.0)
                .build()
                .unwrap(),
        );

        let raw_string_with_metas = super::RawResources::String(
            ResourceBuilder::<String, Raw>::default()
                .context_box(ctx.clone())
                .identifier(Some("var01".to_string()))
                .value("hello".to_string())
                .metas(Some(vec![raw_meta_ref, raw_meta_number]))
                .build()
                .unwrap(),
        );

        let resolved_resource = raw_string_with_metas.try_resolve().unwrap();

        match resolved_resource {
            super::ResolvedResources::String(x) => {
                assert_eq!(x.identifier, Some("var01".to_string()));
                assert_eq!(x.value, "hello");

                let metas = x.metas.unwrap();
                assert_eq!(metas.len(), 2);

                match metas.get(0).unwrap() {
                    super::ResolvedResources::Number(x) => {
                        assert_eq!(x.identifier, None);
                        assert_eq!(x.value, 42.42);
                    }
                    _ => panic!("Should be a number"),
                }

                match metas.get(1).unwrap() {
                    super::ResolvedResources::Number(x) => {
                        assert_eq!(x.identifier, None);
                        assert_eq!(x.value, 123.0);
                    }
                    _ => panic!("Should be a number"),
                }
            }
            _ => panic!("Should be a string"),
        }
    }
}
