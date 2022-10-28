pub const PLACE_HOLDER: u8 = 0x0;

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum RootCommand {
    Object,
    Stack,
    Domain,
    Jump,
    Function,
    Math,
}

/**
 * ## About DataLocator
 *
 * Example: `0xA10000000002`
 *
 * Format: `A1 00 00 00 00 02`
 *
 * Abstraction:  `<Command> <GDF> <Slot>`
 *
 * ### About GlobalDataFlag(GDF)
 *
 * If data is a global data, then `GDF` is set to `0x01`.
 * If data is a private data, `GDF` is set to `0x00`
 */
#[derive(Clone, Hash, Eq, PartialEq)]
pub enum ObjectCommand {
    Create,
    Destroy,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum StackCommand {
    Push,
    PushFromObject,
    Pop,
    PopToObject,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum JumpCommand {
    ToRelative,
    ByStackTop,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum FunctionCommand {
    Enter,
    LeaveWithoutValue,
    LeaveWithValue,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum MathCommand {
    Calculation,
    Logical,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum MathCalcCommand {
    Plus,
    Minus,
    Times,
    Divide,
    Mod,
    Inverse
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum MathLogicalCommand {
    And,
    Or,
    Not,
}
