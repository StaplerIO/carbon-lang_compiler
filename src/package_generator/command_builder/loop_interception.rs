use crate::package_generator::command_builder::templates::jump_command::direct_jump_command_builder;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_descriptor::{JumpCommandBuildResult, RelocationType};

pub fn break_action_command_builder(metadata: &PackageMetadata) -> JumpCommandBuildResult {
    return direct_jump_command_builder(RelocationType::BreakDomain(0), metadata);
}

pub fn continue_action_builder(metadata: &PackageMetadata) -> JumpCommandBuildResult {
    return direct_jump_command_builder(RelocationType::DomainHead(0), metadata);
}
