use crate::shared::package_generation::relocation_descriptor::JumpCommandBuildResult;

impl JumpCommandBuildResult {
    pub fn new_no_relocation(cmd: Vec<u8>) -> JumpCommandBuildResult {
        JumpCommandBuildResult {
            commands: cmd,
            descriptors: vec![]
        }
    }
}
