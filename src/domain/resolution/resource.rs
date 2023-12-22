use std::collections::HashMap;

use crate::result::{AppError, AppResult};
use derive_builder::Builder;

#[derive(PartialEq, Debug, Clone)]
pub enum RawResourceValue {
    String(String),
    Number(f64),
    Reference(String),
}

#[derive(PartialEq, Debug, Clone, Builder)]
#[builder(build_fn(error = "AppError"))]
#[builder(pattern = "owned")]
#[builder(name = "ResourceContextBuilder")]
#[builder(derive(Clone))]
pub struct RawResource {
    #[builder(setter(custom))]
    pub value: RawResourceValue,

    #[builder(default)]
    pub identifier: Option<String>,

    #[builder(default)]
    pub metas: Vec<RawResource>,

    #[builder(default)]
    #[builder(setter(into))]
    pub ctx_variables: Option<HashMap<String, ResolvedResource>>,

    #[builder(default, private)]
    #[builder(setter(into))]
    pub ctx_path: Option<String>,
}

impl ResourceContextBuilder {
    pub fn try_append_ctx_path(self, path: Option<impl Into<String>>) -> AppResult<Self> {
        let new_path: Option<String> = match (self.ctx_path.clone().flatten(), path) {
            (base, None) => base,
            (None, Some(id)) => Some(id.into()),
            (Some(base), Some(id)) => Some(format!("{}.{}", base, id.into())),
        };

        let build = self.ctx_path(new_path);
        Ok(build)
    }

    pub fn build_as_string(mut self, value: &str) -> AppResult<RawResource> {
        self.value = Some(RawResourceValue::String(value.to_string()));
        self.build()
    }

    pub fn build_as_number(mut self, value: f64) -> AppResult<RawResource> {
        self.value = Some(RawResourceValue::Number(value));
        self.build()
    }

    pub fn build_as_reference(mut self, value: &str) -> AppResult<RawResource> {
        self.value = Some(RawResourceValue::Reference(value.to_string()));
        self.build()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum ResolvedResourceValue {
    String(String),
    Number(f64),
    Null,
}
impl ToString for ResolvedResourceValue {
    fn to_string(&self) -> String {
        match self {
            ResolvedResourceValue::String(s) => s.clone(),
            ResolvedResourceValue::Number(n) => n.to_string(),
            ResolvedResourceValue::Null => "".to_string(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Builder)]
#[builder(build_fn(error = "AppError"))]
#[builder(pattern = "owned")]
pub struct ResolvedResource {
    pub value: ResolvedResourceValue,

    #[builder(default)]
    pub identifier: Option<String>,

    #[builder(default)]
    pub metas: Vec<ResolvedResource>,
}

impl ResolvedResourceBuilder {
    pub fn build_as_string(mut self, value: &str) -> AppResult<ResolvedResource> {
        self.value = Some(ResolvedResourceValue::String(value.to_string()));
        self.build()
    }

    pub fn build_as_number(mut self, value: f64) -> AppResult<ResolvedResource> {
        self.value = Some(ResolvedResourceValue::Number(value));
        self.build()
    }

    pub fn build_as_null(mut self) -> AppResult<ResolvedResource> {
        self.value = Some(ResolvedResourceValue::Null);
        self.build()
    }
}
