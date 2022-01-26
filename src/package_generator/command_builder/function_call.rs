use crate::package_generator::command_builder::data_commands::build_data_declaration_command;
use crate::package_generator::command_builder::expression_evaluation::{
    convert_number_to_hex, build_expression_evaluation_command,
};
use crate::package_generator::utils::{align_data_width, combine_command, convert_to_u8_array};
use crate::shared::ast::action::CallAction;
use crate::shared::ast::blocks::expression::SimpleExpression;
use crate::shared::command_map::{
    DomainCommand, ObjectCommand, RootCommand, StackCommand, PLACE_HOLDER,
};
use crate::shared::package_generation::data_descriptor::{DataAccessDescriptor, DataDeclaration};
use crate::shared::package_generation::function::FunctionDescriptor;
use crate::shared::package_generation::package_descriptor::PackageMetadata;

pub fn build_function_call_command(
    action: &CallAction,
    defined_data: &Vec<DataDeclaration>,
    metadata: &PackageMetadata,
    defined_functions: &Vec<FunctionDescriptor>,
) -> Vec<u8> {
    let mut result = vec![];

    // let target_function = defined_functions.iter().find(|&x| x.identifier.eq(&descriptor.target_function_identifier)).unwrap();
    let mut dac_list: Vec<DataAccessDescriptor> = vec![];

    // Calculate parameters
    let calculate_result =
        calculate_parameters(action, defined_data, metadata, defined_functions, dac_list);
    result.extend(calculate_result.0);
    dac_list = calculate_result.1;

    // Push all values to new domain
    // Create domain
    result.push(combine_command(
        RootCommand::Domain.to_opcode(),
        DomainCommand::Create.to_opcode(),
    ));
    // Push all parameters
    result.extend(push_parameters(&dac_list, metadata));
    // Remove old data in parent domain
    result.extend(remove_old_data(action, defined_data, metadata));
    return result;
}

fn calculate_parameters(
    action: &CallAction,
    defined_data: &Vec<DataDeclaration>,
    metadata: &PackageMetadata,
    defined_functions: &Vec<FunctionDescriptor>,
    mut dac_list: Vec<DataAccessDescriptor>,
) -> (Vec<u8>, Vec<DataAccessDescriptor>) {
    let mut result = vec![];

    for (idx, param) in action.arguments.iter().enumerate() {
        // Data of current parameter is on the top of the stack
        // Evaluate the value of the expression
        let expression_eval_cmd = build_expression_evaluation_command(
            &SimpleExpression {
                postfix_expr: param.postfix_expr.clone(),
                output_type: "".to_string(),
            },
            defined_data,
            metadata,
        );
        result.extend(expression_eval_cmd);

        // Declare a temporary slot to save data
        let data_def = build_data_declaration_command(false);
        result.extend(data_def);

        // Pop stack data to data slot
        let mut pop_data = vec![];
        pop_data.push(combine_command(
            RootCommand::Stack.to_opcode(),
            StackCommand::PopToObject.to_opcode(),
        ));
        // Get slot id, build DAC(Data access descriptor)
        let slot_id = defined_data.len() + idx - 1;
        let dac = DataAccessDescriptor {
            is_global: false,
            domain_layer: align_data_width(vec![0x00], metadata.domain_layer_count_alignment),
            slot: align_data_width(
                convert_to_u8_array(convert_number_to_hex(slot_id.to_string())),
                metadata.data_alignment,
            ),
        };
        dac_list.push(dac.clone());
        // Push DAC to target command stream
        pop_data.push(combine_command(u8::from(dac.is_global), PLACE_HOLDER));
        pop_data.extend(dac.domain_layer);
        pop_data.extend(dac.slot);
        result.extend(pop_data);
    }

    return (result, dac_list);
}

fn push_parameters(dac_list: &Vec<DataAccessDescriptor>, metadata: &PackageMetadata) -> Vec<u8> {
    let mut result = vec![];
    for (idx, dac) in dac_list.iter().enumerate() {
        let mut cloned_dac = dac.clone();
        // For a new domain, the data we want to move here is in the parent domain
        // Change DomainLayer to parent domain
        cloned_dac.domain_layer[dac.domain_layer.len() - 1] = 0x01;

        // Create a data space in new domain
        let data_def = build_data_declaration_command(false);
        result.extend(data_def);

        // Move data to new slot
        let mut move_data = vec![];
        // Push data to current stack
        move_data.push(combine_command(
            RootCommand::Stack.to_opcode(),
            StackCommand::PushFromObject.to_opcode(),
        ));
        move_data.push(combine_command(
            u8::from(cloned_dac.is_global),
            PLACE_HOLDER,
        ));
        move_data.extend(cloned_dac.domain_layer);
        move_data.extend(cloned_dac.slot);

        // Pop data from current stack
        // Switch to current DAC
        let current_dac = DataAccessDescriptor {
            is_global: cloned_dac.is_global,
            domain_layer: dac.clone().domain_layer,
            slot: align_data_width(
                convert_to_u8_array(convert_number_to_hex(idx.to_string())),
                metadata.data_alignment,
            ),
        };
        // Pop by DAC
        let mut pop_data = vec![];
        pop_data.push(combine_command(
            u8::from(current_dac.is_global),
            PLACE_HOLDER,
        ));
        pop_data.extend(current_dac.domain_layer);
        pop_data.extend(current_dac.slot);
        result.extend(pop_data);
    }

    return result;
}

fn remove_old_data(
    action: &CallAction,
    defined_data: &Vec<DataDeclaration>,
    metadata: &PackageMetadata,
) -> Vec<u8> {
    let mut result = vec![];

    for idx in 0..action.arguments.len() {
        // Pop stack data to data slot
        let mut pop_data = vec![];
        pop_data.push(combine_command(
            RootCommand::Stack.to_opcode(),
            StackCommand::PopToObject.to_opcode(),
        ));
        // Get slot id, build DAC(Data access descriptor)
        let slot_id = defined_data.len() + idx - 1;
        let dac = DataAccessDescriptor {
            is_global: false,
            domain_layer: align_data_width(vec![0x01], metadata.domain_layer_count_alignment),
            slot: align_data_width(
                convert_to_u8_array(convert_number_to_hex(slot_id.to_string())),
                metadata.data_alignment,
            ),
        };
        // Destroy temporary data in previous stack
        let mut destroy_data = vec![];
        destroy_data.push(combine_command(
            RootCommand::Object.to_opcode(),
            ObjectCommand::Destroy.to_opcode(),
        ));
        destroy_data.push(combine_command(u8::from(dac.is_global), PLACE_HOLDER));
        destroy_data.extend(dac.domain_layer);
        destroy_data.extend(dac.slot);
        result.extend(destroy_data);
    }

    return result;
}
