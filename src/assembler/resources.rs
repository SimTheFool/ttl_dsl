use std::collections::HashMap;

use crate::utils::result::AppResult;

use super::ast;

#[derive(PartialEq, Debug, Clone)]
pub struct Referenced();

#[derive(PartialEq, Debug, Clone)]
pub struct Literal();

#[derive(PartialEq, Debug, Clone)]
pub struct ResourceContext {
    pub variables: Option<HashMap<String, Resources<Literal>>>,
    pub path: Option<String>,
}
impl ResourceContext {
    pub fn new() -> Self {
        Self {
            variables: None,
            path: None,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Resource<T, U>
where
    T: Clone,
    U: Clone,
{
    context: Box<ResourceContext>,
    pub identifier: Option<String>,
    pub value: T,
    _phantom: std::marker::PhantomData<U>,
}

impl<T, U> Resource<T, U>
where
    T: Clone,
    U: Clone,
{
    pub fn new(value: T, identifier: Option<String>, ctx: ResourceContext) -> Self {
        Self {
            context: Box::new(ctx),
            identifier,
            value,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Resource<T, Referenced>
where
    T: Clone,
{
    pub fn try_compute_references(self) -> AppResult<Resource<T, Literal>> {
        Ok(Resource::<T, Literal> {
            context: self.context,
            identifier: self.identifier,
            value: self.value,
            _phantom: std::marker::PhantomData,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Resources<U>
where
    U: Clone,
{
    String(Resource<String, U>),
    Number(Resource<f64, U>),
}

impl Resources<Referenced> {
    pub fn try_compute_references(self) -> AppResult<Resources<Literal>> {
        match self {
            Self::String(s) => Ok(Resources::<Literal>::String(s.try_compute_references()?)),
            Self::Number(n) => Ok(Resources::<Literal>::Number(n.try_compute_references()?)),
        }
    }
}
