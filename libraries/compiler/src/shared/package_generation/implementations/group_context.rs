use crate::package_generator::command_builder::function_block::build_function_command;
use crate::package_generator::utils::align_array_width;
use crate::shared::ast::group::implementation::{FunctionImplementation, MethodImplementation};
use crate::shared::package_generation::group_context::{GeneratedGroup, GroupTableEntry};
use crate::shared::package_generation::package_descriptor::PackageMetadata;
use crate::shared::package_generation::relocation_reference::RelocatableCommandList;
use crate::shared::utils::identifier::Identifier;

impl GroupTableEntry {
    pub fn write_commands(&mut self, function_begin_loc: usize, generated_groups: &Vec<GeneratedGroup>, metadata: &PackageMetadata) -> (RelocatableCommandList, RelocatableCommandList) {
        let mut decl = RelocatableCommandList::new();
        let mut func_cmd = RelocatableCommandList::new();

        // Generate fields
        for field in &self.fields {
            let source = generated_groups.iter()
                                         .find(|g| g.identifier == field.type_id)
                                         .unwrap();

            decl.commands.extend(align_array_width(&source.slot.to_be_bytes().to_vec(), metadata.data_slot_alignment));
        }

        let mut current_loc = function_begin_loc;

        // Generate methods
        decl.commands.extend(align_array_width(&self.relocated_method.len().to_be_bytes().to_vec(), metadata.data_slot_alignment));
        for method in &mut self.relocated_method {
            let return_type = generated_groups.iter()
                                              .find(|g| g.identifier == method.return_type_id)
                                              .unwrap();

            decl.commands.extend(align_array_width(&return_type.slot.to_be_bytes().to_vec(), metadata.data_slot_alignment));

            for impl_block in &mut method.implementations_entry {
                impl_block.entry_address = current_loc;
                func_cmd.combine(impl_block.commands.clone());

                decl.commands.extend(align_array_width(&impl_block.entry_address.to_be_bytes().to_vec(), metadata.address_alignment));
                current_loc += impl_block.commands.commands.len();
            }
        }

        // Generate functions
        decl.commands.extend(align_array_width(&self.relocated_method.len().to_be_bytes().to_vec(), metadata.data_slot_alignment));
        for func in &mut self.relocated_function {
            let return_type = generated_groups.iter()
                                              .find(|g| g.identifier == func.return_type_id)
                                              .unwrap();

            decl.commands.extend(align_array_width(&return_type.slot.to_be_bytes().to_vec(), metadata.data_slot_alignment));

            for impl_block in &mut func.implementations_entry {
                impl_block.entry_address = current_loc;
                func_cmd.combine(impl_block.commands.clone());

                decl.commands.extend(align_array_width(&impl_block.entry_address.to_be_bytes().to_vec(), metadata.address_alignment));
                current_loc += impl_block.commands.commands.len();
            }
        }

        return (decl, func_cmd);
    }

    pub fn get_allocated_code_space(&self, metadata: &PackageMetadata) -> usize {
        let mut result = 0;

        // Generate fields
        result += self.fields.len() * metadata.data_slot_alignment as usize;

        // Generate methods

        // Method count
        result += metadata.data_slot_alignment as usize;

        // Every method
        for method in &self.relocated_method {
            // Return value
            result += metadata.data_slot_alignment as usize;

            // Entry address of each implementation
            result += method.implementations_entry.len() as usize;
        }

        // Generate functions
        result += metadata.data_slot_alignment as usize;
        for func in &self.relocated_function {
            // Return value
            result += metadata.data_slot_alignment as usize;

            // Entry address of each implementation
            result += func.implementations_entry.len() as usize;
        }

        return result;
    }

    pub fn check_dependency(&self, generated_groups: &Vec<GeneratedGroup>) -> Vec<Identifier> {
        let mut dependent_groups = vec![];

        // Check fields
        for field in &self.fields {
            let field_type = generated_groups.iter().find(|g| g.identifier == field.type_id);
            if field_type.is_none() {
                dependent_groups.push(field.type_id.clone());
            }
        }

        // Check methods
        for method in &self.relocated_method {
            let implementations = self.get_method_implementation_by_identifier(&method.identifier);

            for method_impl in implementations {
                for param in method_impl.declarator.parameters {
                    let param_type = generated_groups.iter().find(|g| g.identifier == param.identifier);
                    if param_type.is_none() {
                        dependent_groups.push(param_type.unwrap().identifier.clone());
                    }
                }
            }
        }

        // Check functions
        for func in &self.relocated_function {
            let implementations = self.get_function_implementation_by_identifier(&func.identifier);

            for func_impl in implementations {
                for param in func_impl.declarator.parameters {
                    let param_type = generated_groups.iter().find(|g| g.identifier == param.identifier);
                    if param_type.is_none() {
                        dependent_groups.push(param_type.unwrap().identifier.clone());
                    }
                }
            }
        }

        return dependent_groups;
    }

    pub fn generate_functions(&mut self, metadata: &PackageMetadata) {
        for func in &mut self.relocated_function {
            for (idx, implementation) in func.implementations_entry.iter_mut().enumerate() {
                let func_ast = self.implementation_table[idx]
                    .functions
                    .iter()
                    .find(|f| f.declarator.identifier == func.identifier)
                    .unwrap();

                implementation.commands = build_function_command(func_ast, metadata);
            }
        }
    }

    pub fn generate_methods(&mut self, metadata: &PackageMetadata) {
        for func in &mut self.relocated_method {
            for (idx, implementation) in func.implementations_entry.iter_mut().enumerate() {
                let func_ast = self.implementation_table[idx]
                    .methods
                    .iter()
                    .find(|f| f.declarator.identifier == func.identifier)
                    .unwrap();

                implementation.commands = build_function_command(func_ast, metadata);
            }
        }
    }

    fn get_method_implementation_by_identifier(&self, id: &Identifier) -> Vec<MethodImplementation> {
        let mut result = vec![];

        for implementation_block in &self.implementation_table {
            result.push(implementation_block.methods.iter().find(|m| m.declarator.identifier == *id).unwrap().clone());
        }

        return result;
    }

    fn get_function_implementation_by_identifier(&self, id: &Identifier) -> Vec<FunctionImplementation> {
        let mut result = vec![];

        for implementation_block in &self.implementation_table {
            result.push(implementation_block.functions.iter().find(|f| f.declarator.identifier == *id).unwrap().clone());
        }

        return result;
    }
}
