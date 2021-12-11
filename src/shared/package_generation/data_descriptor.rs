#[derive(Clone)]
pub struct DataDeclaration {
    pub name: String,
    // Slot is start from 0
    pub slot: Vec<u8>,
    pub is_global: bool,
}

#[derive(Clone)]
pub struct DataAccessDescriptor {
    pub is_global: bool,
    pub domain_layer: Vec<u8>,
    pub slot: Vec<u8>,
}
