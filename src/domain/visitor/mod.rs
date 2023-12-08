use super::{
    ast,
    resolution::{RawResource, RawResourceBuilder, RawTransformation, ResolvedResource},
};
use crate::domain::resolution::Resolvable;
use crate::result::AppResult;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashMap;

type VisitorAggregate = (Vec<RawResource>, Vec<RawTransformation>);

pub struct AstVisitor<'a> {
    pub resolver: &'a dyn crate::ports::ResolverPort,
}
impl<'a> AstVisitor<'a> {
    pub fn new(resolver: &'a impl crate::ports::ResolverPort) -> Self {
        Self { resolver: resolver }
    }
}
impl AstVisitor<'_> {
    pub fn visit(&self, val: ast::Value) -> AppResult<VisitorAggregate> {
        let build = RawResourceBuilder::default();
        self.visit_value(val, build)
    }

    fn visit_value(
        &self,
        val: ast::Value,
        build: RawResourceBuilder,
    ) -> AppResult<VisitorAggregate> {
        let visit_result = match val {
            ast::Value::String(s) => self.visit_string(s, build)?,
            ast::Value::Number(nb) => self.visit_number(nb, build)?,
            ast::Value::Reference(reference) => self.visit_reference(reference, build)?,
            ast::Value::Object(ast::Object(elems)) => {
                let result = elems
                    .into_par_iter()
                    .map(|elem| match elem {
                        ast::ObjectElem::Declaration(v) => self.visit_declaration(v, build.clone()),
                        ast::ObjectElem::Import(i) => self.visit_import(i, build.clone()),
                    })
                    .collect::<AppResult<Vec<VisitorAggregate>>>()?;

                let result: VisitorAggregate =
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
    ) -> AppResult<VisitorAggregate> {
        let ast::Declaration {
            identifier,
            value,
            metas,
        } = val;

        let metas = match metas {
            Some(m) => m
                .0
                .into_iter()
                .map(|m| {
                    let meta_build = build.clone();
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
    ) -> AppResult<VisitorAggregate> {
        let ast::Import { declarations, path } = val;

        let mut variables_map = HashMap::<String, ResolvedResource>::new();
        for ast::Declaration {
            identifier, value, ..
        } in declarations
        {
            let updated_build = build.clone();
            let (sub_resource_map, _) = self.visit_value(value, updated_build)?;

            for resource in sub_resource_map {
                variables_map.insert(identifier.0.clone(), resource.try_resolve()?);
            }
        }

        let RawResource { ctx_path, .. } = build.clone().build_as_string("UNUSED")?;

        let import = self.resolver.read(&path.0)?;
        let file = ast::File::try_from(import.as_str())?;

        let file_transforms = match file.transforms {
            None => vec![],
            Some(t) => {
                let transforms = t
                    .into_iter()
                    .flat_map(|t| {
                        RawTransformation::from_ast(
                            t,
                            Some(variables_map.clone()),
                            ctx_path.clone(),
                        )
                        .unwrap_or_default()
                    })
                    .collect::<Vec<RawTransformation>>();
                transforms
            }
        };

        let build = build.ctx_variables(variables_map);
        let (sub_maps, mut sub_transforms) = self.visit_value(file.value, build)?;
        sub_transforms.extend(file_transforms);

        Ok((sub_maps, sub_transforms))
    }

    fn visit_string(
        &self,
        val: ast::StringLit,
        build: RawResourceBuilder,
    ) -> AppResult<VisitorAggregate> {
        Ok((vec![build.build_as_string(&val.0)?], vec![]))
    }

    fn visit_number(
        &self,
        val: ast::Number,
        build: RawResourceBuilder,
    ) -> AppResult<VisitorAggregate> {
        Ok((vec![build.build_as_number(val.0)?], vec![]))
    }

    fn visit_reference(
        &self,
        val: ast::Ref,
        build: RawResourceBuilder,
    ) -> AppResult<VisitorAggregate> {
        Ok((vec![build.build_as_reference(&val.0)?], vec![]))
    }
}
