use crate::package_generator::command_builder::action_block::action_block_builder;
use crate::package_generator::command_builder::templates::jump_command::condition_block_command_builder;
use crate::shared::ast::action::IfAction;
use crate::shared::package_generation::data_descriptor::DataDeclaration;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_descriptor::JumpCommandBuildResult;

pub fn if_command_builder(action: &IfAction, defined_data: &Vec<DataDeclaration>, metadata: &PackageMetadata) -> JumpCommandBuildResult {
    let mut result = JumpCommandBuildResult {
        commands: vec![],
        descriptors: vec![]
    };

    // Build IfBlock
    let mut after_domains: usize = 1 + &action.elif_collection.len() + action.else_action.is_some() as usize;
    result.append(condition_block_command_builder(&action.if_block, after_domains, defined_data, metadata));

    // Push ElifBlocks
    for elif_block in &action.elif_collection {
        after_domains -= 1;
        result.append(condition_block_command_builder(elif_block, after_domains, defined_data, metadata));
    }

    if action.else_action.is_some() {
        result.commands.extend(action_block_builder(&action.else_action.clone().unwrap(), metadata));
    }

    return result;
}
