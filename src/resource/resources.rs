use std::collections::HashMap;

use crate::utils::result::{AppError, AppResult};

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Raw();

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Resolved();

#[derive(PartialEq, Debug, Clone, Default)]
pub struct ResourceContext {
    pub variables: Option<HashMap<String, ResolvedResources>>,
    pub path: Option<String>,
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Resource<T, U>
where
    T: Clone,
    U: Clone,
{
    pub context: Box<ResourceContext>,
    pub identifier: Option<String>,
    pub value: T,
    _phantom: std::marker::PhantomData<U>,
}

impl<T, U> Resource<T, U>
where
    T: Clone,
    U: Clone,
{
    pub fn new(value: T, identifier: Option<String>, ctx: ResourceContext) -> Self {
        Self {
            context: Box::new(ctx),
            identifier,
            value,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum RawResources {
    String(Resource<String, Raw>),
    Number(Resource<f64, Raw>),
    Reference(Resource<String, Raw>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum ResolvedResources {
    String(Resource<String, Resolved>),
    Number(Resource<f64, Resolved>),
}

impl RawResources {
    pub fn try_compute_references(self) -> AppResult<ResolvedResources> {
        let resource = match self {
            Self::String(s) => ResolvedResources::String(Resource::<String, Resolved> {
                context: s.context,
                identifier: s.identifier,
                value: s.value,
                _phantom: std::marker::PhantomData,
            }),
            Self::Number(n) => ResolvedResources::Number(Resource::<f64, Resolved> {
                context: n.context,
                identifier: n.identifier,
                value: n.value,
                _phantom: std::marker::PhantomData,
            }),
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
                        let resource = ResolvedResources::String(Resource::<String, Resolved> {
                            context: r.context,
                            identifier: r.identifier,
                            value: x.value.clone(),
                            _phantom: std::marker::PhantomData,
                        });

                        Ok(resource)
                    }
                    ResolvedResources::Number(x) => {
                        let resource = ResolvedResources::Number(Resource::<f64, Resolved> {
                            context: r.context,
                            identifier: r.identifier,
                            value: x.value.clone(),
                            _phantom: std::marker::PhantomData,
                        });

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
    use super::{Resolved, ResolvedResources, Resource};
    use std::collections::HashMap;

    #[test]
    fn it_should_resolved_raw_ressources() {
        let mut variables = HashMap::<String, ResolvedResources>::new();
        variables.insert(
            "stringvar".to_string(),
            ResolvedResources::String(Resource::<String, Resolved> {
                value: "hello".to_string(),
                ..Resource::<String, Resolved>::default()
            }),
        );
        variables.insert(
            "numbervar".to_string(),
            ResolvedResources::Number(Resource::<f64, Resolved> {
                value: 42.0,
                ..Resource::<f64, Resolved>::default()
            }),
        );

        let ctx = Box::new(super::ResourceContext {
            variables: Some(variables),
            path: None,
        });

        let raw_string_ref =
            super::RawResources::Reference(super::Resource::<String, super::Raw> {
                context: ctx.clone(),
                identifier: Some("var01".to_string()),
                value: "stringvar".to_string(),
                _phantom: std::marker::PhantomData,
            });

        let resolved_string_ref = raw_string_ref.try_compute_references().unwrap();
        match resolved_string_ref {
            super::ResolvedResources::String(x) => {
                assert_eq!(x.identifier, Some("var01".to_string()));
                assert_eq!(x.value, "hello");
            }
            _ => panic!("Should be a reference"),
        }

        let raw_number_ref =
            super::RawResources::Reference(super::Resource::<String, super::Raw> {
                context: ctx.clone(),
                identifier: Some("var02".to_string()),
                value: "numbervar".to_string(),
                _phantom: std::marker::PhantomData,
            });
        let resolved_number_ref = raw_number_ref.try_compute_references().unwrap();
        match resolved_number_ref {
            super::ResolvedResources::Number(x) => {
                assert_eq!(x.identifier, Some("var02".to_string()));
                assert_eq!(x.value, 42.0);
            }
            _ => panic!("Should be a reference"),
        }
    }
}
