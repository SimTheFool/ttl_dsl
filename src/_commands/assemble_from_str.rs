use crate::domain::ast;
use crate::{
    domain::resource::{
        state::Raw, RawResources, ResolvedResources, ResourceBuilder, ResourceContext,
        TryResolveResource,
    },
    ports::TTLInputPort,
    utils::result::AppResult,
};
use indexmap::IndexMap;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashMap;

pub struct AssembleFromStr<'a> {
    pub input_port: &'a dyn TTLInputPort,
}

impl<'a> AssembleFromStr<'a> {
    fn ast_values_to_resource_map(
        &self,
        val: ast::Value,
        identifier: Option<String>,
        ctx: ResourceContext,
    ) -> AppResult<IndexMap<String, RawResources>> {
        let mut resource_map = IndexMap::<String, RawResources>::new();

        match val {
            ast::Value::String(ast::StringLit(str)) => {
                let path = ctx.path.clone();
                let resource = ResourceBuilder::<String, Raw>::default()
                    .context(ctx)
                    .identifier(identifier)
                    .value(str)
                    .build()?;
                let resource = RawResources::String(resource);
                resource_map.insert(path.unwrap_or_default(), resource);
            }
            ast::Value::Number(ast::Number(nb)) => {
                let path = ctx.path.clone();
                let resource = ResourceBuilder::<f64, Raw>::default()
                    .context(ctx)
                    .identifier(identifier)
                    .value(nb)
                    .build()?;
                let resource = RawResources::Number(resource);
                resource_map.insert(path.unwrap_or_default(), resource);
            }
            ast::Value::Reference(ast::Ref(id)) => {
                let path = ctx.path.clone();
                let resource = ResourceBuilder::<String, Raw>::default()
                    .context(ctx)
                    .identifier(identifier)
                    .value(id.clone())
                    .build()?;
                let resource = RawResources::Reference(resource);
                resource_map.insert(path.unwrap_or_default(), resource);
            }
            ast::Value::Object(ast::Object(elems)) => {
                let sub_resource_maps = elems
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

                            let sub_resource_map = self.ast_values_to_resource_map(
                                v.value,
                                Some(v.identifier.0),
                                context,
                            )?;

                            Ok(sub_resource_map)
                        }
                        ast::ObjectElem::Import(ast::Import { declarations, path }) => {
                            let mut variables_map = HashMap::<String, ResolvedResources>::new();
                            for declaration in declarations {
                                let context = ResourceContext {
                                    variables: ctx.variables.clone(),
                                    path: Some(declaration.identifier.0.clone()),
                                };

                                let sub_resource_map = self.ast_values_to_resource_map(
                                    declaration.value,
                                    Some(declaration.identifier.0),
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
                            let value = ast::File::try_from(import.as_str())?.value;

                            let sub_resource_map =
                                self.ast_values_to_resource_map(value, None, import_ctx)?;

                            Ok(sub_resource_map)
                        }
                    })
                    .collect::<AppResult<Vec<IndexMap<String, RawResources>>>>()?;

                let sub_resource_map = sub_resource_maps.into_iter().fold(
                    IndexMap::<String, RawResources>::new(),
                    |mut acc, sub_resource_map| {
                        acc.extend(sub_resource_map);
                        acc
                    },
                );

                resource_map.extend(sub_resource_map);
            }
        };

        Ok(resource_map)
    }

    pub fn execute(&self, file_str: &str) -> AppResult<Vec<ResolvedResources>> {
        let value = ast::File::try_from(file_str)?.value;

        let resources_map = self.ast_values_to_resource_map(
            value,
            None,
            ResourceContext {
                variables: None,
                path: None,
            },
        )?;

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

        Ok(resources_map.into_iter().map(|(_, v)| v).collect())
    }
}
