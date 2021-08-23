pub struct PackageDescriptor {
    pub name: String,

    // If package is an executable, there will be an offset pointed to the start of the entry function
    // Or it'll be `None`
    pub entry_offset: Option<usize>,

    // Description
    pub compiler_version: String,
    pub author: String,
}
