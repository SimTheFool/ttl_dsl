use crate::result::AppError;
use derive_builder::Builder;

use super::state::{Raw, Resolved, ResourceState};
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone, Default)]
pub struct ResourceContext {
    pub variables: Option<HashMap<String, ResolvedResources>>,
    pub path: Option<String>,
}

#[derive(PartialEq, Debug, Clone, Default, Builder)]
#[builder(build_fn(error = "AppError"))]
#[builder(default)]
pub struct Resource<T, U>
where
    T: Clone + Default,
    U: ResourceState,
{
    #[builder(setter(custom))]
    pub context: Box<ResourceContext>,
    pub identifier: Option<String>,
    pub value: T,
    pub metas: Option<U::MetasType>,
    _phantom: std::marker::PhantomData<U>,
}

impl<T, U> ResourceBuilder<T, U>
where
    T: Clone + Default,
    U: ResourceState,
{
    pub fn context(mut self, ctx: ResourceContext) -> Self {
        self.context = Some(Box::new(ctx));
        self
    }

    pub fn context_box(mut self, ctx: Box<ResourceContext>) -> Self {
        self.context = Some(ctx);
        self
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum RawResources {
    String(Resource<String, Raw>),
    Number(Resource<f64, Raw>),
    Reference(Resource<String, Raw>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum ResolvedResources {
    String(Resource<String, Resolved>),
    Number(Resource<f64, Resolved>),
}
