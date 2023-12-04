use crate::result::{AppError, AppResult};
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

impl<T, U> Resource<T, U>
where
    T: Clone + Default,
    U: ResourceState,
{
    pub fn from_with_value<V>(&self, value: V) -> Resource<V, U>
    where
        V: Clone + Default,
    {
        Resource::<V, U> {
            context: self.context.clone(),
            identifier: self.identifier.clone(),
            value,
            metas: self.metas.clone(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn default_with_value(value: T) -> Self {
        let mut resource = Resource::<T, U>::default();
        resource.value = value;
        resource
    }
}

pub type RawStringBuilder = ResourceBuilder<String, Raw>;
pub type RawNumberBuilder = ResourceBuilder<f64, Raw>;
pub type ResolvedStringBuilder = ResourceBuilder<String, Raw>;
pub type ResolvedNumberBuilder = ResourceBuilder<f64, Raw>;

impl<T, U> ResourceBuilder<T, U>
where
    T: Clone + Default,
    U: ResourceState,
{
    pub fn context(&mut self, ctx: ResourceContext) -> &mut Self {
        self.context = Some(Box::new(ctx));
        self
    }

    pub fn context_box(&mut self, ctx: Box<ResourceContext>) -> &mut Self {
        self.context = Some(ctx);
        self
    }
}

impl ResourceBuilder<String, Raw> {
    pub fn build_string_resource(&mut self) -> AppResult<RawResources> {
        let resource = self.build()?;
        Ok(RawResources::String(resource))
    }

    pub fn build_reference_resource(&mut self) -> AppResult<RawResources> {
        let resource = self.build()?;
        Ok(RawResources::Reference(resource))
    }
}

impl ResourceBuilder<f64, Raw> {
    pub fn build_number_resource(&mut self) -> AppResult<RawResources> {
        let resource = self.build()?;
        Ok(RawResources::Number(resource))
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

impl ResolvedResources {
    pub fn to_string(&self) -> String {
        match self {
            ResolvedResources::String(resource) => resource.value.clone(),
            ResolvedResources::Number(resource) => resource.value.to_string(),
        }
    }
}

impl RawResources {
    pub fn get_id(&self) -> String {
        match self {
            RawResources::String(resource) => resource.identifier.clone().unwrap_or_default(),
            RawResources::Number(resource) => resource.identifier.clone().unwrap_or_default(),
            RawResources::Reference(resource) => resource.identifier.clone().unwrap_or_default(),
        }
    }
}
