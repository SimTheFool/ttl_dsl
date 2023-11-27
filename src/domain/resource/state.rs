use super::{RawResources, ResolvedResources};

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Raw();

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Resolved();

pub trait ResourceState: Clone + Default {
    type MetasType: Clone + Default;
    type BuildType: Clone;
}

impl ResourceState for Raw {
    type MetasType = Vec<RawResources>;
    type BuildType = RawResources;
}

impl ResourceState for Resolved {
    type MetasType = Vec<ResolvedResources>;
    type BuildType = ResolvedResources;
}
