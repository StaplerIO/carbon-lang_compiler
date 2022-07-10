pub struct RelocationCredential {
    pub targets: Vec<RelocationTarget>,
    pub references: Vec<RelocationReference>
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
pub enum RelocationTargetType {
    Absolute,
    // Set to 0 to jump out to nearest loop
    DomainHead(usize),
    // Set to 0 to jump out to nearest loop
    BreakDomain(usize),
    NextCommand,
    IgnoreDomain(usize),
    EnterFunction(String),
    Undefined
}

pub struct RelocationTarget {
    pub relocation_type: RelocationTargetType,
    pub command_array_position: usize,
    pub relocated_address: Vec<u8>,
}

pub struct RelocatableCommandList {
    pub commands: Vec<u8>,
    pub descriptors: RelocationCredential
}

pub enum RelocationReferenceType {
    FunctionEntrance,
    EndFunction,
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
}

pub struct RelocationReference {
    pub ref_type: RelocationReferenceType,
    pub command_array_position: usize,
}
