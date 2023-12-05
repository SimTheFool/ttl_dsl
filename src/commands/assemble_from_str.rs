use crate::domain::ast::{self};
use crate::domain::resolution::Resolvable;
use crate::domain::resolution::{RawResource, ResolutionContext, ResolvedResource};
use crate::domain::resolution::{RawResourceBuilder, RawTransformation};
use crate::domain::transformation::apply_transforms;
use crate::utils::result::AppResult;
use indexmap::IndexMap;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashMap;

pub struct AssembleFromStr<'a> {
    pub resolver: &'a dyn crate::ports::ResolverPort,
    pub config: &'a dyn crate::ports::ConfigProviderPort,
}

impl<'a> AssembleFromStr<'a> {
    fn ast_values_to_resource_map(
        &self,
        val: ast::Value,
        identifier: Option<String>,
        metas: Option<ast::Metas>,
        ctx: ResolutionContext,
    ) -> AppResult<(
        IndexMap<String, RawResource>,
        Option<Vec<RawTransformation>>,
    )> {
        let metas = match metas {
            Some(m) => m
                .0
                .into_iter()
                .map(|m| {
                    let meta_build = RawResourceBuilder::default().context(ctx.clone());

                    match m {
                        ast::Meta::String(ast::StringLit(s)) => Ok(meta_build.build_as_string(&s)?),
                        ast::Meta::Number(ast::Number(nb)) => Ok(meta_build.build_as_number(nb)?),
                        ast::Meta::Reference(ast::Ref(id)) => {
                            Ok(meta_build.build_as_reference(&id)?)
                        }
                    }
                })
                .collect::<AppResult<Vec<RawResource>>>()?,
            None => vec![],
        };

        let mut resource_map = IndexMap::<String, RawResource>::new();
        let mut transforms = Vec::<RawTransformation>::new();

        let resource_path = ctx.path.clone();
        let resource_build = RawResourceBuilder::default()
            .context(ctx.clone())
            .metas(metas)
            .identifier(identifier);

        match val {
            ast::Value::String(ast::StringLit(str)) => {
                resource_map.insert(
                    resource_path.unwrap_or_default(),
                    resource_build.build_as_string(&str)?,
                );
            }
            ast::Value::Number(ast::Number(nb)) => {
                resource_map.insert(
                    resource_path.unwrap_or_default(),
                    resource_build.build_as_number(nb)?,
                );
            }
            ast::Value::Reference(ast::Ref(id)) => {
                resource_map.insert(
                    resource_path.unwrap_or_default(),
                    resource_build.build_as_reference(&id)?,
                );
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

                            let context = ResolutionContext {
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
                            let mut variables_map = HashMap::<String, ResolvedResource>::new();
                            match &ctx.path {
                                None => {}
                                Some(path) => {
                                    variables_map.insert(
                                        "".to_string(),
                                        RawResourceBuilder::default()
                                            .context(ctx.clone())
                                            .build_as_string(&path)?
                                            .try_resolve()?,
                                    );
                                }
                            };

                            for declaration in declarations {
                                let resource_path =
                                    match (&ctx.path, &declaration.identifier.0.clone()) {
                                        (None, id) => id.clone(),
                                        (Some(base), id) => format!("{}.{}", base, id),
                                    };

                                let context = ResolutionContext {
                                    variables: ctx.variables.clone(),
                                    path: Some(resource_path),
                                };

                                let (sub_resource_map, _) = self.ast_values_to_resource_map(
                                    declaration.value,
                                    Some(declaration.identifier.0),
                                    None,
                                    context,
                                )?;

                                for (_, v) in sub_resource_map {
                                    variables_map.insert(
                                        v.identifier.clone().unwrap_or_default(),
                                        v.try_resolve()?,
                                    );
                                }
                            }

                            let import_ctx = ResolutionContext {
                                variables: Some(variables_map),
                                path: ctx.path.clone(),
                            };

                            let import = self.resolver.read(&path.0)?;
                            let file = ast::File::try_from(import.as_str())?;

                            let file_transforms = match file.transforms {
                                None => None,
                                Some(t) => {
                                    let transforms = t
                                        .into_iter()
                                        .flat_map(|t| {
                                            RawTransformation::from_ast(t, import_ctx.clone())
                                                .unwrap_or_default()
                                        })
                                        .collect::<Vec<RawTransformation>>();
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
                            IndexMap<String, RawResource>,
                            Option<Vec<RawTransformation>>,
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

    pub fn execute(&self, file_str: &str) -> AppResult<Vec<ResolvedResource>> {
        let ast::File {
            value, transforms, ..
        } = ast::File::try_from(file_str)?;

        let transforms = transforms.map(|t| {
            t.into_iter()
                .flat_map(|t| {
                    RawTransformation::from_ast(t, ResolutionContext::default()).unwrap_or_default()
                })
                .collect::<Vec<RawTransformation>>()
        });

        let (resources_map, inner_transforms) = self.ast_values_to_resource_map(
            value,
            None,
            None,
            ResolutionContext {
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
                IndexMap::<String, ResolvedResource>::new(),
                |mut map, kv| {
                    let (k, v) = kv?;
                    map.insert(k, v);
                    AppResult::Ok(map)
                },
            )?;

        let layers = self.config.get_transform_layers()?;

        let resources_map = match transforms {
            None => Ok(resources_map),
            Some(t) => apply_transforms(resources_map, t.try_resolve()?, layers),
        }?;

        Ok(resources_map.into_iter().map(|(_, v)| v).collect())
    }
}
