use crate::domain::ast::{self};
use crate::domain::resource::{RawNumberBuilder, RawStringBuilder};
use crate::domain::transformation::{self, apply_transforms};
use crate::{
    domain::resource::{RawResources, ResolvedResources, ResourceContext, TryResolveResource},
    utils::result::AppResult,
};
use indexmap::IndexMap;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashMap;

pub struct AssembleFromStr<'a> {
    pub input_port: &'a dyn crate::ports::ResolverPort,
    pub config_port: &'a dyn crate::ports::ConfigProviderPort,
}

impl<'a> AssembleFromStr<'a> {
    fn ast_values_to_resource_map(
        &self,
        val: ast::Value,
        identifier: Option<String>,
        metas: Option<ast::Metas>,
        ctx: ResourceContext,
    ) -> AppResult<(
        IndexMap<String, RawResources>,
        Option<Vec<transformation::Transform>>,
    )> {
        let metas = match metas {
            Some(m) => Some(
                m.0.into_iter()
                    .map(|m| match m {
                        ast::Meta::String(ast::StringLit(s)) => Ok(RawStringBuilder::default()
                            .context(ctx.clone())
                            .value(s)
                            .build_string_resource()?),
                        ast::Meta::Number(ast::Number(nb)) => Ok(RawNumberBuilder::default()
                            .context(ctx.clone())
                            .value(nb)
                            .build_number_resource()?),
                        ast::Meta::Reference(ast::Ref(id)) => Ok(RawStringBuilder::default()
                            .context(ctx.clone())
                            .value(id)
                            .build_reference_resource()?),
                    })
                    .collect::<AppResult<Vec<RawResources>>>()?,
            ),
            None => None,
        };

        let mut resource_map = IndexMap::<String, RawResources>::new();
        let mut transforms = Vec::<transformation::Transform>::new();

        match val {
            ast::Value::String(ast::StringLit(str)) => {
                let path = ctx.path.clone();
                let resource = RawStringBuilder::default()
                    .context(ctx)
                    .identifier(identifier)
                    .value(str)
                    .metas(metas)
                    .build_string_resource()?;
                resource_map.insert(path.unwrap_or_default(), resource);
            }
            ast::Value::Number(ast::Number(nb)) => {
                let path = ctx.path.clone();
                let resource = RawNumberBuilder::default()
                    .context(ctx)
                    .identifier(identifier)
                    .value(nb)
                    .metas(metas)
                    .build_number_resource()?;
                resource_map.insert(path.unwrap_or_default(), resource);
            }
            ast::Value::Reference(ast::Ref(id)) => {
                let path = ctx.path.clone();
                let resource = RawStringBuilder::default()
                    .context(ctx)
                    .identifier(identifier)
                    .value(id)
                    .metas(metas)
                    .build_reference_resource()?;
                resource_map.insert(path.unwrap_or_default(), resource);
            }
            ast::Value::Object(ast::Object(elems)) => {
                let object_resources_and_transforms = elems
                    .into_par_iter()
                    .map(|elem| match elem {
                        ast::ObjectElem::Declaration(v) => {
                            let resource_path = match (&ctx.path, &v.identifier.0) {
                                (None, id) => id.clone(),
                                (Some(base), id) => format!("{}.{}", base, id),
                            };

                            let context = ResourceContext {
                                variables: ctx.variables.clone(),
                                path: Some(resource_path),
                            };

                            self.ast_values_to_resource_map(
                                v.value,
                                Some(v.identifier.0),
                                v.metas,
                                context,
                            )
                        }
                        ast::ObjectElem::Import(ast::Import { declarations, path }) => {
                            let mut variables_map = HashMap::<String, ResolvedResources>::new();
                            for declaration in declarations {
                                let context = ResourceContext {
                                    variables: ctx.variables.clone(),
                                    path: Some(declaration.identifier.0.clone()),
                                };

                                let (sub_resource_map, _) = self.ast_values_to_resource_map(
                                    declaration.value,
                                    Some(declaration.identifier.0),
                                    None,
                                    context,
                                )?;

                                for (k, v) in sub_resource_map {
                                    variables_map.insert(k, v.try_resolve()?);
                                }
                            }

                            let import_ctx = ResourceContext {
                                variables: Some(variables_map),
                                path: ctx.path.clone(),
                            };

                            let import = self.input_port.read(&path.0)?;
                            let file = ast::File::try_from(import.as_str())?;

                            let file_transforms = match file.transforms {
                                None => None,
                                Some(t) => {
                                    let transforms = t
                                        .into_iter()
                                        .flat_map(|t| {
                                            transformation::Transform::from_ast(
                                                t,
                                                import_ctx.clone(),
                                            )
                                            .unwrap_or_default()
                                        })
                                        .collect::<Vec<transformation::Transform>>();
                                    Some(transforms)
                                }
                            };

                            let (sub_maps, sub_transforms) = self
                                .ast_values_to_resource_map(file.value, None, None, import_ctx)?;

                            let transforms = match (file_transforms, sub_transforms) {
                                (None, None) => None,
                                (Some(t), None) => Some(t),
                                (None, Some(t)) => Some(t),
                                (Some(t1), Some(t2)) => {
                                    let mut t1 = t1;
                                    t1.extend(t2);
                                    Some(t1)
                                }
                            };

                            Ok((sub_maps, transforms))
                        }
                    })
                    .collect::<AppResult<
                        Vec<(
                            IndexMap<String, RawResources>,
                            Option<Vec<transformation::Transform>>,
                        )>,
                    >>()?;

                for (resources, transformations) in object_resources_and_transforms {
                    resource_map.extend(resources);
                    transforms.extend(transformations.unwrap_or_default());
                }
            }
        };

        Ok((resource_map, Some(transforms)))
    }

    pub fn execute(&self, file_str: &str) -> AppResult<Vec<ResolvedResources>> {
        let ast::File {
            value, transforms, ..
        } = ast::File::try_from(file_str)?;

        let transforms = transforms.map(|t| {
            t.into_iter()
                .flat_map(|t| {
                    transformation::Transform::from_ast(t, ResourceContext::default())
                        .unwrap_or_default()
                })
                .collect::<Vec<transformation::Transform>>()
        });

        let (resources_map, inner_transforms) = self.ast_values_to_resource_map(
            value,
            None,
            None,
            ResourceContext {
                variables: None,
                path: None,
            },
        )?;

        let transforms = match (transforms, inner_transforms) {
            (Some(t), Some(it)) => {
                let mut t = t;
                t.extend(it);
                Some(t)
            }
            (x, None) => x,
            (None, x) => x,
        };

        let resources_map = resources_map
            .into_iter()
            .map(|(k, v)| {
                let new_kv = (k.clone(), v.try_resolve()?);
                return AppResult::Ok(new_kv);
            })
            .try_fold(
                IndexMap::<String, ResolvedResources>::new(),
                |mut map, kv| {
                    let (k, v) = kv?;
                    map.insert(k, v);
                    AppResult::Ok(map)
                },
            )?;

        let layers = self.config_port.get_transform_layers()?;

        let resources_map = match transforms {
            None => Ok(resources_map),
            Some(t) => apply_transforms(resources_map, t, layers),
        }?;

        Ok(resources_map.into_iter().map(|(_, v)| v).collect())
    }
}
