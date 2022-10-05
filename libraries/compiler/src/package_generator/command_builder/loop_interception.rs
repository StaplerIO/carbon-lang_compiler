use crate::package_generator::command_builder::templates::jump_command::direct_jump_command_builder;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::{RelocatableCommandList, RelocationTargetElement};

pub fn break_action_command_builder(metadata: &PackageMetadata) -> RelocatableCommandList {
    return direct_jump_command_builder(RelocationTargetElement::BreakIteration, metadata);
}

pub fn continue_action_command_builder(metadata: &PackageMetadata) -> RelocatableCommandList {
    return direct_jump_command_builder(RelocationTargetElement::IterationHead, metadata);
}
