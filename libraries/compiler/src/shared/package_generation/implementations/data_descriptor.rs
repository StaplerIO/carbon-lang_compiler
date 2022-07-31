use crate::shared::package_generation::data_descriptor::{DataAccessDescriptor, DataDeclarator, StringConstant};

impl DataAccessDescriptor {
    pub fn new_identifier(declarator: DataDeclarator) -> DataAccessDescriptor {
        DataAccessDescriptor {
            identifier: Some(declarator),
            string_constant: None,
            instant_value: None
        }
    }

    pub fn new_string_constant(string_descriptor: StringConstant) -> DataAccessDescriptor {
        DataAccessDescriptor {
            identifier: None,
            string_constant: Some(string_descriptor),
            instant_value: None
        }
    }

    pub fn new_instant_value(value: String) -> DataAccessDescriptor {
        DataAccessDescriptor {
            identifier: None,
            string_constant: None,
            instant_value: Some(value)
        }
    }
}
