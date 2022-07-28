use crate::shared::package_generation::package_descriptor::PackageMetadata;

impl PackageMetadata {
    pub fn serialize(&self) -> Vec<u8> {
        return vec![self.variable_slot_alignment,
                    self.data_alignment,
                    self.command_alignment,
                    self.domain_layer_count_alignment,
                    self.entry_point_offset as u8];
    }
}
