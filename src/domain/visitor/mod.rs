use super::{
    ast,
    resolution::{
        RawResource, RawTransformation, ResourceContextBuilder, ResourceList, TransformList,
        VariablesMap,
    },
};
use crate::{domain::resolution::Resolvable, result::AppError};
use crate::{random::get_random_uf8, result::AppResult};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

pub struct AstVisitor<'a> {
    pub resolver: &'a dyn crate::ports::ResolverPort,
}
impl<'a> AstVisitor<'a> {
    pub fn new(resolver: &'a impl crate::ports::ResolverPort) -> Self {
        Self { resolver: resolver }
    }
}
impl AstVisitor<'_> {
    pub fn visit(&self, val: ast::Value) -> AppResult<(ResourceList, TransformList)> {
        let build = ResourceContextBuilder::default();
        self.visit_value(val, build)
    }

    fn visit_value(
        &self,
        val: ast::Value,
        build: ResourceContextBuilder,
    ) -> AppResult<(ResourceList, TransformList)> {
        let visit_result = match val {
            ast::Value::String(s) => self.visit_string(s, build)?,
            ast::Value::Number(nb) => self.visit_number(nb, build)?,
            ast::Value::Reference(reference) => self.visit_reference(reference, build)?,
            ast::Value::Text(text) => self.visit_text(text, build)?,
            ast::Value::Object(ast::Object(elems)) => {
                let result = elems
                    .into_par_iter()
                    .map(|elem| match elem {
                        ast::ObjectElem::Declaration(v) => self.visit_declaration(v, build.clone()),
                        ast::ObjectElem::Import(i) => self.visit_import(i, build.clone()),
                    })
                    .collect::<AppResult<Vec<_>>>()?;

                let result: (ResourceList, TransformList) =
                    result.into_iter().fold((vec![], vec![]), |acc, elem| {
                        let (mut acc_resources, mut acc_transforms) = acc;
                        let (elem_resources, elem_transforms) = elem;
                        acc_resources.extend(elem_resources);
                        acc_transforms.extend(elem_transforms);
                        (acc_resources, acc_transforms)
                    });

                result
            }
        };

        Ok(visit_result)
    }

    fn visit_declaration(
        &self,
        val: ast::Declaration,
        build: ResourceContextBuilder,
    ) -> AppResult<(ResourceList, TransformList)> {
        let ast::Declaration {
            identifier,
            value,
            metas,
            mark,
        } = val;

        let metas = match metas {
            Some(m) => m
                .0
                .into_iter()
                .map(|m| {
                    let meta_build = build.clone();
                    match m {
                        ast::Meta::String(ast::String(s)) => Ok(meta_build.build_as_string(&s)?),
                        ast::Meta::Number(ast::Number(nb)) => Ok(meta_build.build_as_number(nb)?),
                        ast::Meta::Reference(reference) => {
                            Ok(meta_build.build_as_reference(reference.get_var_name())?)
                        }
                    }
                })
                .collect::<AppResult<Vec<RawResource>>>()?,
            None => vec![],
        };

        let build = build
            .metas(metas)
            .identifier(Some(identifier.0.clone()))
            .try_append_ctx_path(Some(identifier.0))?;

        let updated_build = match mark {
            ast::DeclarationMark::Direct(_) => build,
            ast::DeclarationMark::Uniq(_) => {
                let random_key = get_random_uf8(10)?;
                build.try_append_ctx_path(Some(random_key))?
            }
        };

        self.visit_value(value, updated_build)
    }

    fn visit_import(
        &self,
        val: ast::Import,
        build: ResourceContextBuilder,
    ) -> AppResult<(ResourceList, TransformList)> {
        let ast::Import {
            import_config,
            import_id,
            import_mark,
        } = val;

        // Récupérer le fichier + name
        let ast::File {
            transforms: file_transforms,
            value: file_value,
            name,
            ..
        } = ast::File::try_from(self.resolver.read(&import_id.0)?.as_str())?;

        // Changer le context build
        let append_new_path: Box<
            dyn FnOnce(ResourceContextBuilder) -> Result<ResourceContextBuilder, AppError>,
        > = match (import_mark, name) {
            (ast::ImportMark::Anon(ast::ImportAnonMark(custom_id)), _) => {
                Box::new(|build: ResourceContextBuilder| build.try_append_ctx_path(custom_id))
            }
            (ast::ImportMark::Named(ast::ImportNamedMark(custom_id)), name) => {
                Box::new(|build: ResourceContextBuilder| {
                    let name = name.map(|name| name.0);
                    build
                        .try_append_ctx_path(custom_id)?
                        .try_append_ctx_path(name)
                })
            }
            (ast::ImportMark::Uniq(ast::ImportUniqMark(custom_id)), _) => {
                Box::new(|build: ResourceContextBuilder| {
                    let random_id = get_random_uf8(10)?;
                    build
                        .try_append_ctx_path(custom_id)?
                        .try_append_ctx_path(Some(random_id))
                })
            }
        };

        let build = append_new_path(build)?;

        // Récupérer les variables et les imports associées
        let (mut variables_acc, mut resource_acc, mut trans_acc) = (
            VariablesMap::new(),
            ResourceList::new(),
            TransformList::new(),
        );

        let mut add_variable = |variable: ast::ImportVariable| -> AppResult<()> {
            let ast::ImportVariable { identifier, value } = variable;
            let (sub_resource_map, _) = self.visit_value(value, build.clone())?;
            for resource in sub_resource_map {
                variables_acc.insert(identifier.0.clone(), resource.try_resolve()?);
            }
            Ok(())
        };

        let mut add_resource_and_transform = |import: ast::Import| -> AppResult<()> {
            let (sub_resources, sub_transforms) = self.visit_import(import, build.clone())?;
            resource_acc.extend(sub_resources);
            trans_acc.extend(sub_transforms);
            Ok(())
        };

        import_config.into_iter().try_for_each(|elem| {
            match elem {
                ast::ImportConfig::Variable(var) => add_variable(var)?,
                ast::ImportConfig::Import(i) => add_resource_and_transform(i)?,
            }
            AppResult::Ok(())
        })?;

        // Résoudre les transformations et les ressources
        let build = build.ctx_variables(variables_acc.clone());

        if let Some(t) = file_transforms {
            let transforms = t
                .into_iter()
                .map(|t| {
                    RawTransformation::from_ast(t, build.clone()).map(|v| v.unwrap_or_default())
                })
                .flat_map(|r| match r {
                    Ok(vec) => vec.into_iter().map(|item| Ok(item)).collect(),
                    Err(er) => vec![Err(er)],
                })
                .collect::<AppResult<Vec<_>>>()?;
            trans_acc.extend(transforms);
        }

        if let Some(value) = file_value {
            let (sub_resources, sub_transforms) = self.visit_value(value, build)?;
            trans_acc.splice(0..0, sub_transforms);
            resource_acc.splice(0..0, sub_resources);
        }

        Ok((resource_acc, trans_acc))
    }

    fn visit_string(
        &self,
        val: ast::String,
        build: ResourceContextBuilder,
    ) -> AppResult<(ResourceList, TransformList)> {
        Ok((vec![build.build_as_string(&val.0)?], vec![]))
    }

    fn visit_text(
        &self,
        val: ast::Text,
        build: ResourceContextBuilder,
    ) -> AppResult<(ResourceList, TransformList)> {
        Ok((vec![build.build_as_string(&val.0)?], vec![]))
    }

    fn visit_number(
        &self,
        val: ast::Number,
        build: ResourceContextBuilder,
    ) -> AppResult<(ResourceList, TransformList)> {
        Ok((vec![build.build_as_number(val.0)?], vec![]))
    }

    fn visit_reference(
        &self,
        val: ast::Ref,
        build: ResourceContextBuilder,
    ) -> AppResult<(ResourceList, TransformList)> {
        Ok((vec![build.build_as_reference(&val.get_var_name())?], vec![]))
    }
}
