use crate::shared::ast::blocks::function::Function;
use crate::shared::package_generation::data_descriptor::DataDeclarator;
use crate::shared::package_generation::package_descriptor::PackageMetadata;

pub struct GeneratorContext {
    pub functions: Vec<Function>,
    pub available_data: Vec<DataDeclarator>,
    pub metadata: PackageMetadata,
}
