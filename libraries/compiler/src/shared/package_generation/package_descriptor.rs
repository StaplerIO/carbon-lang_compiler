pub struct PackageDescriptor {
    pub name: String,

    // If package is an executable, there will be an offset pointed to the start of the entry function
    // Or it'll be `None`
    pub entry_offset: Option<usize>,

    // Description
    pub compiler_version: String,
    pub author: String,

    // Metadata
    pub metadata: PackageMetadata,
}

pub struct PackageMetadata {
    pub package_type: u8,
    pub data_alignment: u8,
    pub domain_layer_count_alignment: u8,
    pub variable_slot_alignment: u8,
    pub address_alignment: u8,
    pub global_command_offset: u8,
}
