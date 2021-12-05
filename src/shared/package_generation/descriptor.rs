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
    pub variable_slot_alignment: u8,
    pub data_alignment: u8,
    pub command_alignment: u8,
    pub entry_point_offset: u8,
    pub domain_layer_count_alignment: u8
}
