use crate::shared::package_generation::package_descriptor::PackageMetadata;

impl PackageMetadata {
    pub fn serialize(&self) -> Vec<u8> {
        return vec![self.package_type,
                    self.data_alignment,
                    self.domain_layer_count_alignment,
                    self.data_slot_alignment,
                    self.address_alignment];
    }
}
