use super::resources::{RawResources, ResolvedResources, Resource, ResourceContext};
use crate::{ast, ports::TTLInputPort, utils::result::AppResult};
use indexmap::IndexMap;
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
            ast::Value::String(s) => {
                let path = ctx.path.clone();
                let resource = RawResources::String(Resource::new(s.0, identifier, ctx));
                resource_map.insert(path.unwrap_or("".to_string()), resource);
            }
            ast::Value::Number(n) => {
                let path = ctx.path.clone();
                let resource = RawResources::Number(Resource::new(n.0, identifier, ctx));
                resource_map.insert(path.unwrap_or("".to_string()), resource);
            }
            ast::Value::Reference(r) => {
                let path = ctx.path.clone();
                let resource = RawResources::Reference(Resource::new(r.0, identifier, ctx));
                resource_map.insert(path.unwrap_or("".to_string()), resource);
            }
            ast::Value::Object(o) => {
                for elem in o.0 {
                    match elem {
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

                            resource_map.extend(sub_resource_map);
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

                            resource_map.extend(sub_resource_map);
                        }
                    };
                }
            }
        };

        Ok(resource_map)
    }
}
