use crate::buildsystem::BuildSystem;

pub struct DefaultBuildSystem;
impl DefaultBuildSystem {
    pub fn new() -> Self { Self {} }
}

impl BuildSystem for DefaultBuildSystem {
}