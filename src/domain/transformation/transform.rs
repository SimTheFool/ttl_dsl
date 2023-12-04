use regex::Regex;

use crate::{
    domain::{ast, resource::ResourceContext},
    result::{AppError, AppResult},
};

pub struct Transform {
    pub context: Box<ResourceContext>,
    pub rule: String,
    pub layer: Option<String>,
}

impl Transform {
    pub fn from_ast(ast: ast::Transform, ctx: ResourceContext) -> Option<Vec<Self>> {
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

    pub fn get_resolved_rule(&self) -> AppResult<String> {
        let mut rule = self.rule.clone();
        let variables = &self.context.variables;

        let var_id_regex = Regex::new(r"\$([\w]*)").expect("Invalid regular expression");
        let mut var_names: Vec<_> = var_id_regex
            .captures_iter(&self.rule)
            .map(|capture| capture[1].to_string())
            .collect();
        var_names.sort_by(|a, b| b.len().cmp(&a.len()));

        let var_names = match var_names {
            v if v.is_empty() => None,
            v => Some(v),
        };

        let rule = match (var_names, variables) {
            (None, _) => rule,
            (Some(_), None) => Err(AppError::Str("No variables for rules"))?,
            (Some(v), Some(vars)) => {
                for var_name in v {
                    let var_value = vars.get(&var_name).ok_or_else(|| {
                        AppError::String(format!("Variable not found: {}", var_name))
                    })?;

                    rule = rule.replace(&format!("${}", var_name), &var_value.to_string());
                }

                rule
            }
        };

        Ok(rule)
    }
}
