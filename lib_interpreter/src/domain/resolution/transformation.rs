use super::{RawResource, ResolvedResource, ResourceContextBuilder};
use crate::{domain::ast, result::AppResult};
use std::collections::HashMap;

pub struct RawTransformation {
    pub rule: String,
    pub layer: Option<String>,
    pub ctx_variables: Option<HashMap<String, ResolvedResource>>,
    pub ctx_path: Option<String>,
}

pub struct ResolvedTransformation {
    pub rule: String,
    pub layer: Option<String>,
}

impl RawTransformation {
    pub fn from_ast(
        ast: ast::Transform,
        build: ResourceContextBuilder,
    ) -> AppResult<Option<Vec<Self>>> {
        let rules = ast.rules;
        let RawResource {
            ctx_path,
            ctx_variables,
            ..
        } = build.clone().build_as_string("UNUSED")?;

        let transf: Option<Vec<RawTransformation>> = rules.map(|rules| {
            rules
                .into_iter()
                .map(|r| Self {
                    rule: r.0,
                    layer: Some(ast.layer.0.clone()),
                    ctx_variables: ctx_variables.clone(),
                    ctx_path: ctx_path.clone(),
                })
                .collect()
        });

        Ok(transf)
    }
}
