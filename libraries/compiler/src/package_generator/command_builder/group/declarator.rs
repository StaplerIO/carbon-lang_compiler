use crate::package_generator::utils::align_array_width;
use crate::shared::ast::blocks::function::FunctionDeclarator;
use crate::shared::ast::group::declaration::GroupDeclarationBlock;
use crate::shared::package_generation::group_context::GeneratedGroup;
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::RelocatableCommandList;
use crate::shared::utils::identifier::Identifier;

pub fn group_declarator_builder(decl_block: GroupDeclarationBlock, generated_groups: &Vec<GeneratedGroup>, metadata: &PackageMetadata) -> Result<RelocatableCommandList, Vec<Identifier>> {
    let mut result = RelocatableCommandList::new();
    let mut dependency = vec![];

    // Generate fields
    result.commands.extend(align_array_width(&decl_block.fields.len().to_be_bytes().to_vec(), metadata.data_slot_alignment));
    for field in decl_block.fields.iter() {
        let source = generated_groups.iter()
                                     .find(|g| g.identifier == field.data_type);

        if source.is_none() {
            dependency.push(field.data_type.clone());
        } else {
            result.commands.extend(align_array_width(&source.unwrap().slot.to_be_bytes().to_vec(), metadata.data_slot_alignment));
        }
    }

    // Generate methods
    result.commands.extend(align_array_width(&decl_block.methods.len().to_be_bytes().to_vec(), metadata.data_slot_alignment));
    for method in decl_block.methods.iter() {
        let declarator = separated_function_declarator_builder(method, generated_groups, metadata);
        if declarator.is_ok() {
            result.combine(declarator.unwrap());
        } else {
            dependency.extend(declarator.unwrap_err());
        }
    }

    // Generate functions
    result.commands.extend(align_array_width(&decl_block.functions.len().to_be_bytes().to_vec(), metadata.data_slot_alignment));
    for function in decl_block.functions.iter() {
        let declarator = separated_function_declarator_builder(function, generated_groups, metadata);
        if declarator.is_ok() {
            result.combine(declarator.unwrap());
        } else {
            dependency.extend(declarator.unwrap_err());
        }
    }

    return if dependency.is_empty() {
        Ok(result)
    } else {
        Err(dependency)
    };
}

fn separated_function_declarator_builder(decl: &FunctionDeclarator, generated_groups: &Vec<GeneratedGroup>, metadata: &PackageMetadata) -> Result<RelocatableCommandList, Vec<Identifier>> {
    let mut result = RelocatableCommandList::new();
    let mut dependency = vec![];

    // Build return type
    let return_type = generated_groups.iter()
                                      .find(|g| g.identifier == decl.return_type);
    if return_type.is_none() {
        dependency.push(decl.return_type.clone());
    } else {
        result.commands.extend(align_array_width(&return_type.unwrap().slot.to_be_bytes().to_vec(), metadata.data_slot_alignment));
    }

    // Build parameters
    result.commands.extend(align_array_width(&decl.parameters.len().to_be_bytes().to_vec(), metadata.data_slot_alignment));
    for param in decl.parameters.iter() {
        let param_type = generated_groups.iter()
                                         .find(|g| g.identifier == param.identifier);
        if param_type.is_none() {
            dependency.push(decl.return_type.clone());
        } else {
            result.commands.extend(align_array_width(&param_type.unwrap().slot.to_be_bytes().to_vec(), metadata.data_slot_alignment));
        }
    }

    return if dependency.is_empty() {
        Ok(result)
    } else {
        Err(dependency)
    };
}
