use super::{RawResources, ResolvedResources};

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Raw();

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Resolved();

pub trait ResourceState: Clone + Default {
    type MetasType: Clone + Default;
}

impl ResourceState for Raw {
    type MetasType = Vec<RawResources>;
}

impl ResourceState for Resolved {
    type MetasType = Vec<ResolvedResources>;
}
