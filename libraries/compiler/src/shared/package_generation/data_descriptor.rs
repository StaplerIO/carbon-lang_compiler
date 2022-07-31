#[derive(Debug, Clone)]
pub enum DataLocation {
    Local,
    Global
}

#[derive(Debug, Clone)]
pub struct DataDeclarator {
    pub name: String,
    // Slot is start from 0
    pub slot: usize,
    pub location: DataLocation,
    pub is_string: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StringConstant {
    pub value: String,
    pub slot: usize,
}

#[derive(Debug, Clone)]
pub struct DataAccessDescriptor {
    pub identifier: Option<DataDeclarator>,
    pub string_constant: Option<StringConstant>,
    pub instant_value: Option<String>
}
