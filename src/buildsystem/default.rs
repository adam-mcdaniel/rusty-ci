use crate::buildsystem::BuildSystem;


/// This is a dummy struct, it just uses an empty impl for BuildSystem.
pub struct DefaultBuildSystem;
impl DefaultBuildSystem {
    pub fn new() -> Self { Self {} }
}

impl BuildSystem for DefaultBuildSystem {
}