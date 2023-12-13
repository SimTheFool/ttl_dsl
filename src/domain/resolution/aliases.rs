use super::{RawResource, RawTransformation, ResolvedResource};
use std::collections::HashMap;

pub type VariablesMap = HashMap<String, ResolvedResource>;
pub type ResourceList = Vec<RawResource>;
pub type TransformList = Vec<RawTransformation>;
