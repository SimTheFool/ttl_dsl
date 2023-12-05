use super::ResolutionContext;
use crate::domain::ast;

pub struct RawTransformation {
    pub context: Box<ResolutionContext>,
    pub rule: String,
    pub layer: Option<String>,
}

pub struct ResolvedTransformation {
    pub rule: String,
    pub layer: Option<String>,
}

impl RawTransformation {
    pub fn from_ast(ast: ast::Transform, ctx: ResolutionContext) -> Option<Vec<Self>> {
        let rules = ast.rules;

        match rules {
            Some(rules) => Some(
                rules
                    .into_iter()
                    .map(|r| Self {
                        context: Box::new(ctx.clone()),
                        rule: r.0,
                        layer: Some(ast.layer.0.clone()),
                    })
                    .collect(),
            ),
            None => None,
        }
    }
}
