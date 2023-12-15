use super::{
    ast,
    resolution::{
        RawResource, RawResourceBuilder, RawTransformation, ResourceList, TransformList,
        VariablesMap,
    },
};
use crate::result::AppResult;
use crate::{domain::resolution::Resolvable, result::AppError};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use uuid::Uuid;

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
        let build = RawResourceBuilder::default();
        self.visit_value(val, build)
    }

    fn visit_value(
        &self,
        val: ast::Value,
        build: RawResourceBuilder,
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
        build: RawResourceBuilder,
    ) -> AppResult<(ResourceList, TransformList)> {
        let ast::Declaration {
            identifier,
            value,
            metas,
            ..
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

        let updated_build = build
            .metas(metas)
            .identifier(Some(identifier.0.clone()))
            .try_append_ctx_path(&identifier.0)?;

        self.visit_value(value, updated_build)
    }

    fn visit_import(
        &self,
        val: ast::Import,
        build: RawResourceBuilder,
    ) -> AppResult<(ResourceList, TransformList)> {
        let ast::Import {
            import_config,
            import_id,
            import_mark,
        } = val;

        let RawResource { ctx_path, .. } = build.clone().build_as_string("UNUSED")?;

        let (mut variables_acc, mut resource_acc, mut trans_acc) = (
            VariablesMap::new(),
            ResourceList::new(),
            TransformList::new(),
        );

        let add_variable =
            |map: &mut VariablesMap, variable: ast::ImportVariable| -> AppResult<()> {
                let ast::ImportVariable { identifier, value } = variable;
                let (sub_resource_map, _) = self.visit_value(value, build.clone())?;
                for resource in sub_resource_map {
                    map.insert(identifier.0.clone(), resource.try_resolve()?);
                }
                Ok(())
            };

        let add_resource_and_transform = |r_list: &mut ResourceList,
                                          t_list: &mut TransformList,
                                          import: ast::Import|
         -> AppResult<()> {
            let (sub_resources, sub_transforms) = self.visit_import(import, build.clone())?;
            r_list.extend(sub_resources);
            t_list.extend(sub_transforms);
            Ok(())
        };

        import_config.into_iter().try_for_each(|elem| {
            match elem {
                ast::ImportConfig::Variable(var) => add_variable(&mut variables_acc, var)?,
                ast::ImportConfig::Import(i) => {
                    add_resource_and_transform(&mut resource_acc, &mut trans_acc, i)?
                }
            }
            AppResult::Ok(())
        })?;

        let ast::File {
            transforms: file_transforms,
            value: file_value,
            name,
            ..
        } = ast::File::try_from(self.resolver.read(&import_id.0)?.as_str())?;

        let build = match (import_mark, name) {
            (ast::ImportMark::Anon(_), _) => build,
            (ast::ImportMark::Named(_), Some(name)) => build.try_append_ctx_path(&name.0)?,
            (ast::ImportMark::Named(_), None) => Err(AppError::String(format!(
                "Imported file {} should have a name",
                import_id.0
            )))?,
            (ast::ImportMark::Uniq(_), None) => {
                build.try_append_ctx_path(&Uuid::new_v4().to_string())?
            }
            (ast::ImportMark::Uniq(_), Some(name)) => {
                let id = format!("{}_{}", name.0, Uuid::new_v4().to_string());
                build.try_append_ctx_path(&id)?
            }
        };
        let build = build.ctx_variables(variables_acc.clone());

        if let Some(t) = file_transforms {
            let transforms = t
                .into_iter()
                .flat_map(|t| {
                    RawTransformation::from_ast(t, Some(variables_acc.clone()), ctx_path.clone())
                        .unwrap_or_default()
                })
                .collect::<Vec<RawTransformation>>();
            trans_acc.extend(transforms);
        }
        let (sub_resources, sub_transforms) = self.visit_value(file_value, build)?;

        trans_acc.splice(0..0, sub_transforms);
        resource_acc.splice(0..0, sub_resources);
        Ok((resource_acc, trans_acc))
    }

    fn visit_string(
        &self,
        val: ast::String,
        build: RawResourceBuilder,
    ) -> AppResult<(ResourceList, TransformList)> {
        Ok((vec![build.build_as_string(&val.0)?], vec![]))
    }

    fn visit_text(
        &self,
        val: ast::Text,
        build: RawResourceBuilder,
    ) -> AppResult<(ResourceList, TransformList)> {
        Ok((vec![build.build_as_string(&val.0)?], vec![]))
    }

    fn visit_number(
        &self,
        val: ast::Number,
        build: RawResourceBuilder,
    ) -> AppResult<(ResourceList, TransformList)> {
        Ok((vec![build.build_as_number(val.0)?], vec![]))
    }

    fn visit_reference(
        &self,
        val: ast::Ref,
        build: RawResourceBuilder,
    ) -> AppResult<(ResourceList, TransformList)> {
        Ok((vec![build.build_as_reference(&val.get_var_name())?], vec![]))
    }
}
