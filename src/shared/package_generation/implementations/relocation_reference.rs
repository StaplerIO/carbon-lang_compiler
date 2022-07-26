use crate::package_generator::utils::{align_data_width, combine_command, is_domain_create_command, is_domain_destroy_command};
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
                        command_array_position: item.command_array_position + base_pos,
                        offset: item.offset,
                        relocated_address: item.relocated_address,
                    });
                }
                _ => {
                    self.descriptors.targets.push(RelocationTarget {
                        relocation_type: item.relocation_type,
                        command_array_position: item.command_array_position + base_pos,
                        offset: item.offset,
                        relocated_address: item.relocated_address,
                    });
                }
            }
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
        }
    }

    pub fn new() -> RelocatableCommandList {
        return RelocatableCommandList {
            commands: vec![],
            command_entries: vec![],
            descriptors: RelocationCredential::new(),
        };
    }

    pub fn calculate_ref_to_target(&mut self, base_offset: usize) {
        let domain_create_command = combine_command(
            RootCommand::Domain.to_opcode(),
            DomainCommand::Create.to_opcode(),
        );

        let domain_destroy_command = combine_command(
            RootCommand::Domain.to_opcode(),
            DomainCommand::Destroy.to_opcode(),
        );

        let mut targets = self.descriptors.targets.clone();

        for (index, desc) in self.descriptors.targets.iter().enumerate() {
            match desc.relocation_type {
                RelocationTargetType::Relative(x) => {
                    // Up to now, `Relative` is just sign because it updates every time when merging into main `RelocatableCommandList`
                    targets[index].relocated_address = (x + base_offset as i32 + desc.offset) as usize;
                }
                RelocationTargetType::DomainHead => {
                    // Search for domain flags
                    let mut domain_refs = self.descriptors.references.clone();
                    domain_refs.retain(|r| r.command_array_position >= desc.command_array_position && (is_domain_create_command(r) || is_domain_destroy_command(r)));

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
                            targets[index].relocated_address = (item.command_array_position as i32 + base_offset as i32 + desc.offset) as usize;
                            break;
                        }
                    }

                    // panic!("Unexpected error! Couldn't find the only DomainCreate command")
                }
                RelocationTargetType::BreakDomain(x) => {
                    // TODO：Bug here
                    let mut refs = self.descriptors.references.clone();
                    refs.retain(|p| is_domain_destroy_command(p));

                    targets[index].relocated_address = (refs[0].command_array_position as i32 + base_offset as i32 + desc.offset) as usize;
                }
                RelocationTargetType::IgnoreDomain(x) => {
                    // Search for domain flags
                    let mut domain_refs = self.descriptors.references.clone();
                    domain_refs.retain(|r| r.command_array_position >= desc.command_array_position && (is_domain_create_command(r) || is_domain_destroy_command(r)));

                    // Maybe there are some inner-domains
                    let mut domain_layer: usize = 0;
                    let mut current_ignored: usize = 0;
                    for item in &domain_refs {
                        if is_domain_destroy_command(item) {
                            domain_layer -= 1;
                        } else if is_domain_create_command(item) {
                            domain_layer += 1;
                        }

                        if domain_layer == 0 {
                            current_ignored += 1;
                        }

                        // Which means we have got out from all sub-domains
                        if current_ignored == x {
                            targets[index].relocated_address = (item.command_array_position as i32 + base_offset as i32 + desc.offset) as usize;
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

        // Update to relocated targets
        self.descriptors.targets = targets;
    }

    pub fn apply_relocation(&mut self, addr_len: u8) {
        for desc in &self.descriptors.targets {
            let mut addr_bytes = desc.relocated_address.to_be_bytes().to_vec();
            while addr_bytes[0] == 0x00 {
                addr_bytes.remove(0);
            }
            let addr_u8_vec = align_data_width(addr_bytes, addr_len);
            self.commands.splice(desc.command_array_position..(desc.command_array_position + addr_len as usize), addr_u8_vec);
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
