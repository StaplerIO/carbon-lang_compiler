use crate::package_generator::command_builder::action_block::action_block_command_builder;
use crate::package_generator::command_builder::function_block::build_function_command;
use crate::package_generator::utils::align_array_width;
use crate::shared::ast::group::declaration::GroupDeclarationBlock;
use crate::shared::ast::group::implementation::GroupImplementationBlock;
use crate::shared::package_generation::group_context::GeneratedGroup;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::RelocatableCommandList;
use crate::shared::utils::identifier::Identifier;

pub fn group_implementation_builder(impl_block: GroupImplementationBlock, source_group: &GroupDeclarationBlock, generated_groups: &Vec<GeneratedGroup>, metadata: &PackageMetadata) -> Result<RelocatableCommandList, Vec<Identifier>> {
    let mut result = RelocatableCommandList::new();
    let mut dependency = vec![];

    // Generate source group of this implementation
    let source_group_index = generated_groups.iter()
                                             .find(|g| g.identifier == impl_block.source_group);
    if source_group_index.is_none() {
        dependency.push(impl_block.source_group.clone());
    } else {
        result.commands.extend(align_array_width(&source_group_index.unwrap().slot.to_be_bytes().to_vec(), metadata.data_slot_alignment));
    }

    // Generate field getter/setter implementation
    for field in impl_block.fields.iter() {
        if field.get_block.is_some() {
            let get = field.get_block.as_ref().unwrap();
            // Assign field slot
            result.commands.extend(align_array_width(&field.slot.to_be_bytes().to_vec(), metadata.data_slot_alignment));

            // Assign whether it as a getter
            result.commands.push('G' as u8);

            // TODO: Self value should be in the defined data
            let commands = action_block_command_builder(get, true, &vec![], metadata);

            // Push command length
            result.commands.extend(align_array_width(&commands.commands.len().to_be_bytes().to_vec(), metadata.data_slot_alignment));
            result.combine(commands);
        }

        if field.set_block.is_some() {
            let get = field.set_block.as_ref().unwrap();
            // Assign field slot
            result.commands.extend(align_array_width(&field.slot.to_be_bytes().to_vec(), metadata.data_slot_alignment));

            // Assign whether it as a setter
            result.commands.push('S' as u8);

            // TODO: Self value should be in the defined data
            let rcl = action_block_command_builder(get, true, &vec![], metadata);

            // Push command length
            result.commands.extend(align_array_width(&rcl.commands.len().to_be_bytes().to_vec(), metadata.data_slot_alignment));
            result.combine(rcl);
        }
    }

    // Generate methods
    for method in impl_block.methods.iter() {
        let source_method_slot = source_group.methods
            .iter()
            .position(|m| *m == method.declarator)
            .unwrap();

        // Assign method slot
        result.commands.extend(align_array_width(&source_method_slot.to_be_bytes().to_vec(), metadata.data_slot_alignment));

        let rcl = build_function_command(method, metadata);

        // Push command length
        result.commands.extend(align_array_width(&rcl.commands.len().to_be_bytes().to_vec(), metadata.data_slot_alignment));
        // Place commands
        result.combine(rcl);
    }

    // Generate functions
    for func in impl_block.functions.iter() {
        let source_func_slot = source_group.functions
                                           .iter()
                                           .position(|m| *m == func.declarator)
                                           .unwrap();

        // Assign function slot
        result.commands.extend(align_array_width(&source_func_slot.to_be_bytes().to_vec(), metadata.data_slot_alignment));

        let rcl = build_function_command(func, metadata);

        // Push command length
        result.commands.extend(align_array_width(&rcl.commands.len().to_be_bytes().to_vec(), metadata.data_slot_alignment));
        // Place commands
        result.combine(rcl);
    }

    return if dependency.is_empty() {
        Ok(result)
    } else {
        Err(dependency)
    };
}
