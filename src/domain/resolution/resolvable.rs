use std::collections::HashMap;

use regex::{Captures, Regex};

use super::{
    RawResource, RawResourceValue, RawTransformation, ResolvedResource, ResolvedResourceBuilder,
    ResolvedTransformation,
};
use crate::{
    regex::replace_all,
    result::{AppError, AppResult},
};

pub trait Resolvable {
    type ResolutionType;

    fn resolve_string(
        str: String,
        variables: &HashMap<String, ResolvedResource>,
    ) -> AppResult<String> {
        let regex1 = Regex::new(r"\$\{(.+)\}")
            .map_err(|e| AppError::String(format!("Invalid regular expression: {}", e)))?;
        let regex2 = Regex::new(r"\$(\p{L}*)")
            .map_err(|e| AppError::String(format!("Invalid regular expression: {}", e)))?;

        let variable_replacer = |captures: &Captures| -> AppResult<String> {
            let full = captures.get(0).map(|g| g.as_str()).unwrap();
            let group = captures.get(1).map(|g| g.as_str());

            let var_value = match group {
                None => Err(AppError::String(format!("No group found for {}", full)))?,
                Some(group) => {
                    let var_value = variables.get(&group.to_string()).ok_or_else(|| {
                        AppError::String(format!("Variable not found: {}", group.to_string()))
                    })?;

                    var_value.value.to_string()
                }
            };

            Ok(var_value)
        };

        let mut replaced: String = str;
        for regex in vec![regex1, regex2] {
            replaced = replace_all(&regex, &replaced, variable_replacer)?;
        }

        Ok(replaced)
    }
    fn try_resolve(self) -> AppResult<Self::ResolutionType>;
}

impl Resolvable for RawResource {
    type ResolutionType = ResolvedResource;
    fn try_resolve(self) -> AppResult<Self::ResolutionType> {
        let resolved_build = ResolvedResourceBuilder::default()
            .metas(
                self.metas
                    .into_iter()
                    .map(|x| x.try_resolve())
                    .collect::<AppResult<Vec<ResolvedResource>>>()?,
            )
            .identifier(self.ctx_path.clone());

        let variables = self.ctx_variables.unwrap_or_default();

        let resolved_resource = match self.value {
            RawResourceValue::String(s) => {
                let resolved_string = Self::resolve_string(s, &variables)?;
                resolved_build.build_as_string(&resolved_string)?
            }
            RawResourceValue::Number(n) => resolved_build.build_as_number(n)?,
            RawResourceValue::Reference(var_name) => {
                let var_value = variables.get(&var_name).map(|r| r.value.clone());
                match var_value {
                    Some(r) => resolved_build.value(r).build()?,
                    None => resolved_build.build_as_null()?,
                }
            }
        };

        Ok(resolved_resource)
    }
}

impl Resolvable for RawTransformation {
    type ResolutionType = ResolvedTransformation;
    fn try_resolve(self) -> AppResult<Self::ResolutionType> {
        let resource_path = self.ctx_path.unwrap_or_default();
        let mut variables = self.ctx_variables.unwrap_or_default();
        variables.insert(
            "".to_string(),
            ResolvedResourceBuilder::default().build_as_string(&resource_path)?,
        );

        let resolved_rule = Self::resolve_string(self.rule, &variables)?;

        Ok(ResolvedTransformation {
            rule: resolved_rule,
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
