#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub enum RelocationTargetType {
    Relative(i32),
    DomainHead,
    // Set to 0 to jump out to nearest loop
    BreakDomain(usize),
    IgnoreDomain(usize),
    EnterFunction(String),
    Undefined
}

#[derive(Clone, Debug)]
pub struct RelocationTarget {
    pub relocation_type: RelocationTargetType,
    pub command_array_position: usize,
    pub relocated_address: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct RelocatableCommandList {
    pub commands: Vec<u8>,
    pub command_entries: Vec<usize>,
    pub descriptors: RelocationCredential
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct RelocationReference {
    pub ref_type: RelocationReferenceType,

    // Set this value when applying reference to target
    pub command_array_position: usize,
}
