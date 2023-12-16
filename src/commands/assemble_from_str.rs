use crate::domain::ast::{self};
use crate::domain::resolution::Resolvable;
use crate::domain::resolution::ResolvedResource;
use crate::domain::resolution::{RawTransformation, ResourceContextBuilder};
use crate::domain::transformation::apply_transforms;
use crate::domain::visitor::AstVisitor;
use crate::utils::result::AppResult;
use indexmap::IndexMap;

pub struct AssembleFromStr<'a, R, C>
where
    R: crate::ports::ResolverPort,
    C: crate::ports::ConfigProviderPort,
{
    pub resolver: &'a R,
    pub config: &'a C,
}

impl<R, C> AssembleFromStr<'_, R, C>
where
    R: crate::ports::ResolverPort,
    C: crate::ports::ConfigProviderPort,
{
    pub fn execute(&self, file_str: &str) -> AppResult<Vec<ResolvedResource>> {
        let ast::File {
            value, transforms, ..
        } = ast::File::try_from(file_str)?;

        let mut transforms = transforms
            .unwrap_or_default()
            .into_iter()
            .map(|t| {
                let blank_context = ResourceContextBuilder::default();
                RawTransformation::from_ast(t, blank_context).map(|v| v.unwrap_or_default())
            })
            .flat_map(|r| match r {
                Ok(vec) => vec.into_iter().map(|item| Ok(item)).collect(),
                Err(er) => vec![Err(er)],
            })
            .collect::<AppResult<Vec<RawTransformation>>>()?;

        let visitor = AstVisitor::new(self.resolver);

        let (resources, inner_transforms) = visitor.visit(value)?;
        transforms.extend(inner_transforms);

        let resources_map = resources
            .into_iter()
            .map(|r| {
                let key_path = r.ctx_path.clone().unwrap_or_default();
                let new_kv = (key_path, r.try_resolve()?);
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
            t if t.is_empty() => Ok(resources_map),
            t => apply_transforms(resources_map, t.try_resolve()?, layers),
        }?;

        Ok(resources_map.into_iter().map(|(_, v)| v).collect())
    }
}
