use crate::shared::package_generation::relocation_reference::{RelocatableCommandList, RelocationCredential, RelocationReference, RelocationTarget};

impl RelocatableCommandList {
    pub fn combine(&mut self, model: RelocatableCommandList) {
        let cred = model.descriptors;

        for item in cred.targets {
            self.descriptors.targets.push(RelocationTarget {
                relocation_type: item.relocation_type,
                command_array_position: item.command_array_position + self.commands.len(),
                relocated_address: item.relocated_address,
            });
        }

        for item in cred.references {
            self.descriptors.references.push(RelocationReference {
                ref_type: item.ref_type,
                command_array_position: item.command_array_position + self.commands.len(),
            })
        }

        self.commands.extend(model.commands);
    }

    pub fn append_commands(&mut self, commands: Vec<u8>) {
        self.commands.extend(commands);
    }

    pub fn new_no_relocation(cmd: Vec<u8>) -> RelocatableCommandList {
        RelocatableCommandList {
            commands: cmd,
            descriptors: RelocationCredential::new(),
        }
    }

    pub fn new() -> RelocatableCommandList {
        return RelocatableCommandList {
            commands: vec![],
            descriptors: RelocationCredential::new(),
        };
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
