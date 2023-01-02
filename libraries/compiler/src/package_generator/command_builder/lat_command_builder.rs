use crate::package_generator::command_builder::assignment_action::build_assignment_command;
use crate::package_generator::command_builder::data_commands::build_data_declaration_command;
use crate::shared::package_generation::data_descriptor::{DataDeclarator, DataLocation};
use crate::shared::package_generation::linear_action_tree::{LinearActionTree, LinearActionType};
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::RelocatableCommandList;

pub fn linear_action_tree_command(tree: LinearActionTree, metadata: &PackageMetadata) -> RelocatableCommandList {
    let mut result = RelocatableCommandList::new();
    let mut defined_data: Vec<DataDeclarator> = vec![];

    for action in tree.action_array {
        match action.lat_content {
            LinearActionType::FunctionEntrance(_) => {}
            LinearActionType::EndFunction => {}
            LinearActionType::IfEntrance(_) => {}
            LinearActionType::ElseIfEntrance(_) => {}
            LinearActionType::ElseEntrance => {}
            LinearActionType::EndIf => {}
            LinearActionType::WhileEntrance(_) => {}
            LinearActionType::EndWhile => {}
            LinearActionType::LoopEntrance => {}
            LinearActionType::EndLoop => {}
            LinearActionType::BreakStatement => {}
            LinearActionType::ContinueStatement => {}
            LinearActionType::AssignmentAction(x) => {
                result.combine(build_assignment_command(&x, &defined_data, metadata));
            }
            LinearActionType::DeclarationAction(x) => {
                let decl_cmd = build_data_declaration_command(false);
                result.combine(decl_cmd);

                defined_data.push(DataDeclarator {
                    name: x.declarator.identifier.clone(),
                    slot:defined_data.len() + 1,
                    location: DataLocation::Local,
                    is_string: false,
                    is_array: x.declarator.is_array,
                });
            }
            LinearActionType::CallAction(_x) => {}
            LinearActionType::ReturnAction(_x) => {}
        }
    }

    return result;
}
