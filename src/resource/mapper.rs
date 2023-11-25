use super::resources::{RawResources, ResolvedResources, Resource, ResourceContext};
use crate::{ast, ports::TTLInputPort, utils::result::AppResult};
use indexmap::IndexMap;
use rayon::prelude::*;
use std::collections::HashMap;

pub struct ResourceMapper<'a> {
    input_port: &'a dyn TTLInputPort,
}

impl<'a> ResourceMapper<'a> {
    pub fn new(input_port: &'a impl TTLInputPort) -> Self {
        Self { input_port }
    }

    pub fn ast_values_to_resource_map(
        &self,
        val: ast::Value,
        identifier: Option<String>,
        ctx: ResourceContext,
    ) -> AppResult<IndexMap<String, RawResources>> {
        let mut resource_map = IndexMap::<String, RawResources>::new();

        match val {
            ast::Value::String(ast::StringLit(str)) => {
                let path = ctx.path.clone();
                let resource = RawResources::String(Resource::new(str, identifier, ctx));
                resource_map.insert(path.unwrap_or_default(), resource);
            }
            ast::Value::Number(ast::Number(nb)) => {
                let path = ctx.path.clone();
                let resource = RawResources::Number(Resource::new(nb, identifier, ctx));
                resource_map.insert(path.unwrap_or_default(), resource);
            }
            ast::Value::Reference(ast::Ref(id)) => {
                let path = ctx.path.clone();
                let resource = RawResources::Reference(Resource::new(id, identifier, ctx));
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
                                    variables_map.insert(k, v.try_compute_references()?);
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
}
