use crate::shared::package_generation::data_descriptor::StringConstant;
use crate::shared::package_generation::func_table::FunctionTable;
use crate::shared::package_generation::group_context::GroupTable;
use crate::shared::utils::identifier::Identifier;

pub type StringPool = Vec<StringConstant>;

#[derive(Clone, Debug)]
pub struct RelocationCredential {
    pub targets: Vec<RelocationTarget>,
    pub references: Vec<RelocationReference>,
}

/// ## RelocationType with value
///
/// ### `BreakDomain`
/// The number in this value is the layer count
///
/// ### `IgnoreDomain`
/// This is used to build `IfBlock`, to ignore next `elif` and `else`
///
/// ### `EnterFunction`
/// Save the identifier of target function in it
#[derive(Clone, Debug)]
pub enum RelocationTargetElement {
    Relative(i32),
    IterationHead,
    BreakIteration,
    DomainHead,
    // Set to 0 to jump out to nearest loop
    BreakDomain(usize),
    IgnoreDomain(usize),
    EnterFunction(Identifier),
    Undefined,
}

/// Properties in `RelocationTarget` you should aware
///
/// # Fields
///
/// **`command_array_position`**
///
/// The position of the head of the command
///
/// **`offset`**
///
/// The offset of the reserved address fields to the head of the command
///
#[derive(Clone, Debug)]
pub struct RelocationTarget {
    pub relocation_elements: Vec<RelocationTargetElement>,
    pub command_array_position: usize,
    pub offset: i32,

    pub relocated_address: i32,
}

#[derive(Clone, Debug)]
pub struct RelocatableCommandList {
    pub commands: Vec<u8>,
    pub command_entries: Vec<usize>,
    pub descriptors: RelocationCredential,
    pub string_pool: StringPool,
    pub function_table: FunctionTable,
    pub group_table: GroupTable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RelocationReferenceType {
    // The string includes function name
    FunctionEntrance(Identifier),
    EndFunction(Identifier),
    IfEntrance,
    ElifEntrance,
    ElseEntrance,
    EndIf,
    EndElif,
    EndElse,
    WhileEntrance,
    EndWhile,
    LoopEntrance,
    EndLoop,
    DomainEntrance,
    EndDomain,
}

#[derive(Clone, Debug)]
pub struct RelocationReference {
    pub ref_type: RelocationReferenceType,

    // Set this value when applying reference to target
    pub command_array_position: usize,
}
