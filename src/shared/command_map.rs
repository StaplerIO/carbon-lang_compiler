#[derive(Clone, Hash, Eq, PartialEq)]
pub enum RootCommand {
    Object,
    Stack,
    Domain,
    Jump,
    Function,
    Math,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum ObjectCommand {
    Create,
    Destroy
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum StackCommand {
    Push,
    PushFromObject,
    Pop,
    PopToObject,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum DomainCommand {
    Create,
    Destory,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum JumpCommand {
    ToOffset,
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
    Logical
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum MathCalcCommand {
    Plus,
    Minus,
    Times,
    Divide,
    Mod,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum MathRelationCommand {
    And,
    Or,
    Not,
}