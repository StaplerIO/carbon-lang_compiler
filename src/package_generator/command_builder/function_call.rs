use crate::package_generator::command_builder::data_commands::data_declaration_builder;
use crate::package_generator::command_builder::expression_evaluation::{convert_number_to_hex, expression_command_set_builder};
use crate::package_generator::utils::{align_data_width, combine_command, convert_to_u8_array};
use crate::shared::ast::action::CallAction;
use crate::shared::ast::blocks::expression::Expression;
use crate::shared::command_map::{DomainCommand, ObjectCommand, PLACE_HOLDER, RootCommand, StackCommand};
use crate::shared::package_generation::data_descriptor::{DataAccessDescriptor, DataDeclaration};
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::function::{FunctionCallDescriptor, FunctionDescriptor};

pub fn function_call_builder(action: &CallAction, defined_data: &Vec<DataDeclaration>, metadata: &PackageMetadata, defined_functions: &Vec<FunctionDescriptor>) -> (Vec<u8>, FunctionCallDescriptor) {
    let mut result = vec![];
    let descriptor: FunctionCallDescriptor = FunctionCallDescriptor {
        target_function_identifier: action.function_name.clone(),
        relocation_start_loc: 0,
    };
    // let target_function = defined_functions.iter().find(|&x| x.identifier.eq(&descriptor.target_function_identifier)).unwrap();

    let mut dac_list = vec![];

    // Calculate parameters
    for (idx, param) in action.arguments.iter().enumerate() {
        // Data of current parameter is on the top of the stack
        let expression_eval_cmd = expression_command_set_builder(Expression {
            postfix_expr: param.postfix_expr.clone(),
            output_type: "".to_string()
        }, defined_data, metadata);
        result.extend(expression_eval_cmd);

        // Declare a temporary slot to save data
        let data_def = data_declaration_builder(false);
        result.extend(data_def);

        // Pop stack data to data slot
        let mut pop_data = vec![];
        pop_data.push(combine_command(RootCommand::Stack.to_opcode(), StackCommand::PopToObject.to_opcode()));
        // Get slot id, build DAC(Data access descriptor)
        let slot_id = defined_data.len() + idx - 1;
        let dac = DataAccessDescriptor {
            is_global: false,
            domain_layer: align_data_width(vec![0x00], metadata.domain_layer_count_alignment),
            slot: align_data_width(convert_to_u8_array(convert_number_to_hex(slot_id.to_string())), metadata.data_alignment)
        };
        dac_list.push(dac.clone());
        // Push DAC to target command stream
        pop_data.push(combine_command(u8::from(dac.is_global), PLACE_HOLDER));
        pop_data.extend(dac.domain_layer);
        pop_data.extend(dac.slot);
        result.extend(pop_data);
    }

    // Push all values to new domain
    // Create domain
    result.push(combine_command(RootCommand::Domain.to_opcode(), DomainCommand::Create.to_opcode()));

    // Push all parameters
    for (idx, dac) in dac_list.iter().enumerate() {
        let mut cloned_dac = dac.clone();
        // For a new domain, the data we want to move here is in the parent domain
        // Change DomainLayer to parent domain
        cloned_dac.domain_layer[dac.domain_layer.len() - 1] = 0x01;

        // Create a data space in new domain
        let data_def = data_declaration_builder(false);
        result.extend(data_def);

        // Move data to new slot
        let mut move_data = vec![];
        // Push data to current stack
        move_data.push(combine_command(RootCommand::Stack.to_opcode(), StackCommand::PushFromObject.to_opcode()));
        move_data.push(combine_command(u8::from(cloned_dac.is_global), PLACE_HOLDER));
        move_data.extend(cloned_dac.domain_layer);
        move_data.extend(cloned_dac.slot);

        // Pop data from current stack
        // Switch to current DAC
        let current_dac = DataAccessDescriptor {
            is_global: cloned_dac.is_global,
            domain_layer: dac.clone().domain_layer,
            slot: align_data_width(convert_to_u8_array(convert_number_to_hex(idx.to_string())), metadata.data_alignment)
        };
        // Pop by DAC
        let mut pop_data = vec![];
        pop_data.push(combine_command(u8::from(current_dac.is_global), PLACE_HOLDER));
        pop_data.extend(current_dac.domain_layer);
        pop_data.extend(current_dac.slot);
        result.extend(pop_data);
    }

    // Remove old data in parent domain
    for idx in 0..action.arguments.len() {
        // Pop stack data to data slot
        let mut pop_data = vec![];
        pop_data.push(combine_command(RootCommand::Stack.to_opcode(), StackCommand::PopToObject.to_opcode()));
        // Get slot id, build DAC(Data access descriptor)
        let slot_id = defined_data.len() + idx - 1;
        let dac = DataAccessDescriptor {
            is_global: false,
            domain_layer: align_data_width(vec![0x01], metadata.domain_layer_count_alignment),
            slot: align_data_width(convert_to_u8_array(convert_number_to_hex(slot_id.to_string())), metadata.data_alignment)
        };
        // Destroy temporary data in previous stack
        let mut destroy_data = vec![];
        destroy_data.push(combine_command(RootCommand::Object.to_opcode(), ObjectCommand::Destroy.to_opcode()));
        destroy_data.push(combine_command(u8::from(dac.is_global), PLACE_HOLDER));
        destroy_data.extend(dac.domain_layer);
        destroy_data.extend(dac.slot);
        result.extend(destroy_data);
    }

    return (result, descriptor);
}
