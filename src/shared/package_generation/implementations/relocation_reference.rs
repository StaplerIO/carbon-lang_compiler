use crate::package_generator::utils::{align_data_width, combine_command, convert_to_u8_array};
use crate::shared::command_map::{DomainCommand, RootCommand};
use crate::shared::package_generation::relocation_reference::{RelocatableCommandList, RelocationCredential, RelocationReference, RelocationReferenceType, RelocationTarget, RelocationTargetType};

impl RelocatableCommandList {
    pub fn combine(&mut self, model: RelocatableCommandList) {
        let cred = model.descriptors;

        let base_pos = self.commands.len();
        for item in cred.targets {
            match item.relocation_type {
                // Accumulate relative position
                RelocationTargetType::Relative(x) => {
                    self.descriptors.targets.push(RelocationTarget {
                        relocation_type: RelocationTargetType::Relative(x + base_pos as i32),
                        command_array_position: item.command_array_position + self.commands.len(),
                        relocated_address: item.relocated_address,
                    });
                }
                _ => {
                    self.descriptors.targets.push(RelocationTarget {
                        relocation_type: item.relocation_type,
                        command_array_position: item.command_array_position + self.commands.len(),
                        relocated_address: item.relocated_address,
                    });
                }
            }
        }

        for item in cred.references {
            self.descriptors.references.push(RelocationReference {
                ref_type: item.ref_type,
                command_array_position: item.command_array_position + self.commands.len(),
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
        }
    }

    pub fn new() -> RelocatableCommandList {
        return RelocatableCommandList {
            commands: vec![],
            command_entries: vec![],
            descriptors: RelocationCredential::new(),
        };
    }

    pub fn apply_ref_to_target(&mut self, addr_len: u8, base_offset: usize) {
        let domain_create_command = combine_command(
            RootCommand::Domain.to_opcode(),
            DomainCommand::Create.to_opcode(),
        );

        let domain_destroy_command = combine_command(
            RootCommand::Domain.to_opcode(),
            DomainCommand::Destroy.to_opcode(),
        );

        for desc in &self.descriptors.targets {
            let fixed_start = desc.command_array_position;
            let overwrite_range = fixed_start..(fixed_start + addr_len as usize);

            match desc.relocation_type {
                RelocationTargetType::Relative(x) => {
                    // Up to now, `Relative` is just sign because it updates every time when merging into main `RelocatableCommandList`
                    let addr_u8_vec = align_data_width(convert_to_u8_array((x + base_offset as i32).to_string()), addr_len);
                    self.commands.splice(overwrite_range, addr_u8_vec);
                }
                RelocationTargetType::DomainHead => {
                    // Search for domain flags
                    let mut domain_refs = self.descriptors.references.clone();
                    domain_refs.retain(|r| r.command_array_position <= desc.command_array_position &&
                        (self.commands[r.command_array_position] == domain_create_command || (self.commands[r.command_array_position] == domain_destroy_command)));
                    domain_refs.sort_by(|f1, f2| f1.command_array_position.cmp(&f2.command_array_position));

                    // Maybe there are some inner-domains
                    let mut domain_layer: usize = 0;
                    for item in domain_refs {
                        if self.commands[item.command_array_position] == domain_destroy_command {
                            domain_layer += 1;
                        } else if self.commands[item.command_array_position] == domain_create_command {
                            domain_layer -= 1;
                        }

                        // Which means we have got out from all sub-domains
                        if domain_layer == 0 {
                            let addr_u8_vec = align_data_width(convert_to_u8_array((item.command_array_position as i32 + base_offset as i32).to_string()), addr_len);
                            self.commands.splice(overwrite_range, addr_u8_vec);
                            break;
                        }
                    }

                    // panic!("Unexpected error! Couldn't find the only DomainCreate command")
                }
                RelocationTargetType::BreakDomain(x) => {
                    let mut targets = self.descriptors.references.clone();
                    targets.retain(|p| self.commands[p.command_array_position] == domain_destroy_command);
                    let addr_u8_vec = align_data_width(convert_to_u8_array((targets[x].command_array_position as i32 + base_offset as i32).to_string()), addr_len);
                    self.commands.splice(overwrite_range, addr_u8_vec);
                }
                RelocationTargetType::IgnoreDomain(x) => {
                    // Search for domain flags
                    let mut domain_refs = self.descriptors.references.clone();
                    domain_refs.retain(|r| r.command_array_position <= desc.command_array_position &&
                        (self.commands[r.command_array_position] == domain_create_command || (self.commands[r.command_array_position] == domain_destroy_command)));
                    domain_refs.sort_by(|f1, f2| f1.command_array_position.cmp(&f2.command_array_position));

                    // Maybe there are some inner-domains
                    let mut domain_layer: usize = 0;
                    let mut current_ignored: usize = 0;
                    for item in domain_refs {
                        if self.commands[item.command_array_position] == domain_destroy_command {
                            domain_layer += 1;
                        } else if self.commands[item.command_array_position] == domain_create_command {
                            domain_layer -= 1;
                        }

                        if domain_layer == 0 {
                            current_ignored += 1;
                        }

                        // Which means we have got out from all sub-domains
                        if current_ignored == x {
                            let addr_u8_vec = align_data_width(convert_to_u8_array((item.command_array_position as i32 + base_offset as i32).to_string()), addr_len);
                            self.commands.splice(overwrite_range, addr_u8_vec);
                            break;
                        }
                    }

                    // panic!("Unexpected error! Couldn't find the only DomainCreate command")
                }
                RelocationTargetType::EnterFunction(_) => {
                    panic!("Feature not implemented!")
                }
                RelocationTargetType::Undefined => {}
            }
        }
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
