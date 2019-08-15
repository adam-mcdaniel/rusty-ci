use crate::buildsystem::BuildSystem;

/// This is a dummy struct, it just uses an empty impl for BuildSystem.
#[derive(Default)]
pub struct DefaultBuildSystem;

impl BuildSystem for DefaultBuildSystem {}
