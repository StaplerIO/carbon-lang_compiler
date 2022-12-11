use crate::package_generator::utils::{align_array_width,
                                      is_domain_create_command,
                                      is_domain_destroy_command,
                                      is_function_end_command,
                                      is_iteration_end_command,
                                      is_iteration_head_command,
                                      jump_command_address_placeholder_len,
                                      pair_container_action};
use crate::shared::package_generation::relocation_reference::{RelocatableCommandList,
                                                              RelocationCredential,
                                                              RelocationReference,
                                                              RelocationReferenceType,
                                                              RelocationTargetElement};
use crate::shared::utils::identifier::Identifier;

impl RelocatableCommandList {
    pub fn combine(&mut self, model: RelocatableCommandList) {
        let cred = model.descriptors;

        let base_pos = self.commands.len();
        for mut reloc_target in cred.targets {
            reloc_target.command_array_position = reloc_target.command_array_position + base_pos;
            self.descriptors.targets.push(reloc_target);
        }

        for item in cred.references {
            self.descriptors.references.push(RelocationReference {
                ref_type: item.ref_type,
                command_array_position: item.command_array_position + base_pos,
            });
        }

        let original_len = self.commands.len();
        for item in model.command_entries {
            self.command_entries.push(item + original_len);
        }

        self.commands.extend(model.commands);
    }

    pub fn append_commands(&mut self, commands: Vec<u8>) {
        self.commands.extend(commands);
    }

    pub fn new_no_relocation(cmd: Vec<u8>) -> RelocatableCommandList {
        RelocatableCommandList {
            commands: cmd,
            command_entries: vec![],
            descriptors: RelocationCredential::new(),
            string_pool: vec![],
            function_table: vec![],
            group_table: vec![],
        }
    }

    pub fn new() -> RelocatableCommandList {
        return RelocatableCommandList {
            commands: vec![],
            command_entries: vec![],
            descriptors: RelocationCredential::new(),
            string_pool: vec![],
            function_table: vec![],
            group_table: vec![],
        };
    }

    pub fn calculate_ref_to_target(&mut self) {
        self.descriptors.references.sort_by(|a, b| a.command_array_position.cmp(&b.command_array_position));
        self.descriptors.targets.sort_by(|a, b| a.command_array_position.cmp(&b.command_array_position));

        for (_, iter_reloc_target) in self.descriptors.targets.iter_mut().enumerate() {
            for element in &iter_reloc_target.relocation_elements {
                match element {
                    RelocationTargetElement::Relative(x) => {
                        // Up to now, `Relative` is just sign because it updates every time when merging into main `RelocatableCommandList`
                        iter_reloc_target.relocated_address += *x;
                    }
                    RelocationTargetElement::DomainHead => {
                        // Search for domain flags
                        let mut domain_refs = self.descriptors.references.clone();
                        domain_refs.retain(|r| r.command_array_position >= iter_reloc_target.command_array_position
                            && (is_domain_create_command(r) || is_domain_destroy_command(r)));

                        // Maybe there are some inner-domains
                        let mut domain_layer: usize = 0;
                        for domain_ref in &domain_refs {
                            if is_domain_destroy_command(domain_ref) {
                                domain_layer += 1;
                            } else if is_domain_create_command(domain_ref) {
                                domain_layer -= 1;
                            }

                            // Which means we have got out from all sub-domains
                            if domain_layer == 0 {
                                iter_reloc_target.relocated_address += domain_ref.command_array_position as i32 - iter_reloc_target.command_array_position as i32;
                                break;
                            }
                        }

                        // panic!("Unexpected error! Couldn't find the only DomainCreate command")
                    }
                    RelocationTargetElement::BreakDomain(_x) => {
                        let mut refs = self.descriptors.references.clone();
                        refs.retain(|p| is_domain_destroy_command(p) && p.command_array_position > iter_reloc_target.command_array_position);

                        iter_reloc_target.relocated_address = refs[0].command_array_position as i32 - iter_reloc_target.command_array_position as i32;
                    }
                    RelocationTargetElement::IgnoreDomain(x) => {
                        // Search for domain flags
                        let mut domain_refs = self.descriptors.references.clone();
                        domain_refs.retain(|r| r.command_array_position > iter_reloc_target.command_array_position
                            && (is_domain_create_command(r) || is_domain_destroy_command(r)));

                        let mut current_index = 0;
                        for _ in 0..*x {
                            let pair_result = pair_container_action(&domain_refs[current_index..]);
                            // Locate the next container entrance
                            current_index = pair_result.1 + 1;
                        }

                        // Restore to end container
                        current_index -= 1;

                        let target_ref = &domain_refs[current_index];
                        iter_reloc_target.relocated_address += target_ref.command_array_position as i32 - iter_reloc_target.command_array_position as i32;

                        // panic!("Unexpected error! Couldn't find the only DomainCreate command")
                    }
                    RelocationTargetElement::EnterFunction(_) => {}
                    RelocationTargetElement::BreakIteration => {
                        // Find the first valid reference
                        let end_ref = self.descriptors.references.iter()
                                          .find(|r| r.command_array_position > iter_reloc_target.command_array_position
                                              && is_iteration_end_command(r))
                                          .unwrap()
                                          .clone();

                        let default = RelocationReference { ref_type: RelocationReferenceType::FunctionEntrance(Identifier::empty()), command_array_position: usize::MAX };
                        let nearest_function_end = self.descriptors
                                                       .references
                                                       .iter()
                                                       .find(|r| is_function_end_command(r) && r.command_array_position > iter_reloc_target.command_array_position)
                                                       .unwrap_or(&default)
                                                       .clone();


                        if end_ref.command_array_position <= nearest_function_end.command_array_position {
                            iter_reloc_target.relocated_address += end_ref.command_array_position as i32 - iter_reloc_target.command_array_position as i32;
                        } else {
                            panic!("Invalid instruction");
                        }
                    }
                    RelocationTargetElement::IterationHead => {
                        // Find the first valid reference
                        let head_ref = self.descriptors.references.iter().find(|r| r.command_array_position < iter_reloc_target.command_array_position
                            && is_iteration_head_command(r))
                                           .unwrap()
                                           .clone();

                        let default = RelocationReference { ref_type: RelocationReferenceType::FunctionEntrance(Identifier::empty()), command_array_position: usize::MIN };
                        let nearest_function_begin = self.descriptors
                                                         .references
                                                         .iter()
                                                         .find(|r| is_function_end_command(r))
                                                         .unwrap_or(&default)
                                                         .clone();

                        if head_ref.command_array_position >= nearest_function_begin.command_array_position {
                            iter_reloc_target.relocated_address += head_ref.command_array_position as i32 - iter_reloc_target.command_array_position as i32;
                        } else {
                            panic!("Invalid instruction");
                        }
                    }
                    RelocationTargetElement::Undefined => {
                        panic!("Encountered undefined relocation target!");
                    }
                }
            }
        }
    }

    pub fn apply_relocation(&mut self, addr_len: u8) {
        let placeholder_len = jump_command_address_placeholder_len(addr_len);
        for desc in &self.descriptors.targets {
            let mut addr_bytes;

            match &desc.relocation_elements[0] {
                RelocationTargetElement::EnterFunction(id) => {
                    let target_function = self.function_table
                                              .iter()
                                              .find(|f| f.name == *id)
                                              .unwrap();

                    addr_bytes = align_array_width(&target_function.slot.to_be_bytes().to_vec(), addr_len);
                    addr_bytes.insert(0, 0x00);
                }
                _ => {
                    let addr = desc.relocated_address;
                    addr_bytes = align_array_width(&addr.abs().to_be_bytes().to_vec(), addr_len);

                    if addr < 0 {
                        addr_bytes.insert(0, 0x0B);
                    } else {
                        addr_bytes.insert(0, 0x0F);
                    }
                }
            }

            let begin_pos = desc.command_array_position + desc.offset as usize;
            self.commands.splice(begin_pos..(begin_pos + placeholder_len), addr_bytes);
        }
    }

    pub fn generate_string_pool(&mut self, addr_len: u8) -> Vec<u8> {
        let mut result = vec![];

        // Push pool size
        result.extend(align_array_width(&self.string_pool.len().to_be_bytes().to_vec(), addr_len));

        for string in self.string_pool.iter() {
            let len = align_array_width(&string.value.len().to_be_bytes().to_vec(), addr_len);
            result.extend(len);
            result.extend(string.value.as_bytes().to_vec());
        }

        return result;
    }

    pub fn generate_function_table(&self, addr_len: u8) -> Vec<u8> {
        let mut result = vec![];

        // Push table size
        result.extend(align_array_width(&self.function_table.len().to_be_bytes().to_vec(), addr_len));

        for func in self.function_table.iter() {
            result.extend(align_array_width(&func.slot.to_be_bytes().to_vec(), addr_len));
            result.extend(align_array_width(&func.relocated_entry_address.to_be_bytes().to_vec(), addr_len));
        }

        return result;
    }
}

impl RelocationCredential {
    pub fn new() -> RelocationCredential {
        return RelocationCredential {
            targets: vec![],
            references: vec![],
        };
    }
}

impl RelocationReference {
    pub fn new(ref_type: RelocationReferenceType) -> RelocationReference {
        return RelocationReference { ref_type, command_array_position: 0 };
    }
}
