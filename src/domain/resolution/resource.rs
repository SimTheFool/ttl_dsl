use super::ResolutionContext;
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
pub struct RawResource {
    #[builder(setter(custom))]
    pub value: RawResourceValue,

    #[builder(default)]
    pub identifier: Option<String>,

    #[builder(default)]
    #[builder(setter(custom))]
    pub context: Box<ResolutionContext>,

    #[builder(default)]
    pub metas: Vec<RawResource>,
}

impl RawResourceBuilder {
    pub fn context(mut self, ctx: ResolutionContext) -> Self {
        self.context = Some(Box::new(ctx));
        self
    }

    pub fn context_box(mut self, ctx: Box<ResolutionContext>) -> Self {
        self.context = Some(ctx);
        self
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
}

#[derive(PartialEq, Debug, Clone, Builder)]
#[builder(build_fn(error = "AppError"))]
#[builder(pattern = "owned")]
pub struct ResolvedResource {
    #[builder(setter(custom))]
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
}
