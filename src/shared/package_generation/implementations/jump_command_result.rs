use crate::shared::package_generation::relocation_reference::{JumpCommandBuildResult, RelocationTarget};

impl JumpCommandBuildResult {
    pub fn append(&mut self, model: JumpCommandBuildResult) {
        let mut aligned_model = vec![];
        for item in model.descriptors {
            aligned_model.push(RelocationTarget {
                relocation_type: item.relocation_type,
                offset: item.offset,
                command_array_position: item.command_array_position + self.commands.len(),
                relocated_address: item.relocated_address
            });
        }

        self.commands.extend(model.commands);
        self.descriptors.extend(aligned_model);
    }

    pub fn append_commands(&mut self, commands: Vec<u8>) {
        self.commands.extend(commands);
    }

    pub fn new_no_relocation(cmd: Vec<u8>) -> JumpCommandBuildResult {
        JumpCommandBuildResult {
            commands: cmd,
            descriptors: vec![]
        }
    }
}
