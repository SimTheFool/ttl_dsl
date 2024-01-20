use crate::domain::ast::{self};
use crate::domain::resolution::{RawTransformation, ResourceContextBuilder};
use crate::domain::resolution::{Resolvable, ResolvedResourceValue};
use crate::domain::transformation::apply_transforms;
use crate::domain::visitor::AstVisitor;
use crate::ports::{ConfigProviderPort, FormatterPort, ResolverPort};
use crate::statics::logger::info;
use crate::utils::result::AppResult;

pub struct AssembleFromStr<'a, R: ResolverPort, C: ConfigProviderPort, F: FormatterPort> {
    pub resolver: &'a R,
    pub config: &'a C,
    pub formatter: &'a F,
}

impl<R: ResolverPort, C: ConfigProviderPort, F: FormatterPort> AssembleFromStr<'_, R, C, F> {
    pub fn execute(&self, file_str: &str) -> AppResult<F::Format> {
        info!("Assembling from string");

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
                Ok(vec) => vec.into_iter().map(Ok).collect(),
                Err(er) => vec![Err(er)],
            })
            .collect::<AppResult<Vec<RawTransformation>>>()?;

        let visitor = AstVisitor::new(self.resolver);

        let (resources, inner_transforms) = match value {
            Some(v) => visitor.visit(v)?,
            None => (vec![], vec![]),
        };

        transforms.extend(inner_transforms);

        let layers = self.config.get_transform_layers()?;

        let resources_map = match transforms {
            t if t.is_empty() => Ok(resources.try_resolve()?),
            t => apply_transforms(resources.try_resolve()?, t.try_resolve()?, layers),
        }?;

        let resources_map = resources_map
            .into_iter()
            .filter(|r| !matches!(r.value, ResolvedResourceValue::Null))
            .collect::<Vec<_>>();
        let formatted = self.formatter.format(resources_map)?;

        Ok(formatted)
    }
}
