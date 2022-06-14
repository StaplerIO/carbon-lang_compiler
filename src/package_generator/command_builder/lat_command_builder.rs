use crate::package_generator::command_builder::assignment_action::build_assignment_command;
use crate::package_generator::command_builder::data_commands::build_data_declaration_command;
use crate::package_generator::command_builder::function_call::build_function_call_command;
use crate::package_generator::utils::{align_data_width, convert_to_u8_array};
use crate::shared::package_generation::data_descriptor::DataDeclarator;
use crate::shared::package_generation::function::FunctionDescriptor;
use crate::shared::package_generation::linear_action_tree::{LinearActionTree, LinearActionType};
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_descriptor::{JumpCommandBuildResult, RelocatableCommandList};

pub fn linear_action_tree_command(tree: LinearActionTree, metadata: &PackageMetadata, defined_functions: &Vec<FunctionDescriptor>) -> RelocatableCommandList {
    let mut result = RelocatableCommandList{ commands: vec![], descriptors: vec![] };
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
                let assignment_cmd = build_assignment_command(&x, &defined_data, metadata);
                result.append(JumpCommandBuildResult::new_no_relocation(assignment_cmd));
            }
            LinearActionType::DeclarationAction(x) => {
                let decl_cmd = build_data_declaration_command(false);
                result.append(JumpCommandBuildResult::new_no_relocation(decl_cmd));

                defined_data.push(DataDeclarator {
                    name: x.identifier,
                    slot: align_data_width(convert_to_u8_array((defined_data.len() + 1).to_string()), metadata.data_alignment),
                    is_global: false
                });
            }
            LinearActionType::CallAction(x) => {}
            LinearActionType::ReturnAction(_) => {}
        }
    }

    return result;
}
