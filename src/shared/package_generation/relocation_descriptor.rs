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
pub enum RelocationType {
    Absolute,
    DomainHead,
    BreakDomain(usize),
    NextCommand,
    IgnoreDomain(usize),
    EnterFunction(String)
}

pub struct RelocationDescriptor {
    pub relocation_type: RelocationType,
    pub offset: isize,
    pub command_array_position: usize,
    pub relocated_address: Vec<u8>,
}

pub struct JumpCommandBuildResult {
    pub commands: Vec<u8>,
    pub descriptors: Vec<RelocationDescriptor>
}
