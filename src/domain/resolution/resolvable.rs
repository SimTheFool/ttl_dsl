use regex::Regex;

use super::{
    RawResource, RawResourceValue, RawTransformation, ResolvedResource, ResolvedResourceBuilder,
    ResolvedResourceValue, ResolvedTransformation,
};
use crate::result::{AppError, AppResult};

pub trait Resolvable {
    type ResolutionType;
    fn try_resolve(self) -> AppResult<Self::ResolutionType>;
}

impl Resolvable for RawResource {
    type ResolutionType = ResolvedResource;
    fn try_resolve(self) -> AppResult<Self::ResolutionType> {
        let build = ResolvedResourceBuilder::default()
            .metas(
                self.metas
                    .into_iter()
                    .map(|x| x.try_resolve())
                    .collect::<AppResult<Vec<ResolvedResource>>>()?,
            )
            .identifier(self.identifier);

        let resolved_resource = match self.value {
            RawResourceValue::String(s) => build.build_as_string(&s)?,
            RawResourceValue::Number(n) => build.build_as_number(n)?,
            RawResourceValue::Reference(var_name) => {
                let variables = self.ctx_variables;
                let resource_path = self.ctx_path.unwrap_or_default();

                let variables = match variables {
                    Some(x) => x,
                    None => Err(AppError::String(format!(
                        "No variables for {}",
                        resource_path
                    )))?,
                };

                let var_value = variables
                    .get(&var_name)
                    .ok_or(AppError::String(format!("No variables {var_name} found")))?
                    .value
                    .clone();

                match var_value {
                    ResolvedResourceValue::String(s) => build.build_as_string(&s)?,
                    ResolvedResourceValue::Number(n) => build.build_as_number(n)?,
                }
            }
        };

        Ok(resolved_resource)
    }
}

impl Resolvable for RawTransformation {
    type ResolutionType = ResolvedTransformation;
    fn try_resolve(self) -> AppResult<Self::ResolutionType> {
        let mut rule = self.rule.clone();
        let resource_path = self.ctx_path.unwrap_or_default();
        let mut variables = self.ctx_variables.unwrap_or_default();
        variables.insert(
            "".to_string(),
            ResolvedResourceBuilder::default().build_as_string(&resource_path)?,
        );

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
            (Some(v), vars) => {
                for var_name in v {
                    let var_value = vars.get(&var_name).ok_or_else(|| {
                        AppError::String(format!("Variable not found: {}", var_name))
                    })?;

                    let var_value = match &var_value.value {
                        ResolvedResourceValue::String(s) => s.clone(),
                        ResolvedResourceValue::Number(n) => n.to_string(),
                    };

                    rule = rule.replace(&format!("${}", var_name), &var_value);
                }

                rule
            }
        };

        Ok(ResolvedTransformation {
            rule,
            layer: self.layer,
        })
    }
}

impl<R, T> Resolvable for T
where
    T: IntoIterator,
    T::Item: Resolvable<ResolutionType = R>,
    T::IntoIter: Iterator<Item = T::Item>,
{
    type ResolutionType = Vec<R>;

    fn try_resolve(self) -> AppResult<Self::ResolutionType> {
        self.into_iter()
            .map(|x| x.try_resolve())
            .collect::<AppResult<Vec<R>>>()
    }
}
