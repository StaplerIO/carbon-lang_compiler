use crate::shared::ast::action::Action;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_descriptor::{JumpCommandBuildResult, RelocationType};

pub fn build_conditional_jump_command(action: &Action, metadata: &PackageMetadata) -> JumpCommandBuildResult {
    panic!("Function not implemented!");
}

pub fn build_jump_command(relocation_type: RelocationType, metadata: &PackageMetadata) -> JumpCommandBuildResult {
    panic!("Function not implemented!");
}
