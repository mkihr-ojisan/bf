#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Increment,
    Decrement,
    PointerIncrement,
    PointerDecrement,
    PutChar,
    GetChar,
    Loop(Vec<Instruction>),

    Add(u8),
    Subtract(u8),
    SetZero,
    PointerAdd(usize),
    PointerSubtract(usize),
    AddValueAt(isize),
    SubtractValueAt(isize),
    AddValueMultipliedBy(u8, isize),
    SubtractValueMultipliedBy(u8, isize),
    Negate,
    IfNotZero(Vec<Instruction>),
}
